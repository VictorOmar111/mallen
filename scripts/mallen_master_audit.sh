#!/usr/bin/env bash
set -euo pipefail
umask 077

cd ~/worker_inegi_engine
PROJECT_DIR="$PWD"

WITH_STRIPE=0
BASE_URL="http://127.0.0.1:8080"

while [ $# -gt 0 ]; do
  case "$1" in
    --with-stripe)
      WITH_STRIPE=1
      shift
      ;;
    --base-url)
      BASE_URL="${2:?Falta valor para --base-url}"
      shift 2
      ;;
    -h|--help)
      echo "Uso: $0 [--with-stripe] [--base-url http://127.0.0.1:8080]"
      exit 0
      ;;
    *)
      echo "Opción desconocida: $1"
      exit 1
      ;;
  esac
done

DB_PATH="${MALLEN_DB_PATH:-data/mallen_prod.db}"
if [ ! -f "$DB_PATH" ]; then
  echo "!! ERROR: No existe la base de datos en ${DB_PATH}"
  exit 1
fi

if [ "$WITH_STRIPE" -eq 1 ]; then
  if ! sudo -n true 2>/dev/null; then
    echo "!! ERROR: --with-stripe requiere sudo sin contraseña (sudo -n true falló)."
    exit 1
  fi
fi

TS="$(date +%Y%m%d_%H%M%S)"
START_ISO="$(date '+%Y-%m-%d %H:%M:%S')"
LOG_FILE="/tmp/mallen_cert_${TS}.log"
BACKUP_ENV=".env.pre_master_audit_${TS}.bak"
ROLLBACK_ARMED=0

cleanup_all() {
  set +e
  unset TEST_SK TEST_WH || true
  if [ "${ROLLBACK_ARMED}" -eq 1 ]; then
    echo "!! Rollback activado: restaurando .env original y reiniciando mallen.service..." | tee -a "$LOG_FILE"
    if [ -f "$BACKUP_ENV" ]; then
      cp "$BACKUP_ENV" .env
      chmod 600 .env
      rm -f "$BACKUP_ENV"
    fi
    sudo -n systemctl restart mallen.service || true
  fi
}
trap cleanup_all EXIT INT TERM HUP

log() {
  echo "$@" | tee -a "$LOG_FILE"
}

mark_fail() {
  log "  [FAIL] $*"
}

mark_warn() {
  log "  [WARN] $*"
}

mark_pass() {
  log "  [PASS] $*"
}

wait_health() {
  local tries="${1:-25}"
  for i in $(seq 1 "$tries"); do
    if [ "$(curl -s -o /dev/null -w '%{http_code}' "${BASE_URL}/health" || true)" = "200" ]; then
      return 0
    fi
    sleep 1
  done
  return 1
}

log "========================================================================"
log "  MALLEN 360° MASTER AUDIT (v4.2) | ${START_ISO} | WITH_STRIPE=${WITH_STRIPE}"
log "  URL: ${BASE_URL} | DB: ${DB_PATH} | Log: ${LOG_FILE}"
log "========================================================================"

NRESTARTS_BEFORE="$(systemctl show mallen.service -p NRestarts --value 2>/dev/null || echo 0)"
MAIN_PID_BEFORE="$(systemctl show mallen.service -p MainPID --value 2>/dev/null || echo 0)"
MAIN_PID_BEFORE="${MAIN_PID_BEFORE:-0}"
[[ "$MAIN_PID_BEFORE" =~ ^[0-9]+$ ]] || MAIN_PID_BEFORE=0

RSS_BEFORE_KB=0
if [ "$MAIN_PID_BEFORE" -gt 0 ] && [ -r "/proc/${MAIN_PID_BEFORE}/status" ]; then
  RSS_BEFORE_KB="$(awk '/VmRSS:/ {print $2}' "/proc/${MAIN_PID_BEFORE}/status" 2>/dev/null || echo 0)"
fi
RSS_BEFORE_KB="${RSS_BEFORE_KB:-0}"

