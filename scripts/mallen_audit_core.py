#!/usr/bin/env python3
import argparse
import concurrent.futures
import contextlib
import hashlib
import hmac
import http.server
import io
import json
import math
import os
import re
import shutil
import sqlite3
import subprocess
import sys
import threading
import time
import unicodedata
import urllib.error
import urllib.parse
import urllib.request
import zipfile

BASE = "http://127.0.0.1:8080"


def pct(arr, p):
    if not arr:
        return 0.0
    s = sorted(arr)
    idx = min(len(s) - 1, max(0, math.ceil((p / 100.0) * len(s)) - 1))
    return s[idx]


def strip_accents(text):
    nfkd = unicodedata.normalize("NFKD", text or "")
    return "".join(c for c in nfkd if not unicodedata.combining(c))


def sanitize_fts(q):
    cleaned = re.sub(r"[^A-Za-z0-9ÁÉÍÓÚÑáéíóúñ ]+", " ", q).strip()
    tokens = [t for t in cleaned.split() if len(t) >= 2][:4]
    if not tokens:
        return '"SA"'
    return " ".join(f'"{t}"' for t in tokens)


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


def extract_pdf_text(pdf_bytes):
    if not shutil.which("pdftotext"):
        return None
    try:
        proc = subprocess.run(
            ["pdftotext", "-", "-"],
            input=pdf_bytes,
            capture_output=True,
            timeout=10,
            check=False,
        )
        if proc.returncode == 0:
            return proc.stdout.decode("utf-8", errors="replace")
    except Exception:
        pass
    return None


def check_cents_parity(j_obj, total_fine):
    findings = j_obj.get("findings")
    if not isinstance(findings, list) or not findings or total_fine is None:
        return None, 0
    sum_cents = 0
    counted = 0
    for item in findings:
        if not isinstance(item, dict):
            continue
        val = item.get("estimated_fine_mxn", item.get("multa_estimada_mxn"))
        if isinstance(val, (int, float)):
            sum_cents += round(float(val) * 100)
            counted += 1
    if counted == 0:
        return None, 0
    total_cents = round(float(total_fine) * 100)
    delta = abs(sum_cents - total_cents)
    return delta == 0, delta


