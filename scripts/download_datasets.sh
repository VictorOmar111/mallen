#!/usr/bin/env bash
set -euo pipefail

mkdir -p data
echo "🌐 Descargando y normalizando padrones públicos en data/..."

# 1. Descarga de Listado Completo 69-B del SAT (Conversión de ISO-8859-1 a UTF-8 limpia)
SAT_69B_URL="http://omawww.sat.gob.mx/cifras_sat/Documents/Listado_Completo_69-B.csv"
echo "⬇️  Descargando Lista 69-B (EFOS) del SAT..."
if curl -sSL --retry 3 --connect-timeout 15 "$SAT_69B_URL" -o data/sat_efos_raw.csv; then
    # Eliminar las primeras 2 líneas de encabezado decorativo del SAT y convertir a UTF-8
    tail -n +3 data/sat_efos_raw.csv | iconv -f ISO-8859-1 -t UTF-8//IGNORE > data/sat_efos.csv
    rm -f data/sat_efos_raw.csv
    echo "✅ data/sat_efos.csv listo ($(wc -l < data/sat_efos.csv) registros)."
else
    echo "⚠️  El portal del SAT no respondió a tiempo; se usará la base local existente."
fi

# 2. Descarga de Listado Art. 69 del SAT (No localizados / Créditos Firmes)
SAT_69_URL="http://omawww.sat.gob.mx/cifras_sat/Documents/No%20localizados.csv"
echo "⬇️  Descargando Lista Art. 69 (No Localizados) del SAT..."
if curl -sSL --retry 3 --connect-timeout 15 "$SAT_69_URL" -o data/sat_69_raw.csv; then
    iconv -f ISO-8859-1 -t UTF-8//IGNORE data/sat_69_raw.csv > data/sat_69_incumplidos.csv
    rm -f data/sat_69_raw.csv
    echo "✅ data/sat_69_incumplidos.csv listo ($(wc -l < data/sat_69_incumplidos.csv) registros)."
else
    echo "⚠️  No se pudo descargar Art. 69 en este momento; se mantendrá semilla local."
fi

echo "🎉 Proceso de descarga terminado. Ejecuta ahora: cargo run --release --bin ingest"
