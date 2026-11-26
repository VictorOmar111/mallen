#!/usr/bin/env python3
"""
MALLEN 360° — Auditoría Exhaustiva (v2.1):
- 931 Códigos SCIAN 6D (determinista, agrupación 2D, anomalías DENUE, Counter HTTP, cents_none)
- 7 Supuestos SAT (69 + 69-B) + Doble Incidencia (sin '&') + pdftotext con timeout/returncode
- Keep-alive por hilo con cierre limpio en finally
- Telemetría en caliente: VmRSS, VmHWM, /proc/<pid>/fd, WAL y extracto de journalctl ante fallos
"""
import collections
import concurrent.futures
import contextlib
import http.client
import json
import math
import os
import sqlite3
import subprocess
import sys
import threading
import time
import urllib.parse

BASE = os.environ.get("MALLEN_BASE_URL", "http://127.0.0.1:8080")
DB_PATH = os.environ.get("MALLEN_DB_PATH", "data/mallen_prod.db")
WORKERS = int(os.environ.get("MALLEN_FULL_SCIAN_WORKERS", "4"))

_parsed = urllib.parse.urlparse(BASE)
_HOST = _parsed.hostname or "127.0.0.1"
_PORT = _parsed.port or (443 if _parsed.scheme == "https" else 80)
_tls = threading.local()
_conns_lock = threading.Lock()
_all_conns = []
_retries = 0


def pct(arr, p):
    if not arr:
        return 0.0
    s = sorted(arr)
    idx = min(len(s) - 1, max(0, math.ceil((p / 100.0) * len(s)) - 1))
    return s[idx]


def _get_conn():
    conn = getattr(_tls, "conn", None)
    if conn is None:
        conn = http.client.HTTPConnection(_HOST, _PORT, timeout=20)
        _tls.conn = conn
        with _conns_lock:
            _all_conns.append(conn)
    return conn


def close_all_conns():
    with _conns_lock:
        for c in _all_conns:
            try:
                c.close()
            except Exception:
                pass
        _all_conns.clear()


def http_req(method, path, payload=None):
    global _retries
    t0 = time.perf_counter()
    body_bytes = json.dumps(payload).encode("utf-8") if payload is not None else None
    headers = {"Connection": "keep-alive"}
    if body_bytes is not None:
        headers["Content-Type"] = "application/json"

    for attempt in range(2):
        conn = _get_conn()
        try:
            conn.request(method, path, body=body_bytes, headers=headers)
            resp = conn.getresponse()
            data = resp.read()
            return resp.status, data, (time.perf_counter() - t0) * 1000.0
        except Exception as e:
            try:
                conn.close()
            except Exception:
                pass
            _tls.conn = None
            if attempt == 0:
                with _conns_lock:
                    _retries += 1
            else:
                return 0, str(e).encode("utf-8"), (time.perf_counter() - t0) * 1000.0


def check_cents_parity(j_obj, total_fine):
    findings = j_obj.get("findings")
    if not isinstance(findings, list) or not findings or total_fine is None:
        return None, 0
    sum_cents = 0
    counted = 0
    for item in findings:
        if isinstance(item, dict):
            val = item.get("estimated_fine_mxn", item.get("multa_estimada_mxn"))
            if isinstance(val, (int, float)):
                sum_cents += round(float(val) * 100)
                counted += 1
    if counted == 0:
        return None, 0
    delta = abs(sum_cents - round(float(total_fine) * 100))
    return delta == 0, delta


def get_service_telemetry():
    pid = subprocess.run(
        ["systemctl", "show", "mallen.service", "-p", "MainPID", "--value"],
        capture_output=True, text=True, check=False,
    ).stdout.strip()
    nrestarts = subprocess.run(
        ["systemctl", "show", "mallen.service", "-p", "NRestarts", "--value"],
        capture_output=True, text=True, check=False,
    ).stdout.strip()
    rss_mb = hwm_mb = fd_count = 0
    if pid.isdigit() and int(pid) > 0:
        status_p = f"/proc/{pid}/status"
        fd_p = f"/proc/{pid}/fd"
        if os.path.exists(status_p):
            with open(status_p, "r", encoding="utf-8") as f:
                for line in f:
                    if line.startswith("VmRSS:"):
                        rss_mb = int(line.split()[1]) // 1024
                    elif line.startswith("VmHWM:"):
                        hwm_mb = int(line.split()[1]) // 1024
        if os.path.isdir(fd_p):
            try:
                fd_count = len(os.listdir(fd_p))
            except Exception:
                fd_count = -1
    wal_path = f"{DB_PATH}-wal"
    wal_mb = (os.path.getsize(wal_path) / (1024 * 1024)) if os.path.exists(wal_path) else 0.0
    return pid, int(nrestarts or 0), rss_mb, hwm_mb, fd_count, wal_mb