def run_readonly(db_path):
    fails = 0
    db_uri = f"file:{urllib.parse.quote(os.path.abspath(db_path))}?mode=ro"

    with contextlib.closing(sqlite3.connect(db_uri, uri=True)) as conn:
        print("\n=== PILAR 1: FORM-BOT POR SCIAN, HOMONIMIA, ENTES PÚBLICOS Y LOOKUP ===")
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
            if not rows:
                print(f"  [FAIL] Homónimo '{h:<26}' -> 0 hits en FTS5 (query={q})")
                fails += 1
                continue
            top_norm = strip_accents(rows[0][0]).upper()
            raw_tokens = [strip_accents(t).upper() for t in re.findall(r"[A-Za-z0-9ÁÉÍÓÚÑáéíóúñ]+", h) if len(t) >= 3]
            token_ok = any(tok in top_norm for tok in raw_tokens) if raw_tokens else True
            if not token_ok:
                fails += 1
            print(
                f"  [{'PASS' if token_ok else 'FAIL'}] Homónimo '{h:<26}' -> hits={len(rows)} | "
                f"top='{rows[0][0][:32]}' | token_ok={token_ok} | scian={rows[0][1]} | estrato={rows[0][2]}"
            )

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

        for probe in ("EST850628K51", "AAG090703QT6", "STEREN"):
            stl, _, msl = http_req("GET", f"/api/lookup?q={urllib.parse.quote(probe)}")
            if stl != 200:
                fails += 1
            print(f"  [{'PASS' if stl == 200 else 'FAIL'}] GET /api/lookup?q={probe:<12} -> HTTP {stl} ({msl:.1f}ms)")

        print("\n=== PILAR 2: SINCRONÍA SSOT (SCAN <-> SQLITE <-> PDFTEXT <-> CENTAVOS <-> CHECKOUT) ===")
        if not shutil.which("pdftotext"):
            print("  [WARN] pdftotext no disponible en PATH; se omitirá validación de texto interno del PDF")

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
            last_json = None
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
                last_json = j
                snapshots.append((
                    j.get("risk_score"),
                    j.get("total_estimated_fine_mxn", j.get("exact_vemp_fine_mxn")),
                    j.get("estimated_employees"),
                    j.get("codigo_scian"),
                ))

            if not scan_ok or len(snapshots) < 3 or last_json is None:
                print(f"  [FAIL] SSOT {label} ({rfc}): error o JSON inválido en /api/scan")
                fails += 1
                continue

            det_ok = len(set(snapshots)) == 1
            r1, f1, e1, sc1 = snapshots[0]

            db_row = conn.execute(
                "SELECT master_json FROM audit_sessions WHERE UPPER(tax_id)=UPPER(?) ORDER BY rowid DESC LIMIT 1",
                (rfc,),
            ).fetchone()
            db_parity = False
            if db_row and db_row[0]:
                try:
                    dj = json.loads(db_row[0])
                    db_fine = dj.get("total_estimated_fine_mxn", dj.get("exact_vemp_fine_mxn"))
                    db_scian = dj.get("codigo_scian", dj.get("scian"))
                    fine_eq = (
                        db_fine is None
                        or f1 is None
                        or abs(float(db_fine) - float(f1)) < 0.01
                    )
                    scian_eq = (db_scian is None or sc1 is None or str(db_scian) == str(sc1))
                    db_parity = (
                        dj.get("risk_score") == r1
                        and dj.get("estimated_employees") == e1
                        and fine_eq
                        and scian_eq
                    )
                except Exception:
                    db_parity = False

            cents_ok, cents_delta = check_cents_parity(last_json, f1)
            cents_str = "n/a" if cents_ok is None else f"delta={cents_delta}c"

            stp, bp, msp = http_req("GET", f"/api/micro_scan/{rfc}")
            pdf_lat.append(msp)
            pdf_bin_ok = stp == 200 and bp.startswith(b"%PDF-") and len(bp) > 5000
            pdf_txt = extract_pdf_text(bp) if pdf_bin_ok else None
            pdf_txt_ok = True
            if pdf_txt is not None:
                fine_fmt_2 = f"{float(f1):,.2f}" if f1 is not None else ""
                fine_fmt_0 = f"{int(float(f1)):,}" if f1 is not None else ""
                pdf_txt_ok = (rfc in pdf_txt) and (fine_fmt_2 in pdf_txt or fine_fmt_0 in pdf_txt)

            stc, _, _ = http_req("GET", f"/checkout/{rfc}")
            chk_ok = stc == 200

            ok = det_ok and db_parity and pdf_bin_ok and pdf_txt_ok and chk_ok and (cents_ok is not False)
            if not ok:
                fails += 1
            print(
                f"  [{'PASS' if ok else 'FAIL'}] {label:<14} {rfc:<13} | det={det_ok} | db_eq={db_parity} | "
                f"cents={cents_str} | pdf_txt={pdf_txt_ok} | risk={r1} | fine=${f1} | chk={stc}"
            )

        print("\n=== PILAR 4: CONCURRENCIA MULTI-RFC (25 REQS / 15 HILOS WAL) + INPUTS HOSTILES + SSRF CANARIO ===")
        burst_rfcs = [r for _, r in ssot_cases]

        def burst_worker(idx):
            target_rfc = burst_rfcs[idx % len(burst_rfcs)]
            if idx % 2 == 0:
                st, _, ms = http_req("POST", "/api/scan", {"tax_id": target_rfc})
            else:
                st, _, ms = http_req("GET", f"/api/micro_scan/{target_rfc}")
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
            f"  [{'PASS' if burst_ok else 'FAIL'}] Ráfaga 25 reqs multi-RFC (15 hilos) -> 5xx={burst_5xx} | "
            f"p50={pct(burst_ms, 50):.1f}ms | p95={burst_p95:.1f}ms (umbral <=5000ms) | max={max(burst_ms):.1f}ms"
        )

        canary_srv, canary_port, canary_hits = start_ssrf_canary()
        try:
            hostile_payloads = [
                ("RFC con &", {"tax_id": "&DP0504056D9"}, False),
                ("FTS5 roto", {"tax_id": '" * OR NOT ('}, False),
                ("SQLi básico", {"tax_id": "' OR 1=1 --"}, False),
                ("SSRF 127.0.0.1", {"tax_id": "EST850628K51", "domain": f"http://127.0.0.1:{canary_port}/ssrf-canary"}, True),
                ("SSRF localhost", {"tax_id": "EST850628K51", "domain": f"http://localhost:{canary_port}/ssrf-canary"}, True),
                ("SSRF IPv6 ::1", {"tax_id": "EST850628K51", "domain": f"http://[::1]:{canary_port}/ssrf-canary"}, True),
                ("SSRF AWS meta", {"tax_id": "EST850628K51", "domain": "http://169.254.169.254/latest/meta-data/"}, True),
            ]
            for name, pl, is_ssrf in hostile_payloads:
                hits_before = len(canary_hits)
                st, body, ms = http_req("POST", "/api/scan", pl)
                txt = body.decode("utf-8", errors="replace")
                txt_l = txt.lower()
                hit_delta = len(canary_hits) - hits_before
                sql_leak = any(tok in txt_l for tok in ("sqlite_error", "sql syntax", "no such column", "panicked at"))
                ssrf_leak = is_ssrf and (
                    hit_delta > 0
                    or any(tok in txt for tok in ("CANARY_SSRF_HIT", "ami-id", "instance-id", "iam/security-credentials"))
                )
                leaked = sql_leak or ssrf_leak
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

    return 1 if fails > 0 else 0