FAILS_BEFORE_RO="$(grep -c '\[FAIL\]' "$LOG_FILE" || true)"
set +e
python3 scripts/mallen_audit_core.py --mode readonly --db "$DB_PATH" --base-url "$BASE_URL" --project-dir "$PROJECT_DIR" 2>&1 | tee -a "$LOG_FILE"
PY_RO_RC=${PIPESTATUS[0]}
set -e
FAILS_AFTER_RO="$(grep -c '\[FAIL\]' "$LOG_FILE" || true)"
if [ "$PY_RO_RC" -ne 0 ] && [ "$FAILS_AFTER_RO" -eq "$FAILS_BEFORE_RO" ]; then
  mark_fail "mallen_audit_core.py (readonly) abortó con código ${PY_RO_RC}"
fi

NRESTARTS_MID="$(systemctl show mallen.service -p NRestarts --value 2>/dev/null || echo 0)"
if [ "$NRESTARTS_MID" -ne "$NRESTARTS_BEFORE" ]; then
  mark_fail "mallen.service se reinició durante fase readonly (${NRESTARTS_BEFORE} -> ${NRESTARTS_MID})"
fi

PEAK_HWM_KB=0
if [ "$MAIN_PID_BEFORE" -gt 0 ] && [ -r "/proc/${MAIN_PID_BEFORE}/status" ]; then
  PEAK_HWM_KB="$(awk '/VmHWM:/ {print $2}' "/proc/${MAIN_PID_BEFORE}/status" 2>/dev/null || echo 0)"
fi
PEAK_HWM_KB="${PEAK_HWM_KB:-0}"
PEAK_HWM_MB=$((PEAK_HWM_KB / 1024))

NRESTARTS_BASE_FINAL="$NRESTARTS_MID"

if [ "$WITH_STRIPE" -eq 1 ]; then
  log ""
  log "=== PILAR 3: STRIPE TEST E2E (MSI + OXXO + REPLAY + ZIP + LIMPIEZA) ==="
  if grep -qE "^[[:space:]]*MALLEN_AUTO_CFDI[[:space:]]*=[[:space:]]*[\"']?1[\"']?([[:space:]]*(#.*)?)?$" .env 2>/dev/null; then
    mark_fail "MALLEN_AUTO_CFDI=1 está activo en .env. Abortando Pilar 3."
    exit 1
  fi

  cp .env "$BACKUP_ENV"
  chmod 600 "$BACKUP_ENV"
  ROLLBACK_ARMED=1

  read -rsp "Pega tu STRIPE_SECRET_KEY de PRUEBA (sk_test_...): " TEST_SK
  echo ""
  read -rsp "Pega tu STRIPE_WEBHOOK_SECRET de PRUEBA (whsec_...): " TEST_WH
  echo ""
  [[ "$TEST_SK" == sk_test_* ]] || { mark_fail "STRIPE_SECRET_KEY debe empezar con sk_test_"; exit 1; }
  [[ "$TEST_WH" == whsec_* ]]   || { mark_fail "STRIPE_WEBHOOK_SECRET debe empezar con whsec_"; exit 1; }

  AUDIT_STRIPE_SK="$TEST_SK" AUDIT_STRIPE_WH="$TEST_WH" python3 - << 'ENVEOF'
import os, re
sk = os.environ["AUDIT_STRIPE_SK"]
wh = os.environ["AUDIT_STRIPE_WH"]
with open(".env", "r", encoding="utf-8") as f:
    s = f.read()
def upsert(text, k, v):
    pat = re.compile(rf"^#?\s*{k}=.*$", re.MULTILINE)
    if pat.search(text):
        return pat.sub(lambda _: f"{k}={v}", text, count=1)
    return text.rstrip("\n") + f"\n{k}={v}\n"
s = upsert(upsert(s, "STRIPE_SECRET_KEY", sk), "STRIPE_WEBHOOK_SECRET", wh)
with open(".env", "w", encoding="utf-8") as f:
    f.write(s)
