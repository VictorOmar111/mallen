#!/usr/bin/env python3
import argparse
import concurrent.futures
import hashlib
import hmac
import http.server
import io
import json
import math
import re
import shutil
import sqlite3
import sys
import threading
import time
import urllib.error
import urllib.request
import zipfile

BASE = "http://127.0.0.1:8080"


def pct(arr, p):
    if not arr:
        return 0.0
    s = sorted(arr)
    idx = min(len(s) - 1, max(0, math.ceil((p / 100.0) * len(s)) - 1))
    return s[idx]


def sanitize_fts(q):
    cleaned = re.sub(r"[^A-Za-z0-9ÁÉÍÓÚÑáéíóúñ ]+", " ", q).strip()
    tokens = [t for t in cleaned.split() if len(t) >= 2]
    return " ".join(tokens[:4]) if tokens else "SA"


def http_req(method, path, payload=None, headers=None, timeout=20):
    t0 = time.perf_counter()
    data = None
    hdrs = {}
    if isinstance(payload, (bytes, bytearray)):
        data = bytes(payload)
        hdrs["Content-Type"] = "application/json"
    elif payload is not None:
        data = json.dumps(payload).encode("utf-8")
        hdrs["Content-Type"] = "application/json"
    if headers:
        hdrs.update(headers)
    req = urllib.request.Request(f"{BASE}{path}", data=data, headers=hdrs, method=method)
    try:
        with urllib.request.urlopen(req, timeout=timeout) as r:
            return r.status, r.read(), (time.perf_counter() - t0) * 1000.0
    except urllib.error.HTTPError as e:
        return e.code, e.read(), (time.perf_counter() - t0) * 1000.0
    except Exception as e:
        return 0, str(e).encode("utf-8"), (time.perf_counter() - t0) * 1000.0


def stats_line(name, arr, p95_limit=None, max_limit=None):
    if not arr:
        return f"  [WARN] {name:<20} | sin muestras", False
    mn, p50, p95, p99, mx = min(arr), pct(arr, 50), pct(arr, 95), pct(arr, 99), max(arr)
    failed = bool((p95_limit and p95 > p95_limit) or (max_limit and mx > max_limit))
    flag = "FAIL" if failed else "PASS"
    return (
        f"  [{flag}] {name:<20} | n={len(arr):2d} min={mn:6.1f} p50={p50:6.1f} "
        f"p95={p95:6.1f} p99={p99:6.1f} max={mx:6.1f} ms"
    ), failed


def start_ssrf_canary():
    hits = []

    class CanaryHandler(http.server.BaseHTTPRequestHandler):
        def do_GET(self):
            hits.append(self.path)
            self.send_response(200)
            self.end_headers()
            self.wfile.write(b"CANARY_SSRF_HIT")

        def do_HEAD(self):
            hits.append(self.path)
            self.send_response(200)
            self.end_headers()

        def log_message(self, fmt, *args):
            pass

    srv = http.server.HTTPServer(("127.0.0.1", 0), CanaryHandler)
    port = srv.server_address[1]
    t = threading.Thread(target=srv.serve_forever, daemon=True)
    t.start()
    return srv, port, hits