def run_stripe(db_path, project_dir, ts_tag):
    fails = 0
    sk = os.environ.get("AUDIT_STRIPE_SK", "").strip()
    wh = os.environ.get("AUDIT_STRIPE_WH", "").strip()
    if not sk.startswith("sk_test_") or not wh.startswith("whsec_"):
        print("  [FAIL] AUDIT_STRIPE_SK (sk_test_*) o AUDIT_STRIPE_WH (whsec_*) ausentes o inválidos en entorno")
        return 1

    if not ts_tag or len(ts_tag) < 11:
        ts_tag = time.strftime("%Y%m%d_%H%M%S")

    wh_bytes = wh.encode("utf-8")
    test_rfc = f"CER{ts_tag[2:8]}{ts_tag[-3:]}"
    kit_dir = os.path.join(os.path.abspath(project_dir), "kits_blindaje", test_rfc)
    evt_paid = None
    evt_oxxo = None

    def sign(body_bytes, ts_override=None):
        t = str(ts_override if ts_override is not None else int(time.time()))
        sig = hmac.new(wh_bytes, f"{t}.".encode("utf-8") + body_bytes, hashlib.sha256).hexdigest()
        return f"t={t},v1={sig}"

    try:
        st0, _, _ = http_req("POST", "/api/scan", {"tax_id": test_rfc})
        if st0 != 200:
            print(f"  [FAIL] Pre-scan falló para {test_rfc}: HTTP {st0}")
            return 1

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
        if st2b != 401:
            fails += 1
        print(f"  [{'PASS' if st2b == 401 else 'FAIL'}] Replay t-600s rechazado -> HTTP {st2b} (esperado 401)")

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
        for _ in range(20):
            stz, zbytes, _ = http_req("GET", f"/download/zip/{test_rfc}", timeout=10)
            if stz == 200 and zbytes.startswith(b"PK\x03\x04"):
                zf = zipfile.ZipFile(io.BytesIO(zbytes))
                if zf.testzip() is None:
                    names = zf.namelist()
                    pdfs = [n for n in names if n.lower().endswith(".pdf") and zf.getinfo(n).file_size > 5000]
                    leaked_typ = [n for n in names if n.lower().endswith(".typ")]
                    if len(pdfs) >= 1 and len(leaked_typ) == 0:
                        print(f"  [PASS] Entregable ZIP íntegro ({len(zbytes)}B, {len(pdfs)} PDFs > 5KB, 0 .typ filtrados)")
                        zip_ok = True
                        break
            time.sleep(1)
        if not zip_ok:
            print("  [FAIL] Timeout o integridad fallida esperando ZIP post-pago (20s)")
            fails += 1

        stv, _, _ = http_req("GET", f"/verify/{test_rfc}", timeout=10)
        if stv != 200:
            fails += 1
        print(f"  [{'PASS' if stv == 200 else 'FAIL'}] GET /verify/{test_rfc} -> HTTP {stv}")
    finally:
        events_to_del = [e for e in (evt_paid, evt_oxxo) if e is not None]
        with contextlib.closing(sqlite3.connect(db_path, timeout=15)) as wconn:
            snap_col = None
            snap_cols = [r[1] for r in wconn.execute("PRAGMA table_info(audit_snapshots)").fetchall()]
            for cand in ("tax_id_key", "tax_id"):
                if cand in snap_cols:
                    snap_col = cand
                    break

            with wconn:
                c1 = 0
                if events_to_del:
                    ph = ",".join("?" for _ in events_to_del)
                    c1 = wconn.execute(f"DELETE FROM processed_webhooks WHERE event_id IN ({ph})", events_to_del).rowcount
                c2 = wconn.execute("DELETE FROM prospects WHERE UPPER(tax_id)=UPPER(?)", (test_rfc,)).rowcount
                c3 = wconn.execute("DELETE FROM audit_sessions WHERE UPPER(tax_id)=UPPER(?)", (test_rfc,)).rowcount
                c4 = 0
                if snap_col:
                    c4 = wconn.execute(f"DELETE FROM audit_snapshots WHERE UPPER({snap_col})=UPPER(?)", (test_rfc,)).rowcount

            shutil.rmtree(kit_dir, ignore_errors=True)

            rem_wh = 0
            if events_to_del:
                ph = ",".join("?" for _ in events_to_del)
                rem_wh = wconn.execute(f"SELECT COUNT(*) FROM processed_webhooks WHERE event_id IN ({ph})", events_to_del).fetchone()[0]
            rem_pr = wconn.execute("SELECT COUNT(*) FROM prospects WHERE UPPER(tax_id)=UPPER(?)", (test_rfc,)).fetchone()[0]
            rem_se = wconn.execute("SELECT COUNT(*) FROM audit_sessions WHERE UPPER(tax_id)=UPPER(?)", (test_rfc,)).fetchone()[0]
            rem_sn = 0
            if snap_col:
                rem_sn = wconn.execute(f"SELECT COUNT(*) FROM audit_snapshots WHERE UPPER({snap_col})=UPPER(?)", (test_rfc,)).fetchone()[0]

        zip_exists = os.path.exists(kit_dir)
        clean_ok = (rem_wh == 0 and rem_pr == 0 and rem_se == 0 and rem_sn == 0 and not zip_exists)
        if not clean_ok:
            fails += 1
        print(
            f"  [{'PASS' if clean_ok else 'FAIL'}] Limpieza verificada ({test_rfc}): "
            f"borrados=(wh={c1},pr={c2},se={c3},sn={c4}) | residuales=(wh={rem_wh},pr={rem_pr},se={rem_se},sn={rem_sn},dir={zip_exists})"
        )

    return 1 if fails > 0 else 0


def main():
    global BASE
    p = argparse.ArgumentParser()
    p.add_argument("--mode", choices=["readonly", "stripe"], required=True)
    p.add_argument("--db", required=True)
    p.add_argument("--base-url", default="http://127.0.0.1:8080")
    p.add_argument("--project-dir", default=".")
    p.add_argument("--ts", default="")
    args = p.parse_args()

    BASE = args.base_url.rstrip("/")

    if args.mode == "readonly":
        sys.exit(run_readonly(args.db))
    sys.exit(run_stripe(args.db, args.project_dir, args.ts))


if __name__ == "__main__":
    main()