ENVEOF
  chmod 600 .env

  sudo -n systemctl restart mallen.service
  if ! wait_health 25; then
    mark_fail "/health no respondió HTTP 200 en 25s tras activar llaves de prueba"
    exit 1
  fi

  FAILS_BEFORE_ST="$(grep -c '\[FAIL\]' "$LOG_FILE" || true)"
  set +e
  AUDIT_STRIPE_SK="$TEST_SK" AUDIT_STRIPE_WH="$TEST_WH" \
    python3 scripts/mallen_audit_core.py --mode stripe --db "$DB_PATH" --base-url "$BASE_URL" --project-dir "$PROJECT_DIR" --ts "$TS" \
    2>&1 | tee -a "$LOG_FILE"
  PY_ST_RC=${PIPESTATUS[0]}
  set -e
  FAILS_AFTER_ST="$(grep -c '\[FAIL\]' "$LOG_FILE" || true)"
  if [ "$PY_ST_RC" -ne 0 ] && [ "$FAILS_AFTER_ST" -eq "$FAILS_BEFORE_ST" ]; then
    mark_fail "mallen_audit_core.py (stripe) abortó con código ${PY_ST_RC}"
  fi

  cp "$BACKUP_ENV" .env
  chmod 600 .env
  sudo -n systemctl restart mallen.service
  if ! wait_health 25; then
    mark_fail "/health no respondió HTTP 200 en 25s tras restaurar .env"
    exit 1
  fi

  ROLLBACK_ARMED=0
  rm -f "$BACKUP_ENV"
  NRESTARTS_BASE_FINAL="$(systemctl show mallen.service -p NRestarts --value 2>/dev/null || echo 0)"
  mark_pass ".env original restaurado y mallen.service en HTTP 200"
fi

log ""
log "=== PILAR 5: TELEMETRÍA DEL SISTEMA, RELOJ NTP, SEGURIDAD Y ALMACENAMIENTO ==="
if systemctl is-active --quiet mallen.service; then
  mark_pass "mallen.service estado active"
else
  mark_fail "mallen.service no está active"
fi

NRESTARTS_AFTER="$(systemctl show mallen.service -p NRestarts --value 2>/dev/null || echo 0)"
MAIN_PID_AFTER="$(systemctl show mallen.service -p MainPID --value 2>/dev/null || echo 0)"
MAIN_PID_AFTER="${MAIN_PID_AFTER:-0}"
[[ "$MAIN_PID_AFTER" =~ ^[0-9]+$ ]] || MAIN_PID_AFTER=0

RSS_AFTER_KB=0
if [ "$MAIN_PID_AFTER" -gt 0 ] && [ -r "/proc/${MAIN_PID_AFTER}/status" ]; then
  RSS_AFTER_KB="$(awk '/VmRSS:/ {print $2}' "/proc/${MAIN_PID_AFTER}/status" 2>/dev/null || echo 0)"
fi
RSS_AFTER_KB="${RSS_AFTER_KB:-0}"
RSS_AFTER_MB=$((RSS_AFTER_KB / 1024))

[ "$NRESTARTS_AFTER" -eq "$NRESTARTS_BASE_FINAL" ] \
  && mark_pass "mallen.service NRestarts delta=0 (total=${NRESTARTS_AFTER})" \
  || mark_fail "mallen.service tuvo reinicios inesperados (${NRESTARTS_BASE_FINAL} -> ${NRESTARTS_AFTER})"

if [ "$PEAK_HWM_KB" -eq 0 ]; then
  mark_warn "No se pudo leer VmHWM (MAIN_PID_BEFORE=${MAIN_PID_BEFORE}) | RSS final=${RSS_AFTER_MB} MB"
elif [ "$PEAK_HWM_MB" -lt 512 ]; then
  mark_pass "Memoria pico real (VmHWM pre-restart)=${PEAK_HWM_MB} MB | RSS final=${RSS_AFTER_MB} MB (antes=$((RSS_BEFORE_KB / 1024)) MB, umbral < 512 MB)"
