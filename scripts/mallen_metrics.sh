#!/usr/bin/env bash
set -euo pipefail
cd ~/worker_inegi_engine
DB="${MALLEN_DB_PATH:-data/mallen_prod.db}"

echo "=========================================================================="
echo "  TABLERO OPERATIVO MALLEN 360° — $(date -u '+%Y-%m-%d %H:%M:%S UTC')"
echo "=========================================================================="
echo "  Servicio : $(systemctl is-active mallen.service 2>/dev/null || echo 'desconocido')"
systemctl status mallen.service --no-pager 2>/dev/null | grep -E "Memory:|CPU:" | sed 's/^/  /' || true
ls -lh "${DB}"* 2>/dev/null | awk '{print "  DB/WAL   : " $5 " " $9}'

echo -e "\n--- EMBUDO DE PROSPECCIÓN (prospect_logs / prospect_pipeline) ---"
sqlite3 -header -column "${DB}" "
SELECT status, COUNT(*) AS total
FROM prospect_logs
GROUP BY status
ORDER BY total DESC;
" 2>/dev/null || echo "  (Sin registros aún en prospect_logs)"

echo -e "\n--- EMBUDO DE CONVERSIÓN WEB Y VENTAS (audit_sessions) ---"
sqlite3 -header -column "${DB}" "
SELECT
  COUNT(*) AS total_escaneos,
  SUM(CASE WHEN is_accredited = 1 THEN 1 ELSE 0 END) AS pagados_zip_liberado,
  SUM(CASE WHEN is_accredited = 0 THEN 1 ELSE 0 END) AS pendientes_pago
FROM audit_sessions;
" 2>/dev/null || echo "  (Sin registros aún en audit_sessions)"

echo -e "\n--- ÚLTIMOS 15 EVENTOS DE FUNNEL EN journalctl ---"
sudo journalctl -u mallen.service -n 15 --no-pager 2>/dev/null | grep -E "FUNNEL|ERROR|WARN|panic" || echo "  (Sin eventos recientes)"
echo "=========================================================================="
