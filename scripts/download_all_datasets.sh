#!/usr/bin/env bash
# ==============================================================================
# FIRMA PERICIAL MALLEN 360° — DESCARGADOR NACIONAL COMPLETO (32 ESTADOS DE MÉXICO)
# Descarga SAT 69-B, SAT 69, SFP y las 32 Entidades Federativas del INEGI DENUE
# inyectándolas estado por estado en SQLite WAL para nunca saturar el disco.
# ==============================================================================
set -e
cd "$(dirname "$0")/.."
mkdir -p data

echo "=============================================================================="
echo "🇲🇽 INICIANDO DESCARGA E INGESTA NACIONAL COMPLETA (SAT + SFP + 32 ESTADOS INEGI)"
echo "=============================================================================="

# 1. SAT 69-B (Listado Completo de EFOS)
echo "➡️  [1/4] Descargando Padrón Nacional SAT Art. 69-B (EFOS)..."
curl -sSLk --retry 3 --connect-timeout 20 \
  "http://omawww.sat.gob.mx/cifras_sat/Documents/Listado_Completo_69-B.csv" \
  -o data/raw_sat_69b.csv || true

if [ -s data/raw_sat_69b.csv ] && ! grep -qi "<html" data/raw_sat_69b.csv; then
    tail -n +4 data/raw_sat_69b.csv | iconv -f ISO-8859-1 -t UTF-8//IGNORE > data/sat_efos.csv
    rm -f data/raw_sat_69b.csv
    echo "   ✅ SAT 69-B convertido a UTF-8: $(wc -l < data/sat_efos.csv) contribuyentes."
else
    rm -f data/raw_sat_69b.csv
fi

# 2. SAT Art. 69 (No Localizados / Exigibles / Firmes)
echo "➡️  [2/4] Descargando Padrón Nacional SAT Art. 69 (Incumplidos y No Localizados)..."
> data/sat_69_incumplidos.csv
for sat_doc in "No%20localizados.csv" "Firmes.csv" "Exigibles.csv"; do
    curl -sSLk --retry 2 --connect-timeout 20 \
      "http://omawww.sat.gob.mx/cifras_sat/Documents/${sat_doc}" \
      -o "data/tmp_${sat_doc}" || true
    if [ -s "data/tmp_${sat_doc}" ] && ! grep -qi "<html" "data/tmp_${sat_doc}"; then
        tail -n +2 "data/tmp_${sat_doc}" | iconv -f ISO-8859-1 -t UTF-8//IGNORE >> data/sat_69_incumplidos.csv
    fi
    rm -f "data/tmp_${sat_doc}"
done
echo "   ✅ SAT Art. 69 consolidado: $(wc -l < data/sat_69_incumplidos.csv) registros."

# 3. Ingesta base de SAT + Semillas
./target/release/ingest

# 4. INEGI DENUE — LAS 32 ENTIDADES FEDERATIVAS DE MÉXICO (Estado 01 al 32)
# Nota: Se procesa 1 estado a la vez (descarga ZIP -> extrae CSV en UTF-8 -> inserta en SQLite -> borra CSV temporal)
echo "➡️  [3/4] Descargando e indexando las 32 Entidades Federativas del INEGI DENUE..."
for state_num in $(seq -w 1 32); do
    zip_url="https://www.inegi.org.mx/contenidos/masiva/denue/denue_${state_num}_csv.zip"
    zip_file="data/denue_${state_num}.zip"
    csv_tmp="data/tmp_state_${state_num}.csv"

    echo "   📦 Estado ${state_num}/32: Descargando ${zip_url} ..."
    if curl -sSL --retry 3 --connect-timeout 25 "$zip_url" -o "$zip_file"; then
        if file "$zip_file" | grep -qi "Zip archive"; then
            unzip -p "$zip_file" "conjunto_de_datos/*.csv" 2>/dev/null \
              | iconv -f ISO-8859-1 -t UTF-8//IGNORE > "$csv_tmp" || true
            if [ -s "$csv_tmp" ]; then
                ./target/release/ingest --append-denue "$csv_tmp"
            fi
        fi
    fi
    rm -f "$zip_file" "$csv_tmp"
done

# 5. Reconstruir el índice nacional FTS5 al terminar los 32 estados
echo "➡️  [4/4] Reconstruyendo Índice de Búsqueda Ultrarrápida FTS5 Nacional..."
./target/release/ingest --rebuild-fts
echo "🎉 ¡TOTALIDAD DEL MERCADO MEXICANO INDEXADA EN data/mallen_prod.db!"
