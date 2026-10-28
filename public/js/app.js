(function () {
    let exitIntentShown = false;
    let currentTaxId = "EST850628K51";

    function animateModalExplosion(modalEl) {
        if (!modalEl) return;
        const card = modalEl.querySelector('.photonic-modal-card');
        if (card) {
            card.classList.remove('is-exploding-in');
            void card.offsetWidth;
            card.classList.add('is-exploding-in');
        }
        if (typeof window.triggerPhotonicBurst === 'function') {
            window.triggerPhotonicBurst();
        }
    }

    // 1. Disparador del Modal "Espera: No te vayas a ciegas" con explosión Roja -> Verde
    document.addEventListener("mouseleave", (e) => {
        if (e.clientY <= 0 && !exitIntentShown) {
            const consentModal = document.getElementById("legal-consent-modal");
            if (consentModal && !consentModal.classList.contains("hidden")) return;

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
                animateModalExplosion(modal);
            }
        }
    });

    window.closeExitModal = function () {
        const modal = document.getElementById("exit-intent-modal");
        if (modal) {
            if (typeof window.triggerPhotonicBurst === "function") {
                window.triggerPhotonicBurst();
            }
            modal.classList.add("hidden");
            modal.classList.remove("flex");
        }
    };

    // 2. Modal Translúcido de Consentimiento Legal (Privacidad LFPDPPP, Términos Art. 89 C.Com y Cookies Técnicas)
    window.openLegalConsentModal = function () {
        const modal = document.getElementById("legal-consent-modal");
        if (modal) {
            modal.classList.remove("hidden");
            modal.classList.add("flex");
            animateModalExplosion(modal);
        }
    };

    window.acceptLegalConsentModal = function () {
        try {
            localStorage.setItem(
                "mallen_legal_consent_2026",
                JSON.stringify({
                    accepted_at_utc: new Date().toISOString(),
                    lfpdppp_privacy: true,
                    ccom_art89_terms: true,
                    session_integrity_cookies: true
                })
            );
        } catch (_) {}

        const modal = document.getElementById("legal-consent-modal");
        if (modal) {
            if (typeof window.triggerPhotonicBurst === "function") {
                window.triggerPhotonicBurst();
            }
            modal.classList.add("hidden");
            modal.classList.remove("flex");
        }
    };

    document.addEventListener("DOMContentLoaded", () => {
        try {
            if (!localStorage.getItem("mallen_legal_consent_2026")) {
                setTimeout(() => {
                    window.openLegalConsentModal();
                }, 650);
            }
        } catch (_) {}
    });

    function setIfEmpty(id, val) {
        const el = document.getElementById(id);
        if (el && !el.value.trim() && val) {
            el.value = val;
        }
    }

    window.runLiveCalibrationScan = async function (event) {
        if (event) event.preventDefault();
        const rfc = document.getElementById("folioInput").value.trim().toUpperCase();
        const domain = document.getElementById("domainInput")?.value?.trim() || "";
        const razonOverride = document.getElementById("dos_razon")?.value?.trim() || null;
        if (!rfc) return;
        currentTaxId = rfc;

        const btn = document.getElementById("btnRunScan");
        const origText = btn.textContent;
        btn.textContent = "Auditando y Sincronizando Datos Registrales...";
        btn.disabled = true;

        const valOrNull = (id) => {
            const v = document.getElementById(id)?.value?.trim();
            return v ? v : null;
        };

        const payload = {
            tax_id: rfc,
            domain: domain ? domain : null,
            razon_social_override: razonOverride,
            email_contacto: valOrNull("dos_email"),
            calibration: {
                exact_employee_count: parseInt(document.getElementById("cal_emp")?.value || "45", 10),
                has_telework_over_40_percent: document.getElementById("cal_nom037")?.checked ?? true,
                uses_subcontracted_services_repse: document.getElementById("cal_repse")?.checked ?? true,
                has_pressure_vessels_boilers: document.getElementById("cal_nom020")?.checked ?? false,
                has_forklifts_or_manual_loads: document.getElementById("cal_nom006")?.checked ?? true,
                handles_hazardous_chemicals: document.getElementById("cal_nom005")?.checked ?? false,
                uses_biometrics_or_health_data: document.getElementById("cal_bio")?.checked ?? true,
                transfers_data_to_third_parties: document.getElementById("cal_transfer")?.checked ?? true,
                pays_intangible_b2b_services: document.getElementById("cal_69b")?.checked ?? true,
                transports_goods_federal_highways: document.getElementById("cal_cartaporte")?.checked ?? false,
                performs_vulnerable_activities_pld: document.getElementById("cal_pld")?.checked ?? false,
                dossier: {
                    razon_social_oficial: razonOverride,
                    domicilio_fiscal_completo: valOrNull("dos_domicilio"),
                    codigo_postal_fiscal: valOrNull("dos_cp"),
                    entidad_jurisdiccion: valOrNull("dos_estado"),
                    giro_scian_descripcion: valOrNull("dos_giro"),
                    representante_legal_nombre: valOrNull("dos_rep_nombre"),
                    representante_legal_cargo: valOrNull("dos_rep_cargo"),
                    oficial_privacidad_nombre: valOrNull("dos_oficial_priv"),
                    email_arco_privacidad: valOrNull("dos_email"),
                    telefono_corporativo: valOrNull("dos_tel"),
                    registro_patronal_imss: valOrNull("dos_reg_patronal"),
                    comision_coordinador_patronal: valOrNull("dos_csh_coord"),
                    comision_secretario_trabajadores: valOrNull("dos_csh_sec"),
                    horario_jornada_laboral: valOrNull("dos_horario"),
                    monto_apoyo_teletrabajo_mxn: valOrNull("dos_apoyo_tele"),
                    proveedor_principal_razon_social: valOrNull("dos_prov_nombre"),
                    proveedor_principal_rfc: valOrNull("dos_prov_rfc"),
                    proveedor_principal_servicio_objeto: valOrNull("dos_prov_obj")
                }
            }
        };

        try {
            const resp = await fetch("/api/scan", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify(payload)
            });
            const data = await resp.json();

            setIfEmpty("dos_razon", data.razon_social);
            setIfEmpty("dos_giro", `${data.codigo_scian} - ${data.actividad_scian_desc}`);
            setIfEmpty("dos_domicilio", data.domicilio_fiscal_o_denue);
            if (data.domicilio_fiscal_o_denue && data.domicilio_fiscal_o_denue.includes(",")) {
                const parts = data.domicilio_fiscal_o_denue.split(",");
                setIfEmpty("dos_estado", parts[parts.length - 1].trim());
            }
            if (data.calibration_applied && data.calibration_applied.dossier) {
                const d = data.calibration_applied.dossier;
                setIfEmpty("dos_email", d.email_arco_privacidad);
                setIfEmpty("dos_tel", d.telefono_corporativo);
            }

            if (typeof window.triggerPhotonicBurst === "function") {
                window.triggerPhotonicBurst();
            }

            const box = document.getElementById("live-scan-result");
            if (box) {
                box.classList.remove("hidden");
                document.getElementById("res-razon").textContent = `${data.razon_social} (${data.tax_id})`;
                document.getElementById("res-score").textContent = `${data.risk_score}/100 PUNTOS`;
                document.getElementById("res-fine").textContent =
                    "$" + Number(data.total_estimated_fine_mxn || 0).toLocaleString("es-MX", { minimumFractionDigits: 2, maximumFractionDigits: 2 }) + " MXN";
                document.getElementById("res-findings-count").textContent =
                    `${(data.findings || []).length} vectores normativos evaluados — Datos registrales pre-cargados en su Cédula`;
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