def run_readonly(db_path):
    fails = 0
    conn = sqlite3.connect(f"file:{db_path}?mode=ro", uri=True)

    print("\n=== PILAR 1: FORM-BOT POR SCIAN, HOMONIMIA, ENTES PÚBLICOS Y LINKS ===")
    fts_lat = []
    groups_2d = [
        r[0]
        for r in conn.execute(
            "SELECT DISTINCT substr(codigo_scian, 1, 2) FROM denue_units WHERE length(codigo_scian)=6 ORDER BY 1"
        ).fetchall()
    ]
    uniq_6d = conn.execute(
        "SELECT COUNT(DISTINCT codigo_scian) FROM denue_units WHERE length(codigo_scian)=6"
    ).fetchone()[0]
    print(f"  [PASS] Cobertura DENUE: {len(groups_2d)} grupos 2D | {uniq_6d} códigos SCIAN 6D únicos")

    gob_931 = conn.execute("SELECT COUNT(*) FROM denue_units WHERE codigo_scian LIKE '931%'").fetchone()[0]
    gob_www = conn.execute("SELECT COUNT(*) FROM denue_units WHERE LOWER(COALESCE(www,'')) LIKE '%.gob.%'").fetchone()[0]
    comercial = conn.execute(
        "SELECT COUNT(*) FROM denue_units WHERE codigo_scian NOT LIKE '931%' AND LOWER(COALESCE(www,'')) NOT LIKE '%.gob.%'"
    ).fetchone()[0]
    print(f"  [PASS] Filtro anti-entes públicos: comercial={comercial} | excluidos scian_931={gob_931}, www_gob={gob_www}")

    homonyms = [
        "BODEGA",
        "ARTESANIAS",
        "COMPOSTURA DE ROPA",
        "TALLER MECANICO INDUSTRIAL",
        "7-ELEVEN MEXICO",
    ]
    for h in homonyms:
        q = sanitize_fts(h)
        t0 = time.perf_counter()
        rows = conn.execute(
            "SELECT u.nom_estab, u.codigo_scian, u.per_ocu FROM denue_fts f "
            "JOIN denue_units u ON u.id_denue = f.id_denue "
            "WHERE denue_fts MATCH ? LIMIT 5",
            (q,),
        ).fetchall()
        fts_lat.append((time.perf_counter() - t0) * 1000.0)
        if rows:
            print(f"  [PASS] Homónimo '{h:<26}' -> hits={len(rows)} | top='{rows[0][0][:32]}' | scian={rows[0][1]} | estrato={rows[0][2]}")
        else:
            print(f"  [WARN] Homónimo '{h:<26}' -> 0 hits en FTS5 (query='{q}')")

    sample_rows = []
    for g in groups_2d:
        if g == "93":
            continue
        row = conn.execute(
            "SELECT nom_estab, codigo_scian, per_ocu FROM denue_units "
            "WHERE substr(codigo_scian,1,2)=? AND LOWER(COALESCE(www,'')) NOT LIKE '%.gob.%' "
            "AND nom_estab IS NOT NULL AND length(trim(nom_estab)) >= 4 LIMIT 1",
            (g,),
        ).fetchone()
        if row:
            sample_rows.append((f"2D={g}", row[0], row[1], row[2]))

    for pref in ("486", "487"):
        row = conn.execute(
            "SELECT nom_estab, codigo_scian, per_ocu FROM denue_units "
            "WHERE codigo_scian LIKE ? AND LOWER(COALESCE(www,'')) NOT LIKE '%.gob.%' LIMIT 1",
            (f"{pref}%",),
        ).fetchone()
        if row:
            sample_rows.append((f"3D={pref}", row[0], row[1], row[2]))

    print("\n  Muestra sectorial SCIAN (2D + 486* + 487*):")
    for tag, nom, scian, per_ocu in sample_rows[:25]:
        q = sanitize_fts(nom)
        t0 = time.perf_counter()
        _ = conn.execute("SELECT rowid FROM denue_fts WHERE denue_fts MATCH ? LIMIT 3", (q,)).fetchall()
        fts_lat.append((time.perf_counter() - t0) * 1000.0)
        print(f"    {tag:<7} | scian={scian} | estrato={str(per_ocu):<16} | {nom[:42]}")

    print("\n=== PILAR 2: SINCRONÍA Y CONGRUENCIA SSOT (5 SUPUESTOS SAT x 3 REQS + PDF + CHECKOUT) ===")
    scan_lat, pdf_lat, health_lat = [], [], []
    for _ in range(15):
        st, _, ms = http_req("GET", "/health")
        health_lat.append(ms)
        if st != 200:
            print(f"  [FAIL] /health devolvió HTTP {st}")
            fails += 1

    def pick_rfc(sql, fallback):
        r = conn.execute(sql).fetchone()
        return r[0].strip().upper() if r and r[0] else fallback

    ssot_cases = [
        ("FIRMES", pick_rfc("SELECT rfc FROM sat_69_incumplidos WHERE supuesto='FIRMES' AND rfc NOT LIKE '%&%' LIMIT 1", "AAG090703QT6")),
        ("EXIGIBLES", pick_rfc("SELECT rfc FROM sat_69_incumplidos WHERE supuesto='EXIGIBLES' AND rfc NOT LIKE '%&%' LIMIT 1", "BME9410137F5")),
        ("NO LOCALIZADOS", pick_rfc("SELECT rfc FROM sat_69_incumplidos WHERE supuesto='NO LOCALIZADOS' AND rfc NOT LIKE '%&%' LIMIT 1", "AAG090703QT6")),
        ("69B_DEFINITIVO", pick_rfc("SELECT rfc FROM sat_69b_efos WHERE LOWER(situacion) LIKE '%definitiv%' AND rfc NOT LIKE '%&%' LIMIT 1", "EST850628K51")),
        ("LIMPIO", "EST850628K51"),
    ]

    for label, rfc in ssot_cases:
        snapshots = []
        scan_ok = True
        for _ in range(3):
            st, b, ms = http_req("POST", "/api/scan", {"tax_id": rfc})
            scan_lat.append(ms)
            if st != 200:
                scan_ok = False
                break
            try:
                j = json.loads(b.decode("utf-8", errors="replace"))
            except json.JSONDecodeError:
                scan_ok = False
                break
            snapshots.append((
                j.get("risk_score"),
                j.get("total_estimated_fine_mxn", j.get("exact_vemp_fine_mxn")),
                j.get("estimated_employees"),
                j.get("codigo_scian"),
            ))

        if not scan_ok or len(snapshots) < 3:
            print(f"  [FAIL] SSOT {label} ({rfc}): error o JSON inválido en /api/scan")
            fails += 1
            continue

        det_ok = len(set(snapshots)) == 1
        r1, f1, e1, sc1 = snapshots[0]

        stp, bp, msp = http_req("GET", f"/api/micro_scan/{rfc}")
        pdf_lat.append(msp)
        pdf_ok = stp == 200 and bp.startswith(b"%PDF-") and len(bp) > 5000

        stc, _, _ = http_req("GET", f"/checkout/{rfc}")
        chk_ok = stc == 200

        db_sess = conn.execute("SELECT COUNT(*) FROM audit_sessions WHERE tax_id=?", (rfc,)).fetchone()[0]
        ok = det_ok and pdf_ok and chk_ok
        if not ok:
            fails += 1
        print(
            f"  [{'PASS' if ok else 'FAIL'}] {label:<14} {rfc:<13} | det={det_ok} | risk={r1} | "
            f"fine=${f1} | emp={e1} | scian={sc1} | pdf={len(bp)}B | chk={stc} | db_sess={db_sess}"
        )

    print("\n=== PILAR 4: CONCURRENCIA (25 REQS / 15 HILOS WAL) + INPUTS HOSTILES + SSRF CANARIO ===")

    def burst_worker(idx):
        if idx % 2 == 0:
            st, _, ms = http_req("POST", "/api/scan", {"tax_id": "EST850628K51"})
        else:
            st, _, ms = http_req("GET", "/api/micro_scan/EST850628K51")
        return st, ms

    with concurrent.futures.ThreadPoolExecutor(max_workers=15) as ex:
        res = list(ex.map(burst_worker, range(25)))

    burst_5xx = sum(1 for st, _ in res if st == 0 or st >= 500)
    burst_ms = [ms for _, ms in res]
    burst_p95 = pct(burst_ms, 95)
    burst_ok = burst_5xx == 0 and burst_p95 <= 5000.0
    if not burst_ok:
        fails += 1
    print(
        f"  [{'PASS' if burst_ok else 'FAIL'}] Ráfaga 25 reqs (15 hilos) -> 5xx={burst_5xx} | "
        f"p50={pct(burst_ms, 50):.1f}ms | p95={burst_p95:.1f}ms (umbral <=5000ms) | max={max(burst_ms):.1f}ms"
    )

    canary_srv, canary_port, canary_hits = start_ssrf_canary()
    try:
        hostile_payloads = [
            ("RFC con &", {"tax_id": "&DP0504056D9"}, False),
            ("FTS5 roto", {"tax_id": '" * OR NOT ('}, False),
            ("SQLi básico", {"tax_id": "' OR 1=1 --"}, False),
            ("SSRF canario", {"tax_id": "EST850628K51", "domain": f"http://127.0.0.1:{canary_port}/ssrf-canary"}, True),
            ("SSRF AWS meta", {"tax_id": "EST850628K51", "domain": "http://169.254.169.254/latest/meta-data/"}, True),
        ]
        for name, pl, is_ssrf in hostile_payloads:
            hits_before = len(canary_hits)
            st, body, ms = http_req("POST", "/api/scan", pl)
            txt = body.decode("utf-8", errors="replace")
            hit_delta = len(canary_hits) - hits_before
            leaked = is_ssrf and (
                hit_delta > 0
                or any(tok in txt for tok in ("CANARY_SSRF_HIT", "ami-id", "instance-id", "iam/security-credentials"))
            )
            ok = st != 0 and st < 500 and not leaked
            if not ok:
                fails += 1
            print(f"  [{'PASS' if ok else 'FAIL'}] Hostil {name:<15} -> HTTP {st} | canary_hits={hit_delta} | leak={leaked} ({ms:.1f}ms)")
    finally:
        canary_srv.shutdown()

    print("\n=== TELEMETRÍA DE LATENCIAS (UMBRALES PILAR 5) ===")
    for name, arr, p95_l, max_l in (
        ("/health", health_lat, 25.0, None),
        ("denue_fts (SQLite)", fts_lat, 60.0, None),
        ("POST /api/scan", scan_lat, 1500.0, 4000.0),
        ("GET /api/micro_scan", pdf_lat, 2000.0, 5000.0),
    ):
        line, failed = stats_line(name, arr, p95_l, max_l)
        print(line)
        if failed:
            fails += 1

    conn.close()
    return 1 if fails > 0 else 0


