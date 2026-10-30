(function () {
    let exitIntentShown = false;

    window.lastAuditData = null;

    window.closeVectorsPopover = function () {
        const pop = document.getElementById("vectors-37-popover");
        if (pop) {
            pop.classList.add("hidden");
            pop.classList.remove("flex");
        }
    };

    window.openVectorsPopover = function () {
        const data = window.lastAuditData;
        if (!data) return;
        const rawList = data.findings || data.vectors || data.hallazgos || [];
        if (!rawList.length) return;

        const pop = document.getElementById("vectors-37-popover");
        const list = document.getElementById("vectors-37-list");
        const title = document.getElementById("popover-company-title");
        if (!pop || !list) return;

        if (title) {
            title.textContent = `${data.razon_social} (${data.tax_id}) • ${rawList.length} Vectores Evaluados`;
        }

        list.innerHTML = rawList.map((f, idx) => {
            const auth = f.authority || f.organo || f.dependencia || `Vector #${idx + 1}`;
            const code = f.code || f.norma || f.precepto || f.titulo || "Precepto Normativo";
            const desc = f.description || f.hallazgo || f.detalle || "";
            const uma = f.uma_range || f.rango_uma || "Tabulador UMA 2026";
            const fineVal = Number(f.estimated_fine_mxn ?? f.multa_mxn ?? f.multa ?? 0);
            const isFine = fineVal > 0;
            const badgeClass = isFine
                ? "bg-red-950/80 text-red-300 border-red-500/40"
                : "bg-emerald-950/80 text-emerald-300 border-emerald-500/40";
            const fineTxt = isFine
                ? "$" + fineVal.toLocaleString("es-MX", { minimumFractionDigits: 2 }) + " MXN"
                : "BLINDADO ($0.00)";
            return `
                <div class="bg-slate-950/90 border border-white/10 rounded-lg p-2.5 flex items-center justify-between gap-3 hover:border-emerald-500/40 transition">
                    <div class="min-w-0 flex-1">
                        <div class="flex items-center gap-1.5 flex-wrap">
                            <span class="text-[9px] font-bold px-1.5 py-0.5 rounded border ${badgeClass}">${auth}</span>
                            <span class="text-[11px] font-bold text-white truncate">${code}</span>
                        </div>
                        <p class="text-[10px] text-slate-400 truncate mt-0.5">${desc}</p>
                    </div>
                    <div class="text-right shrink-0">
                        <span class="text-[11px] font-extrabold ${isFine ? 'text-red-400' : 'text-emerald-400'} block">${fineTxt}</span>
                        <span class="text-[8px] text-slate-500 block">${uma}</span>
                    </div>
                </div>
            `;
        }).join("");

        pop.classList.remove("hidden");
        pop.classList.add("flex");
    };

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

    function setField(id, val, overwrite) {
        const el = document.getElementById(id);
        if (el && val && (overwrite || !el.value.trim())) {
            el.value = val;
        }
    }

    function safeSetText(id, text) {
        const el = document.getElementById(id);
        if (el) el.textContent = text;
    }

    function safeSetHref(id, href) {
        const el = document.getElementById(id);
        if (el) el.href = href;
    }

    window.runLiveCalibrationScan = async function (event) {
        if (event) event.preventDefault();
        const rawInput = document.getElementById("folioInput")?.value?.trim() || "";
        const domain = document.getElementById("domainInput")?.value?.trim() || "";
        const razonManual = document.getElementById("dos_razon")?.value?.trim() || "";

        if (!rawInput && !domain && !razonManual) return;

        // Si el usuario escribió un nombre comercial (ej. "Pizzeria Cancino") en el primer campo y dejó vacío "Razón Social", lo usamos también como pista de nombre
        const effectiveTaxOrName = (rawInput || razonManual || domain).toUpperCase();
        const effectiveRazonOverride = razonManual ? razonManual : (rawInput.includes(" ") || rawInput.length < 12 ? rawInput : null);
        currentTaxId = effectiveTaxOrName;

        const btn = document.getElementById("btnRunScan");
        const origText = btn ? btn.textContent : "Auditar Exposición Legal";
        if (btn) {
            btn.textContent = "Auditando Padrones Oficiales y Sitio Web...";
            btn.disabled = true;
        }

        const valOrNull = (id) => {
            const v = document.getElementById(id)?.value?.trim();
            return v ? v : null;
        };

        const payload = {
            tax_id: effectiveTaxOrName,
            domain: domain ? domain : null,
            razon_social_override: effectiveRazonOverride,
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
                    razon_social_oficial: effectiveRazonOverride,
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
            if (!resp.ok) throw new Error("HTTP " + resp.status);
            const data = await resp.json();
            window.lastAuditData = data;

            currentTaxId = data.tax_id || effectiveTaxOrName;

            // Actualizar los campos de la Cédula con los datos reales encontrados en DENUE / OSINT
            setField("dos_razon", data.razon_social, true);
            setField("dos_giro", `${data.codigo_scian} - ${data.actividad_scian_desc}`, true);
            setField("dos_domicilio", data.domicilio_fiscal_o_denue, true);
            if (data.domicilio_fiscal_o_denue && data.domicilio_fiscal_o_denue.includes(",")) {
                const parts = data.domicilio_fiscal_o_denue.split(",");
                setField("dos_estado", parts[parts.length - 1].trim(), true);
            }
            if (data.calibration_applied && data.calibration_applied.dossier) {
                const d = data.calibration_applied.dossier;
                setField("dos_email", d.email_arco_privacidad, false);
                setField("dos_tel", d.telefono_corporativo, false);
            }

            if (typeof window.triggerPhotonicBurst === "function") {
                window.triggerPhotonicBurst();
            }

            const box = document.getElementById("live-scan-result");
            if (box) {
                box.classList.remove("hidden");
                safeSetText("res-razon", `${data.razon_social} (${data.tax_id})`);

                const setBadge = (id, text, isAlert, isWarn) => {
                    const el = document.getElementById(id);
                    if (!el) return;
                    el.textContent = text;
                    el.className = "text-[11px] font-bold block mt-0.5 " + (isAlert ? "text-red-400" : isWarn ? "text-amber-400" : "text-emerald-400");
                };
                const s69b = (data.sat_69b_status || "LIMPIO").toUpperCase();
                setBadge("blk-69b", s69b.includes("LIMPIO") ? "SIN ANTECEDENTES 69-B" : `ALERTA: ${s69b}`, !s69b.includes("LIMPIO"), false);

                const s69 = (data.sat_69_status || "LIMPIO").toUpperCase();
                setBadge("blk-69", s69.includes("LIMPIO") ? "LOCALIZADO / SIN CRÉDITOS" : `ALERTA: ${s69}`, !s69.includes("LIMPIO"), false);

                setBadge("blk-repse", data.repse_registered ? "PADRÓN REPSE ACTIVO" : "VALIDAR ICSOE / SISSUB", false, !data.repse_registered);
                setBadge("blk-sfp", data.sfp_sanctioned ? "INHABILITADO EN SFP" : "SIN SANCIONES SFP", !!data.sfp_sanctioned, false);

                const profecoOk = data.web_telemetry && data.web_telemetry.has_profeco_rrpac_mention;
                setBadge("blk-profeco", profecoOk ? "CONTRATO RRPAC VISIBLE" : "REVISAR ART. 76 BIS", false, !profecoOk);

                const isImmune = Number(data.total_estimated_fine_mxn || 0) === 0;
                setBadge("blk-laboral", isImmune ? "BLINDAJE PATRONAL 100%" : `${(data.findings || []).length} BRECHAS DETECTADAS`, !isImmune, false);

                safeSetText("res-score", `${data.risk_score}/100 PUNTOS`);
                safeSetText(
                    "res-fine",
                    "$" + Number(data.total_estimated_fine_mxn || 0).toLocaleString("es-MX", { minimumFractionDigits: 2, maximumFractionDigits: 2 }) + " MXN"
                );
                safeSetText(
                    "res-findings-count",
                    `Giro SCIAN ${data.codigo_scian} • Ver los ${(data.findings || []).length} Vectores Evaluados (Clic / Pasar Cursor)`
                );
                safeSetText("res-hash", data.audit_sha256_hash);

                const encodedId = encodeURIComponent(data.tax_id);
                safeSetHref("btn-go-verify", `/verify/${encodedId}`);
                safeSetHref("btn-go-micro", `/api/micro_scan/${encodedId}`);
                safeSetHref("btn-go-checkout", `/checkout?ref=${encodedId}`);

                box.scrollIntoView({ behavior: "smooth", block: "nearest" });
            }
        } catch (err) {
            console.error("Error en escaneo:", err);
            window.location.href = `/verify/${encodeURIComponent(effectiveTaxOrName)}`;
        } finally {
            if (btn) {
                btn.textContent = origText;
                btn.disabled = false;
            }
        }
    };
})();
