(function () {
    let exitIntentShown = false;
    let currentTaxId = "EST850628K51";

    // Disparador Anti-Abandono (Exit-Intent Popup) cuando el mouse sale por la parte superior de la ventana
    document.addEventListener("mouseleave", (e) => {
        if (e.clientY <= 0 && !exitIntentShown) {
            const modal = document.getElementById("exit-intent-modal");
            if (modal) {
                exitIntentShown = true;
                const inputVal = document.getElementById("folioInput")?.value?.trim();
                if (inputVal) currentTaxId = inputVal.toUpperCase();
                const link = document.getElementById("exit-micro-pdf-link");
                const label = document.getElementById("exit-rfc-label");
                if (label) label.textContent = currentTaxId;
                if (link) link.href = `/api/micro_scan/${encodeURIComponent(currentTaxId)}`;
                modal.classList.remove("hidden");
                modal.classList.add("flex");
            }
        }
    });

    window.closeExitModal = function () {
        const modal = document.getElementById("exit-intent-modal");
        if (modal) {
            modal.classList.add("hidden");
            modal.classList.remove("flex");
        }
    };

    window.runLiveCalibrationScan = async function (event) {
        event.preventDefault();
        const rfc = document.getElementById("folioInput").value.trim().toUpperCase();
        const domain = document.getElementById("domainInput")?.value?.trim() || "";
        if (!rfc) return;
        currentTaxId = rfc;

        const btn = document.getElementById("btnRunScan");
        const origText = btn.textContent;
        btn.textContent = "Auditando 39 NOMs + SAT + OSINT...";
        btn.disabled = true;

        const payload = {
            tax_id: rfc,
            domain: domain ? domain : null,
            razon_social_override: null,
            email_contacto: null,
            calibration: {
                exact_employee_count: parseInt(document.getElementById("cal_emp")?.value || "25", 10),
                has_telework_over_40_percent: document.getElementById("cal_nom037")?.checked ?? true,
                uses_subcontracted_services_repse: document.getElementById("cal_repse")?.checked ?? true,
                has_pressure_vessels_boilers: document.getElementById("cal_nom020")?.checked ?? false,
                has_forklifts_or_manual_loads: document.getElementById("cal_nom006")?.checked ?? true,
                handles_hazardous_chemicals: document.getElementById("cal_nom005")?.checked ?? false,
                uses_biometrics_or_health_data: document.getElementById("cal_bio")?.checked ?? true,
                transfers_data_to_third_parties: document.getElementById("cal_transfer")?.checked ?? true,
                pays_intangible_b2b_services: document.getElementById("cal_69b")?.checked ?? true,
                transports_goods_federal_highways: document.getElementById("cal_cartaporte")?.checked ?? false,
                performs_vulnerable_activities_pld: document.getElementById("cal_pld")?.checked ?? false
            }
        };

        try {
            const resp = await fetch("/api/scan", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify(payload)
            });
            const data = await resp.json();

            const box = document.getElementById("live-scan-result");
            if (box) {
                box.classList.remove("hidden");
                document.getElementById("res-razon").textContent = `${data.razon_social} (${data.tax_id})`;
                document.getElementById("res-score").textContent = `${data.risk_score}/100 PUNTOS`;
                document.getElementById("res-fine").textContent =
                    "$" + Number(data.total_estimated_fine_mxn || 1485600).toLocaleString("es-MX", { minimumFractionDigits: 2 }) + " MXN";
                document.getElementById("res-findings-count").textContent =
                    `${(data.findings || []).length} vulnerabilidades normativas detectadas (STPS/SAT/SABG/IMSS/PROFECO)`;
                document.getElementById("res-hash").textContent = data.audit_sha256_hash;
                document.getElementById("btn-go-verify").href = `/verify/${encodeURIComponent(data.tax_id)}`;
                document.getElementById("btn-go-micro").href = `/api/micro_scan/${encodeURIComponent(data.tax_id)}`;
                document.getElementById("btn-go-checkout").href = `/checkout?ref=${encodeURIComponent(data.tax_id)}`;
            }
        } catch (_) {
            window.location.href = `/verify/${encodeURIComponent(rfc)}`;
        } finally {
            btn.textContent = origText;
            btn.disabled = false;
        }
    };
})();