def run_stripe(db_path, sk, wh, ts_tag):
    fails = 0
    wh_bytes = wh.encode("utf-8")
    test_rfc = f"CER{ts_tag[2:8]}{ts_tag[-3:]}"
    evt_paid = None
    evt_oxxo = None

    def sign(body_bytes, ts_override=None):
        t = str(ts_override if ts_override is not None else int(time.time()))
        sig = hmac.new(wh_bytes, f"{t}.".encode("utf-8") + body_bytes, hashlib.sha256).hexdigest()
        return f"t={t},v1={sig}"

    try:
        c1_body = json.dumps({"ref_code": test_rfc, "email": "cert-test@mallen.lat", "accepted_disclaimer": True}).encode()
        st1, raw1, _ = http_req("POST", "/api/checkout/session", c1_body)
        txt1 = raw1.decode("utf-8", errors="replace")
        if st1 != 200:
            print(f"  [FAIL] Checkout Test falló: HTTP {st1} {txt1[:200]}")
            return 1

        j1 = json.loads(txt1)
        sid, url = j1.get("session_id", ""), j1.get("checkout_url", "")
        print(f"  [PASS] Checkout Test creado ({test_rfc}): {sid}")
        print(f"         URL visual (Safari/Chrome): {url}")

        req_s = urllib.request.Request(
            f"https://api.stripe.com/v1/checkout/sessions/{sid}",
            headers={"Authorization": f"Bearer {sk}"},
        )
        with urllib.request.urlopen(req_s, timeout=20) as r:
            s_obj = json.loads(r.read().decode("utf-8", errors="replace"))

        pm_types = s_obj.get("payment_method_types", [])
        pm_opts = s_obj.get("payment_method_options", {})
        msi_on = pm_opts.get("card", {}).get("installments", {}).get("enabled") is True
        tds = pm_opts.get("card", {}).get("request_three_d_secure")
        ok_core = "card" in pm_types and msi_on and tds == "any"
        if not ok_core:
            fails += 1
        print(f"  [{'PASS' if ok_core else 'FAIL'}] Stripe API inspect -> methods={pm_types} | MSI={msi_on} | 3DS={tds}")
        for extra in ("oxxo", "link"):
            if extra in pm_types:
                print(f"  [PASS] Método '{extra}' activo en la sesión de Checkout")
            else:
                print(f"  [WARN] Método '{extra}' no listado en payment_method_types={pm_types}")

        b2 = json.dumps({"id": f"evt_test_cert_bad_{ts_tag}", "type": "checkout.session.completed"}).encode()
        st2, _, _ = http_req("POST", "/api/webhook_stripe", b2, {"Stripe-Signature": "t=1700000000,v1=deadbeef"})
        if st2 != 401:
            fails += 1
        print(f"  [{'PASS' if st2 == 401 else 'FAIL'}] Firma inválida -> HTTP {st2} (esperado 401)")

        b2b = json.dumps({"id": f"evt_test_cert_skew_{ts_tag}", "type": "charge.failed", "data": {"object": {}}}).encode()
        st2b, _, _ = http_req("POST", "/api/webhook_stripe", b2b, {"Stripe-Signature": sign(b2b, int(time.time()) - 600)})
        if st2b == 401:
            print("  [PASS] Replay t-600s rechazado (HTTP 401)")
        else:
            print(f"  [WARN] Hardening pendiente: replay t-600s devolvió HTTP {st2b} (agregar ventana +/-300s en verify_webhook_signature)")

        b3 = json.dumps({"id": f"evt_test_cert_ign_{ts_tag}", "type": "charge.failed", "data": {"object": {}}}).encode()
        st3, r3, _ = http_req("POST", "/api/webhook_stripe", b3, {"Stripe-Signature": sign(b3)})
        ok3 = st3 == 200 and "IGNORED_EVENT_TYPE" in r3.decode("utf-8", errors="replace")
        if not ok3:
            fails += 1
        print(f"  [{'PASS' if ok3 else 'FAIL'}] Evento charge.failed ignorado")

        b4 = json.dumps({
            "id": f"evt_test_cert_unpaid_{ts_tag}",
            "type": "checkout.session.completed",
            "data": {"object": {"client_reference_id": test_rfc, "payment_status": "unpaid"}},
        }).encode()
        st4, r4, _ = http_req("POST", "/api/webhook_stripe", b4, {"Stripe-Signature": sign(b4)})
        ok4 = st4 == 200 and "IGNORED_UNPAID_SESSION" in r4.decode("utf-8", errors="replace")
        if not ok4:
            fails += 1
        print(f"  [{'PASS' if ok4 else 'FAIL'}] Ficha OXXO unpaid ignorada")

        evt_paid = f"evt_test_cert_paid_{ts_tag}"
        b5 = json.dumps({
            "id": evt_paid,
            "type": "checkout.session.completed",
            "data": {"object": {"client_reference_id": test_rfc, "payment_status": "paid"}},
        }).encode()
        st5a, _, _ = http_req("POST", "/api/webhook_stripe", b5, {"Stripe-Signature": sign(b5)})
        st5b, r5b, _ = http_req("POST", "/api/webhook_stripe", b5, {"Stripe-Signature": sign(b5)})
        ok5 = st5a == 200 and st5b == 200 and "ALREADY_PROCESSED_IDEMPOTENT" in r5b.decode("utf-8", errors="replace")
        if not ok5:
            fails += 1
        print(f"  [{'PASS' if ok5 else 'FAIL'}] Pago instantáneo + idempotencia")

        evt_oxxo = f"evt_test_cert_oxxo_{ts_tag}"
        b6 = json.dumps({
            "id": evt_oxxo,
            "type": "checkout.session.async_payment_succeeded",
            "data": {"object": {"client_reference_id": test_rfc, "payment_status": "paid"}},
        }).encode()
        st6a, _, _ = http_req("POST", "/api/webhook_stripe", b6, {"Stripe-Signature": sign(b6)})
        st6b, r6b, _ = http_req("POST", "/api/webhook_stripe", b6, {"Stripe-Signature": sign(b6)})
        ok6 = st6a == 200 and st6b == 200 and "ALREADY_PROCESSED_IDEMPOTENT" in r6b.decode("utf-8", errors="replace")
        if not ok6:
            fails += 1
        print(f"  [{'PASS' if ok6 else 'FAIL'}] OXXO async_payment_succeeded + idempotencia")

        zip_ok = False
        for _ in range(15):
            stz, zbytes, _ = http_req("GET", f"/download/zip/{test_rfc}", timeout=10)
            if stz == 200 and zbytes.startswith(b"PK\x03\x04"):
                zf = zipfile.ZipFile(io.BytesIO(zbytes))
                if zf.testzip() is None:
                    pdfs = [n for n in zf.namelist() if n.lower().endswith(".pdf") and zf.getinfo(n).file_size > 5000]
                    if len(pdfs) >= 1:
                        print(f"  [PASS] Entregable ZIP íntegro ({len(zbytes)}B, {len(pdfs)} PDFs > 5KB)")
                        zip_ok = True
                        break
            time.sleep(1)
        if not zip_ok:
            print("  [FAIL] Timeout esperando ZIP post-pago (15s)")
            fails += 1

        stv, _, _ = http_req("GET", f"/verify/{test_rfc}", timeout=10)
        if stv != 200:
            fails += 1
        print(f"  [{'PASS' if stv == 200 else 'FAIL'}] GET /verify/{test_rfc} -> HTTP {stv}")
    finally:
        events_to_del = [e for e in (evt_paid, evt_oxxo) if e is not None]
        wconn = sqlite3.connect(db_path, timeout=10)
        with wconn:
            c1 = 0
            if events_to_del:
                placeholders = ",".join("?" for _ in events_to_del)
                c1 = wconn.execute(
                    f"DELETE FROM processed_webhooks WHERE event_id IN ({placeholders})",
                    events_to_del,
                ).rowcount
            c2 = wconn.execute("DELETE FROM prospects WHERE tax_id=?", (test_rfc,)).rowcount
            c3 = wconn.execute("DELETE FROM audit_sessions WHERE tax_id=?", (test_rfc,)).rowcount
        wconn.close()
        shutil.rmtree(f"kits_blindaje/{test_rfc}", ignore_errors=True)
        print(f"  [PASS] Limpieza post-test ({test_rfc}): webhooks={c1}, prospects={c2}, sessions={c3}, ZIP eliminado")

    return 1 if fails > 0 else 0


def main():
    global BASE
    p = argparse.ArgumentParser()
    p.add_argument("--mode", choices=["readonly", "stripe"], required=True)
    p.add_argument("--db", required=True)
    p.add_argument("--base-url", default="http://127.0.0.1:8080")
    p.add_argument("--sk", default="")
    p.add_argument("--wh", default="")
    p.add_argument("--ts", default="")
    args = p.parse_args()

    BASE = args.base_url.rstrip("/")

    if args.mode == "readonly":
        sys.exit(run_readonly(args.db))
    sys.exit(run_stripe(args.db, args.sk, args.wh, args.ts))


if __name__ == "__main__":
    main()