def cleanup_synthetic():
    with contextlib.closing(sqlite3.connect(DB_PATH, timeout=15)) as wconn:
        wconn.execute("PRAGMA busy_timeout = 5000;")
        c1 = wconn.execute("DELETE FROM audit_sessions WHERE tax_id LIKE 'AUD%'").rowcount
        c2 = wconn.execute("DELETE FROM audit_snapshots WHERE tax_id_key LIKE 'AUD%'").rowcount
        c3 = wconn.execute("DELETE FROM prospects WHERE tax_id LIKE 'AUD%'").rowcount
        wconn.commit()
    return c1, c2, c3


def main():
    t_start_iso = time.strftime("%Y-%m-%d %H:%M:%S")
    pid0, nr0, rss0, hwm0, fd0, wal0 = get_service_telemetry()
    print("=" * 78)
    print(f"  MALLEN 360° FULL SCIAN + SAT AUDIT (v2.1) | {t_start_iso} | workers={WORKERS}")
    print(f"  PID={pid0} | RSS={rss0} MB | VmHWM={hwm0} MB | FDs={fd0} | WAL={wal0:.2f} MB")
    print("=" * 78)

    db_uri = f"file:{urllib.parse.quote(os.path.abspath(DB_PATH))}?mode=ro"
    with contextlib.closing(sqlite3.connect(db_uri, uri=True)) as conn:
        conn.execute("PRAGMA query_only = ON;")
        conn.execute("PRAGMA busy_timeout = 5000;")
        anomalies = conn.execute(
            """
            SELECT
              SUM(CASE WHEN codigo_scian IS NULL OR TRIM(codigo_scian)='' THEN 1 ELSE 0 END),
              SUM(CASE WHEN length(TRIM(COALESCE(codigo_scian,''))) != 6 THEN 1 ELSE 0 END),
              SUM(CASE WHEN codigo_scian GLOB '*[^0-9]*' THEN 1 ELSE 0 END),
              SUM(CASE WHEN substr(codigo_scian,1,2)='93' THEN 1 ELSE 0 END)
            FROM denue_units
            """
        ).fetchone()
        print(
            f"\n=== 0. HIGIENE DENUE_UNITS ===\n"
            f"  nulos/vacíos={anomalies[0] or 0} | len!=6={anomalies[1] or 0} "
            f"| no_numéricos={anomalies[2] or 0} | gobierno_93*={anomalies[3] or 0}"
        )

        scian_rows = conn.execute(
            """
            SELECT u.codigo_scian, u.nom_estab, u.raz_social
            FROM denue_units u
            JOIN (
                SELECT codigo_scian, MIN(id_denue) AS min_id
                FROM denue_units
                WHERE length(codigo_scian) = 6 AND codigo_scian NOT GLOB '*[^0-9]*'
                GROUP BY codigo_scian
            ) m ON u.codigo_scian = m.codigo_scian AND u.id_denue = m.min_id
            ORDER BY u.codigo_scian
            """
        ).fetchall()

        sat_cases = []
        for sup in ("FIRMES", "NO LOCALIZADOS", "EXIGIBLES"):
            r = conn.execute(
                "SELECT TRIM(rfc) FROM sat_69_incumplidos "
                "WHERE supuesto=? AND LENGTH(TRIM(rfc)) BETWEEN 12 AND 13 AND rfc NOT LIKE '%&%' LIMIT 1",
                (sup,),
            ).fetchone()
            if r:
                sat_cases.append((f"SAT69:{sup}", r[0], None))

        for sit in ("Definitivo", "Presunto", "Sentencia Favorable", "Desvirtuado"):
            r = conn.execute(
                "SELECT TRIM(rfc) FROM sat_69b_efos "
                "WHERE situacion=? AND LENGTH(TRIM(rfc)) BETWEEN 12 AND 13 AND rfc NOT LIKE '%&%' LIMIT 1",
                (sit,),
            ).fetchone()
            if r:
                sat_cases.append((f"69B:{sit}", r[0], sit.upper()))

        r_dual = conn.execute(
            """
            SELECT UPPER(TRIM(a.rfc))
            FROM sat_69_incumplidos a
            JOIN sat_69b_efos b ON UPPER(TRIM(a.rfc)) = UPPER(TRIM(b.rfc))
            WHERE LENGTH(TRIM(a.rfc)) BETWEEN 12 AND 13 AND a.rfc NOT LIKE '%&%'
            LIMIT 1
            """
        ).fetchone()
        if r_dual:
            sat_cases.append(("DUAL:69+69B", r_dual[0], None))

    fails = 0
    cleaned = False
    try:
        print(f"\n=== 1. BARRIDO EXHAUSTIVO DE LOS {len(scian_rows)} CÓDIGOS SCIAN 6D (RFC 12-CHAR) ===")

        def scan_scian(row):
            scian, nom, raz = row
            display_name = (raz or nom or f"EMPRESA SCIAN {scian}").strip()
            tax_id = f"AUD{scian}A01"
            st, body, ms = http_req(
                "POST",
                "/api/scan",
                {"tax_id": tax_id, "razon_social_override": display_name, "domain": None},
            )
            if st != 200:
                snippet = body.decode("utf-8", errors="replace")[:120]
                return scian, st, False, False, ms, 0.0, f"HTTP {st} ({snippet})"
            try:
                j = json.loads(body.decode("utf-8"))
            except Exception as e:
                return scian, st, False, False, ms, 0.0, f"JSON err: {e}"
            risk = int(j.get("risk_score", 0) or 0)
            fine = float(j.get("total_estimated_fine_mxn", 0.0) or 0.0)
            ret_scian = str(j.get("codigo_scian", "")).strip()
            cents_ok, delta = check_cents_parity(j, fine)
            cents_none = cents_ok is None
            ok = (risk > 0) and (fine > 0.0) and (len(ret_scian) == 6) and (cents_ok is not False)
            reason = "" if ok else f"risk={risk} fine={fine} ret_scian={ret_scian} delta={delta}c"
            return scian, st, ok, cents_none, ms, fine, reason

        t0_all = time.perf_counter()
        latencies = []
        scian_failures = []
        http_counts = collections.Counter()
        cents_none_count = 0
        by_2d = collections.defaultdict(lambda: {"n": 0, "ok": 0, "lat": [], "fines": []})

        with concurrent.futures.ThreadPoolExecutor(max_workers=WORKERS) as ex:
            for scian, st, ok, cents_none, ms, fine, reason in ex.map(scan_scian, scian_rows):
                latencies.append(ms)
                http_counts[st] += 1
                if cents_none:
                    cents_none_count += 1
                g2 = scian[:2]
                by_2d[g2]["n"] += 1
                by_2d[g2]["lat"].append(ms)
                if ok:
                    by_2d[g2]["ok"] += 1
                    by_2d[g2]["fines"].append(fine)
                else:
                    scian_failures.append((scian, reason))

        elapsed_s = time.perf_counter() - t0_all
        passed_scian = len(scian_rows) - len(scian_failures)
        if scian_failures:
            fails += len(scian_failures)
            print(f"  [FAIL] SCIAN 6D cubiertos: {passed_scian}/{len(scian_rows)} | fallos={len(scian_failures)} en {elapsed_s:.1f}s")
            for sc, rsn in scian_failures[:15]:
                print(f"         -> SCIAN {sc}: {rsn}")
        else:
            print(
                f"  [PASS] 100% SCIAN 6D certificados: {passed_scian}/{len(scian_rows)} en {elapsed_s:.1f}s "
                f"| p50={pct(latencies, 50):.1f}ms | p95={pct(latencies, 95):.1f}ms | p99={pct(latencies, 99):.1f}ms"
            )
        print(f"  HTTP status={dict(http_counts)} | retries={_retries} | sin_findings={cents_none_count}")

        print("\n  Resumen por Sector SCIAN 2D (n | ok | p95_ms | multa_prom_mxn):")
        for g2 in sorted(by_2d.keys()):
            d = by_2d[g2]
            avg_f = sum(d["fines"]) / len(d["fines"]) if d["fines"] else 0.0
            print(f"    2D={g2} | n={d['n']:<3} | ok={d['ok']:<3} | p95={pct(d['lat'], 95):>6.1f}ms | avg_fine=${avg_f:>11,.2f}")

        print(f"\n=== 2. COBERTURA TOTAL SAT ({len(sat_cases)} SUPUESTOS + DOBLE INCIDENCIA + PDFTEXT) ===")
        for label, rfc, expect_69b in sat_cases:
            st, body, _ = http_req("POST", "/api/scan", {"tax_id": rfc})
            if st != 200:
                print(f"  [FAIL] {label:<24} {rfc:<13} | POST /api/scan -> HTTP {st}")
                fails += 1
                continue
            try:
                j = json.loads(body.decode("utf-8"))
            except Exception as e:
                print(f"  [FAIL] {label:<24} {rfc:<13} | JSON inválido: {e}")
                fails += 1
                continue
            rfc = rfc.strip().upper()
            risk = int(j.get("risk_score", 0) or 0)
            fine = float(j.get("total_estimated_fine_mxn", 0.0) or 0.0)
            ret_scian = str(j.get("codigo_scian", "")).strip()
            s69b = str(j.get("sat_69b_status", "")).upper()
            s69 = str(j.get("sat_69_status", "")).upper()
            cents_ok, delta = check_cents_parity(j, fine)

            stp, pdf_bytes, _ = http_req("GET", f"/api/micro_scan/{urllib.parse.quote(rfc, safe='')}")
            pdf_ok = stp == 200 and pdf_bytes.startswith(b"%PDF-") and len(pdf_bytes) > 5000
            txt = ""
            pdftxt_rc = -1
            if pdf_ok:
                proc = subprocess.run(
                    ["pdftotext", "-", "-"], input=pdf_bytes, capture_output=True, timeout=15, check=False
                )
                pdftxt_rc = proc.returncode
                if pdftxt_rc == 0:
                    txt = proc.stdout.decode("utf-8", errors="replace")

            f2 = f"{fine:,.2f}"
            f0 = f"{int(round(fine)):,}"
            no_placeholders = ("{{" not in txt) and ("}}" not in txt)
            scian_in_pdf = (ret_scian in txt) if ret_scian else True
            pdf_txt_ok = pdf_ok and (pdftxt_rc == 0) and (rfc in txt) and (f2 in txt or f0 in txt) and no_placeholders and scian_in_pdf
            if label == "DUAL:69+69B":
                s69b_ok = (s69 not in ("", "LIMPIO")) and (s69b not in ("", "LIMPIO"))
            elif expect_69b:
                kw = "FAVORABLE" if "FAVORABLE" in expect_69b else expect_69b.split()[0]
                s69b_ok = kw in s69b
            else:
                s69b_ok = True
            ok = pdf_txt_ok and s69b_ok and (risk > 0) and (fine > 0.0) and (cents_ok is not False)
            if not ok:
                fails += 1
            print(
                f"  [{'PASS' if ok else 'FAIL'}] {label:<24} {rfc:<13} | risk={risk:<2} | fine=${fine:<11.2f} "
                f"| 69B={s69b:<11} | cents_d={delta}c | pdf_txt={pdf_txt_ok}"
            )

        c1, c2, c3 = cleanup_synthetic()
        cleaned = True
        print(f"\n  [PASS] Limpieza sintética (AUD%): audit_sessions={c1}, audit_snapshots={c2}, prospects={c3}")
        pid1, nr1, rss1, hwm1, fd1, wal1 = get_service_telemetry()
        mem_ok = hwm1 < 512 and (nr1 - nr0) == 0 and pid1 == pid0 and wal1 < 50.0
        if not mem_ok:
            fails += 1
        print("\n=== 3. TELEMETRÍA EN CALIENTE (RAM, FDs, WAL) Y JOURNALCTL ===")
        print(
            f"  [{'PASS' if mem_ok else 'FAIL'}] PID={pid1} | NRestarts_d={nr1 - nr0} | "
            f"RSS={rss0}->{rss1} MB | VmHWM={hwm0}->{hwm1} MB | FDs={fd0}->{fd1} | WAL={wal0:.2f}->{wal1:.2f} MB"
        )

        jout = subprocess.run(
            ["journalctl", "-u", "mallen.service", "--since", t_start_iso, "--no-pager"],
            capture_output=True, text=True, check=False,
        ).stdout
        bad_lines = [
            ln for ln in jout.splitlines()
            if "panicked at" in ln.lower() or "sqlite_busy" in ln.lower() or "database is locked" in ln.lower()
        ]
        log_ok = len(bad_lines) == 0
        if not log_ok:
            fails += 1
        print(f"  [{'PASS' if log_ok else 'FAIL'}] journalctl desde {t_start_iso}: {len(bad_lines)} errores críticos (panics/SQLITE_BUSY)")
        for ln in bad_lines[-5:]:
            print(f"         -> {ln}")

    finally:
        close_all_conns()
        if not cleaned:
            c1, c2, c3 = cleanup_synthetic()
            print(f"  [PASS] Limpieza finally (AUD%): audit_sessions={c1}, audit_snapshots={c2}, prospects={c3}")

    print("=" * 78)
    print(f"  VEREDICTO FULL SCIAN: {'CERTIFICADO (PASS)' if fails == 0 else 'REPROBADO (FAIL)'} | Fallos={fails}")
    print("=" * 78)
    sys.exit(0 if fails == 0 else 1)


if __name__ == "__main__":
    main()
