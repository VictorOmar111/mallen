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

echo -e "\n--- EMBUDO DE PROSPECCIÓN: PRODUCCIÓN REAL (excluyendo 127.0.0.1/localhost) ---"
sqlite3 -header -column "${DB}" "
SELECT resultado_codigo, contact_method_tier AS tier, COUNT(*) AS total
FROM prospect_logs
WHERE final_domain NOT IN ('127.0.0.1', 'localhost', '')
  AND target_url NOT LIKE '%127.0.0.1%'
  AND target_url NOT LIKE '%localhost%'
GROUP BY resultado_codigo, contact_method_tier
ORDER BY total DESC
LIMIT 15;
" 2>/dev/null || echo "  (Sin registros de producción aún en prospect_logs)"

echo -e "\n--- RESUMEN TOTAL prospect_logs (Prod vs Pruebas Locales) ---"
sqlite3 -header -column "${DB}" "
SELECT
  COUNT(*) AS total_logs,
  SUM(CASE WHEN final_domain NOT IN ('127.0.0.1','localhost','') AND target_url NOT LIKE '%127.0.0.1%' THEN 1 ELSE 0 END) AS prod_externos,
  SUM(CASE WHEN final_domain IN ('127.0.0.1','localhost','') OR target_url LIKE '%127.0.0.1%' THEN 1 ELSE 0 END) AS pruebas_locales
FROM prospect_logs;
" 2>/dev/null || true

echo -e "\n--- EMBUDO DE CONVERSIÓN WEB Y VENTAS (audit_sessions) ---"
sqlite3 -header -column "${DB}" "
SELECT
  COUNT(*) AS total_escaneos,
  SUM(CASE WHEN is_accredited = 1 THEN 1 ELSE 0 END) AS pagados_zip_liberado,
  SUM(CASE WHEN is_accredited = 0 THEN 1 ELSE 0 END) AS pendientes_pago
FROM audit_sessions;
" 2>/dev/null || echo "  (Sin registros aún en audit_sessions)"

echo -e "\n--- ÚLTIMOS 15 EVENTOS DE FUNNEL EN journalctl (sin ruido 404 de bots) ---"
sudo journalctl -u mallen.service -n 80 --no-pager 2>/dev/null \
  | grep -E "FUNNEL|ERROR|WARN|panic" \
  | grep -Ev "/wp-admin|/Public/|/static/admin|/favicon\.ico|/\.env|/xmlrpc\.php|/v2/health" \
  | tail -n 15 || echo "  (Sin eventos recientes)"
echo "=========================================================================="