else
  mark_fail "Memoria pico (VmHWM)=${PEAK_HWM_MB} MB excede 512 MB"
fi

NTP_SYNC="$(timedatectl show -p NTPSynchronized --value 2>/dev/null || echo unknown)"
if [ "$NTP_SYNC" = "yes" ]; then
  mark_pass "Reloj del servidor sincronizado vía NTP (NTPSynchronized=yes)"
elif [ "$NTP_SYNC" = "no" ]; then
  mark_fail "Reloj NTP desincronizado (NTPSynchronized=no — riesgo para ventana +/-300s de Stripe)"
else
  mark_warn "No se pudo verificar NTPSynchronized (valor=${NTP_SYNC})"
fi

WAL_BYTES="$(stat -c%s "${DB_PATH}-wal" 2>/dev/null || echo 0)"
WAL_MB=$((WAL_BYTES / 1048576))
[ "$WAL_MB" -lt 50 ] \
  && mark_pass "SQLite WAL=${WAL_MB} MB (umbral < 50 MB) | DB=$(du -h "$DB_PATH" | awk '{print $1}')" \
  || mark_fail "SQLite WAL=${WAL_MB} MB excede 50 MB"

for mount_pt in "/" "." "/tmp"; do
  use_pct="$(df -P "$mount_pt" | awk 'NR==2 {gsub("%","",$5); print $5}')"
  free_pct=$((100 - use_pct))
  [ "$free_pct" -gt 20 ] \
    && mark_pass "Espacio libre en '${mount_pt}' = ${free_pct}% (usado=${use_pct}%)" \
    || mark_fail "Espacio libre en '${mount_pt}' = ${free_pct}% (<= 20%)"
done

ENV_PERM="$(stat -c%a .env 2>/dev/null || echo missing)"
[ "$ENV_PERM" = "600" ] \
  && mark_pass "Permisos de .env = ${ENV_PERM}" \
  || mark_fail "Permisos de .env = ${ENV_PERM} (esperado 600)"

if { [ -r .env ] && cat .env; systemctl cat mallen.service 2>/dev/null; systemctl show mallen.service -p Environment --value 2>/dev/null; } \
  | grep -qE "(^|[[:space:]])(Environment=)?[\"']?MALLEN_ALLOW_SIMULATED_WEBHOOK[\"']?[[:space:]]*=[[:space:]]*[\"']?1[\"']?([[:space:]]|$|#)"; then
  mark_fail "MALLEN_ALLOW_SIMULATED_WEBHOOK=1 detectado en .env o systemd"
else
  mark_pass "MALLEN_ALLOW_SIMULATED_WEBHOOK apagado (.env + systemd/drop-ins)"
fi

PANICS="$(journalctl -u mallen.service --since "$START_ISO" --no-pager | grep -Ei "panic|SQLITE_BUSY|database is locked" | wc -l || true)"
[ "$PANICS" -eq 0 ] \
  && mark_pass "journalctl desde ${START_ISO}: 0 panics / 0 SQLITE_BUSY" \
  || mark_fail "journalctl registró ${PANICS} líneas con panic/SQLITE_BUSY"

TOTAL_FAILS="$(grep -c '\[FAIL\]' "$LOG_FILE" || true)"
TOTAL_WARNS="$(grep -c '\[WARN\]' "$LOG_FILE" || true)"

log "========================================================================"
if [ "$TOTAL_FAILS" -eq 0 ]; then
  log "  VEREDICTO FINAL: CERTIFICADO (PASS) | Fallos=0 | Avisos=${TOTAL_WARNS}"
  log "  Log guardado en: ${LOG_FILE}"
  log "========================================================================"
  exit 0
else
  log "  VEREDICTO FINAL: REPROBADO (FAIL) | Fallos=${TOTAL_FAILS} | Avisos=${TOTAL_WARNS}"
  log "  Log guardado en: ${LOG_FILE}"
  log "========================================================================"
  exit 1
fi
