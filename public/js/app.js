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
        const dosRazonEl = document.getElementById("dos_razon");
        if (dosRazonEl && dosRazonEl.dataset.autoFilled === "true" && rawInput) {
            dosRazonEl.value = "";
            dosRazonEl.dataset.autoFilled = "false";
        }
        const dosGiroEl = document.getElementById("dos_giro");
        if (dosGiroEl && dosGiroEl.dataset.autoFilled === "true" && rawInput) {
            dosGiroEl.value = "";
        }
        const dosDomEl = document.getElementById("dos_domicilio");
        if (dosDomEl && dosDomEl.dataset.autoFilled === "true" && rawInput) {
            dosDomEl.value = "";
        }
        const razonManual = dosRazonEl?.value?.trim() || "";

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
                exact_employee_count: (function() { const v = document.getElementById("cal_emp")?.value?.trim(); const n = v ? parseInt(v, 10) : NaN; return (!isNaN(n) && n > 0) ? n : null; })(),
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
            const rEl = document.getElementById("dos_razon");
            if (rEl) rEl.dataset.autoFilled = "true";
            const gEl = document.getElementById("dos_giro");
            if (gEl) gEl.dataset.autoFilled = "true";
            const dEl = document.getElementById("dos_domicilio");
            if (dEl) dEl.dataset.autoFilled = "true";
            setField("dos_razon", data.razon_social, true);
            setField("dos_giro", `${data.codigo_scian} - ${data.actividad_scian_desc}`, true);
            setField("dos_domicilio", data.domicilio_fiscal_o_denue, true);
            if (data.estimated_employees && data.estimated_employees > 0) {
                const calEmpEl = document.getElementById("cal_emp");
                if (calEmpEl) calEmpEl.value = data.estimated_employees;
            }
            window.__MALLEN_OFFICIAL_VEMP_BASE = Number(data.total_estimated_fine_mxn || 0);
            window.__MALLEN_OFFICIAL_EMP_BASE = Number(data.estimated_employees || 0);
            window.__MALLEN_OFFICIAL_SCORE_BASE = Number(data.risk_score || 0);
            if (typeof window.applyUnifiedSsotToAllPanels === "function") {
                window.applyUnifiedSsotToAllPanels(
                    data.tax_id || effectiveTaxOrName,
                    data.razon_social,
                    data.codigo_scian,
                    data.actividad_scian_desc,
                    data.domicilio_fiscal_o_denue,
                    (data.calibration_applied && data.calibration_applied.dossier && data.calibration_applied.dossier.entidad_jurisdiccion) || "",
                    Number(data.estimated_employees || 0),
                    Number(data.total_estimated_fine_mxn || 0),
                    Number(data.risk_score || 0)
                );
            }
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


document.addEventListener("DOMContentLoaded", () => {
    const rInput = document.getElementById("dos_razon");
    if (rInput) {
        rInput.addEventListener("input", () => { rInput.dataset.autoFilled = "false"; });
    }
});

window.acceptLegalConsent = function() { if (typeof window.saveMexicoCookieConsentUnified === 'function') { window.saveMexicoCookieConsentUnified('full_mx'); } };


// Seguimiento de iluminación especular a 60 FPS (Awwwards / FWA Sensory Richness)
(() => {
  let rafScheduled = false;
  document.addEventListener('pointermove', (ev) => {
    if (rafScheduled) return;
    rafScheduled = true;
    requestAnimationFrame(() => {
      const cards = document.querySelectorAll('.mallen-glass-card, .quantum-card, .mallen-surface-card');
      for (let i = 0; i < cards.length; i++) {
        const rect = cards[i].getBoundingClientRect();
        const x = ev.clientX - rect.left;
        const y = ev.clientY - rect.top;
        if (x >= -100 && x <= rect.width + 100 && y >= -100 && y <= rect.height + 100) {
          cards[i].style.setProperty('--mouse-x', `${x}px`);
          cards[i].style.setProperty('--mouse-y', `${y}px`);
        }
      }
      rafScheduled = false;
    });
  }, { passive: true });
})();

// Atajo de teclado Cmd/Ctrl + K para enfocar el buscador nacional (Heurística #7 de Nielsen)
document.addEventListener('keydown', (ev) => {
  if ((ev.metaKey || ev.ctrlKey) && ev.key.toLowerCase() === 'k') {
    const input = document.getElementById('folioInput');
    if (input) {
      ev.preventDefault();
      input.focus();
      input.select();
    }
  }
});

// Simulación instantánea en 1 clic sin pre-llenar estáticamente el HTML
window.runQuickSampleScan = function(taxId, domain) {
  const folioInput = document.getElementById('folioInput');
  const domainInput = document.getElementById('domainInput');
  if (folioInput) folioInput.value = taxId;
  if (domainInput) domainInput.value = domain || '';
  const triggerBtn = document.getElementById('btnExecuteScan') || document.querySelector('[data-action="scan"]') || document.querySelector('#express-check button[type="submit"]');
  if (triggerBtn) {
    triggerBtn.click();
  } else if (folioInput && folioInput.form) {
    folioInput.form.dispatchEvent(new Event('submit', { cancelable: true, bubbles: true }));
  }
};




// GUARDIÁN INTELIGENTE DE PLANTILLA: Auto-detecta por RFC y solo marca en rojo sutil cuando no se detecta y está vacío
window.validateMandatoryExactEmployees = async function() {
  const calEmpInput = document.getElementById('cal_emp');
  const folioInput = document.getElementById('folioInput');
  if (!calEmpInput) return true;

  let hintEl = document.getElementById('mallen-mandatory-emp-warning');
  let rawVal = (calEmpInput.value || '').trim();
  let numVal = parseInt(rawVal, 10);

  // Si está vacío pero el usuario ingresó un RFC o empresa, intentar detectar automáticamente desde el padrón
  const qFolio = (folioInput?.value || '').trim();
  if ((!rawVal || isNaN(numVal) || numVal < 1) && qFolio.length >= 3) {
    try {
      const resp = await fetch('/api/lookup?rfc=' + encodeURIComponent(qFolio));
      if (resp.ok) {
        const lk = await resp.json();
        if (lk && lk.estimated_employees && Number(lk.estimated_employees) > 0) {
          calEmpInput.value = String(lk.estimated_employees);
          rawVal = calEmpInput.value;
          numVal = Number(lk.estimated_employees);
        }
      }
    } catch (_) {}
  }

  // Si tras la autodetección sigue sin número de empleados válido, iluminar en rojo sutil y no avanzar
  if (!rawVal || isNaN(numVal) || numVal < 1) {
    const detailsParent = calEmpInput.closest('details');
    if (detailsParent) detailsParent.setAttribute('open', 'open');

    calEmpInput.classList.add('mallen-input-subtle-red');
    if (!hintEl) {
      hintEl = document.createElement('div');
      hintEl.id = 'mallen-mandatory-emp-warning';
      hintEl.setAttribute('role', 'alert');
      hintEl.className = 'mallen-subtle-red-hint';
      calEmpInput.parentElement.appendChild(hintEl);
    }
    hintEl.textContent = 'No se detectó plantilla automática para este registro. Indique el número exacto de colaboradores para calcular su diagnóstico.';
    hintEl.style.display = 'block';
    calEmpInput.focus();
    calEmpInput.scrollIntoView({ behavior: 'smooth', block: 'center' });
    return false;
  }

  calEmpInput.classList.remove('mallen-input-subtle-red');
  if (hintEl) hintEl.style.display = 'none';
  return true;
};

document.addEventListener('DOMContentLoaded', () => {
  const folioEl = document.getElementById('folioInput');
  if (folioEl) folioEl.setAttribute('placeholder', 'Ingrese RFC');
  const domEl = document.getElementById('domainInput');
  if (domEl) domEl.setAttribute('placeholder', 'Ingrese dominio web');
  const calEmpEl = document.getElementById('cal_emp');
  if (calEmpEl) {
    calEmpEl.value = '';
    calEmpEl.setAttribute('placeholder', '');
    calEmpEl.removeAttribute('required');
    calEmpEl.addEventListener('input', () => {
      if (parseInt(calEmpEl.value, 10) >= 1) {
        calEmpEl.classList.remove('mallen-input-subtle-red');
        const warn = document.getElementById('mallen-mandatory-emp-warning');
        if (warn) warn.style.display = 'none';
      }
    });
  }
});


// ATERRIZAJE INTELIGENTE DESDE ENLACE FORMBOT (?calibrate=1&rfc=...&razon=...&scian=...&emp=...)
document.addEventListener('DOMContentLoaded', async () => {
  try {
    const params = new URLSearchParams(window.location.search);
    const rfcParam = (params.get('rfc') || params.get('ref') || '').trim();
    const domParam = (params.get('domain') || '').trim();
    const empParam = parseInt(params.get('emp') || '0', 10);
    const isCalibrateLanding = params.get('calibrate') === '1' || rfcParam.length >= 3;

    if (!isCalibrateLanding || !rfcParam) return;

    const folioEl = document.getElementById('folioInput');
    const domEl = document.getElementById('domainInput');
    const calEmpEl = document.getElementById('cal_emp');

    if (folioEl) folioEl.value = rfcParam;
    if (domEl && domParam) domEl.value = domParam;

    // Consultar /api/lookup para pre-llenar todos los datos que sí existan en el padrón
    const resp = await fetch('/api/lookup?rfc=' + encodeURIComponent(rfcParam));
    if (resp.ok) {
      const lk = await resp.json();
      if (lk && lk.estimated_employees && Number(lk.estimated_employees) > 0) {
        if (calEmpEl) calEmpEl.value = String(lk.estimated_employees);
      } else if (empParam > 0 && calEmpEl) {
        calEmpEl.value = String(empParam);
      } else if (calEmpEl) {
        // Si el FormBot no detectó empleados exactos, abrir el panel y resaltar suavemente #cal_emp
        const det = calEmpEl.closest('details');
        if (det) det.setAttribute('open', 'open');
        calEmpEl.classList.add('mallen-input-subtle-red');
        let hintEl = document.getElementById('mallen-mandatory-emp-warning');
        if (!hintEl) {
          hintEl = document.createElement('div');
          hintEl.id = 'mallen-mandatory-emp-warning';
          hintEl.className = 'mallen-subtle-red-hint';
          calEmpEl.parentElement.appendChild(hintEl);
        }
        hintEl.textContent = 'Su registro está listo. Ingrese únicamente el número exacto de colaboradores de su empresa para generar su Pre-Dictamen.';
        hintEl.style.display = 'block';
        setTimeout(() => {
          calEmpEl.focus();
          calEmpEl.scrollIntoView({ behavior: 'smooth', block: 'center' });
        }, 250);
      }
    }
  } catch (_) {}
});


// ============================================================================
// 1. REPARACIÓN INTERACTIVA DE LAS 4 TARJETAS DEL SIMULADOR IN SITU (-30%, -25%, -20%, -25% VEMP)
// ============================================================================
window.initInteractiveRiskSandboxCards = function() {
  const sbxCheckboxes = Array.from(document.querySelectorAll('.sbx-toggle'));
  const slider = document.getElementById('sbx-emp-slider');
  const fineEl = document.getElementById('sbx-total-fine');
  const scoreEl = document.getElementById('sbx-risk-score');
  const empLabelEl = document.getElementById('sbx-emp-label');

  function recalculateSandbox() {
    const emp = slider ? Math.max(1, parseInt(slider.value || '45', 10)) : 45;
    if (empLabelEl) empLabelEl.textContent = emp + ' colaboradores';

    // Base VEMP proporcional a la plantilla o sincronizada con el último escaneo
    const baseFine = window.__lastScannedVempMxn && window.__lastScannedEmpCount
      ? (window.__lastScannedVempMxn * (emp / Math.max(1, window.__lastScannedEmpCount)))
      : (425000 + emp * 27960);

    const weights = [0.30, 0.25, 0.20, 0.25];
    let mitigationRatio = 0;

    sbxCheckboxes.forEach((cb, idx) => {
      const card = cb.closest('label') || cb.parentElement;
      const w = parseFloat(cb.getAttribute('data-weight') || String(weights[idx] || 0.25));
      if (cb.checked) {
        mitigationRatio += w;
        if (card) {
          card.classList.add('mallen-sandbox-card-active');
          card.classList.remove('mallen-sandbox-card-inactive');
        }
      } else {
        if (card) {
          card.classList.remove('mallen-sandbox-card-active');
          card.classList.add('mallen-sandbox-card-inactive');
        }
      }
    });

    // Si los 4 instrumentos están activos, deja una contingencia residual mínima operativa (2.5%);
    // a medida que se desactivan casillas, sube hasta el 100% de la exposición VEMP.
    const effectiveReduction = Math.min(0.975, mitigationRatio * 0.975);
    const residualFine = Math.max(0, baseFine * (1 - effectiveReduction));
    const residualScore = Math.max(12, Math.round(88 * (1 - effectiveReduction * 0.85)));

    if (fineEl) {
      fineEl.textContent = '$' + residualFine.toLocaleString('es-MX', { minimumFractionDigits: 2, maximumFractionDigits: 2 }) + ' MXN';
    }
    if (scoreEl) {
      scoreEl.textContent = residualScore + '/100';
    }
  }

  // Asegurar que tanto los checkboxes `.sbx-toggle` como las tarjetas que dicen "Blindaje STPS...",
  // "Contrato Marco Fecha Cierta...", "Conciliación IMSS..." y "Aviso LFPDPPP..." sean 100% clicables
  sbxCheckboxes.forEach((cb) => {
    cb.addEventListener('change', recalculateSandbox);
    const card = cb.closest('label') || cb.parentElement;
    if (card && !card.dataset.sbxBound) {
      card.dataset.sbxBound = '1';
      card.style.cursor = 'pointer';
      card.addEventListener('click', (ev) => {
        if (ev.target !== cb) {
          cb.checked = !cb.checked;
          recalculateSandbox();
        }
      });
    }
  });

  if (slider) {
    slider.addEventListener('input', recalculateSandbox);
  }
  recalculateSandbox();
};

// ============================================================================
// 2. MENÚ DESPLEGABLE LATERAL EXCLUSIVO PARA VERSIÓN MÓVIL (DRAWER RESPONSIVO)
//    + CORRECCIÓN DE "39 NOMs" EN LA BARRA SUPERIOR
// ============================================================================
window.initMobileResponsiveDrawerAndNav = function() {
  const headerContainer = document.querySelector('header > div');
  const desktopNav = document.querySelector('header nav');

  if (desktopNav) {
    desktopNav.className = 'hidden md:flex items-center gap-4 text-xs font-semibold text-slate-300';
    desktopNav.innerHTML =
      '<a href="#express-check" class="hover:text-white transition">Diagnóstico Exprés</a>' +
      '<a href="#mallen-vanta-trust-center" class="hover:text-white transition">Entregables y Precios</a>' +
      '<a href="/normativa" id="nav-link-normativa-dedicada" class="hover:text-white transition">Directorio Normativo (39 NOMs)</a>' +
      '<a href="/radar-dof" id="nav-link-radar-dedicado" class="hover:text-white transition">Radar DOF Diario</a>' +
      '<a href="/dashboard.html" class="text-emerald-400 hover:text-emerald-300 transition">Panel Despachos</a>' +
      '<span class="inline-flex rounded-md border border-slate-700 p-0.5 text-[11px] font-semibold">' +
        '<a href="/?country=MX" class="px-2 py-0.5 rounded bg-emerald-600 text-white">MX</a>' +
        '<a href="/region/de" class="px-2 py-0.5 hover:text-emerald-400">DACH</a>' +
        '<a href="/region/nl" class="px-2 py-0.5 hover:text-emerald-400">Benelux</a>' +
      '</span>' +
      '<button type="button" data-mallen-theme-btn onclick="window.toggleMallenTheme()" class="mallen-btn-secondary px-3 py-1 text-xs">Modo Claro</button>';
  }

  if (headerContainer && !document.getElementById('mallen-mobile-menu-trigger')) {
    const mobileControls = document.createElement('div');
    mobileControls.className = 'flex md:hidden items-center gap-2';
    mobileControls.innerHTML =
      '<button type="button" data-mallen-theme-btn onclick="window.toggleMallenTheme()" class="mallen-btn-secondary px-2.5 py-1.5 text-xs">Modo Claro</button>' +
      '<button type="button" id="mallen-mobile-menu-trigger" aria-label="Abrir menú de navegación" class="mallen-btn-primary px-3.5 py-2 text-xs font-bold">' +
        'Menú ☰' +
      '</button>';
    headerContainer.appendChild(mobileControls);
  }

  if (!document.getElementById('mallen-mobile-drawer')) {
    const backdrop = document.createElement('div');
    backdrop.id = 'mallen-mobile-drawer-backdrop';
    const drawer = document.createElement('aside');
    drawer.id = 'mallen-mobile-drawer';
    drawer.setAttribute('aria-label', 'Menú móvil');
    drawer.innerHTML =
      '<div>' +
        '<div class="flex items-center justify-between pb-4 mb-4 border-b border-slate-800">' +
          '<div>' +
            '<span class="font-extrabold text-base block">FIRMA MALLEN®</span>' +
            '<span class="text-xs text-emerald-400 font-semibold uppercase">Ingeniería Legal Preventiva</span>' +
          '</div>' +
          '<button type="button" id="mallen-mobile-drawer-close" class="mallen-btn-secondary px-3 py-1 text-xs">Cerrar ✕</button>' +
        '</div>' +
        '<nav class="flex flex-col space-y-2.5">' +
          '<a href="#express-check" class="mallen-mobile-link py-2.5 px-3 rounded-lg border border-slate-800 font-semibold">Diagnóstico Exprés por RFC</a>' +
          '<a href="#mallen-vanta-trust-center" class="mallen-mobile-link py-2.5 px-3 rounded-lg border border-slate-800 font-semibold">Entregables y Precios ($3,000 MXN)</a>' +
          '<a href="/normativa" class="mallen-mobile-link py-2.5 px-3 rounded-lg border border-slate-800 font-semibold">Directorio Normativo (39 NOMs)</a>' +
          '<a href="/radar-dof" class="mallen-mobile-link py-2.5 px-3 rounded-lg border border-slate-800 font-semibold">Radar DOF Diario</a>' +
          '<a href="/dashboard.html" class="mallen-mobile-link py-2.5 px-3 rounded-lg border border-emerald-700/50 text-emerald-400 font-bold">Panel para Despachos</a>' +
        '</nav>' +
        '<div class="mt-6 pt-4 border-t border-slate-800">' +
          '<span class="block text-xs font-semibold uppercase text-slate-400 mb-2">Jurisdicción Activa:</span>' +
          '<div class="grid grid-cols-3 gap-2 text-center text-xs font-bold">' +
            '<a href="/?country=MX" class="py-2 rounded-lg bg-emerald-600 text-white">MX</a>' +
            '<a href="/region/de" class="py-2 rounded-lg border border-slate-700">DACH</a>' +
            '<a href="/region/nl" class="py-2 rounded-lg border border-slate-700">Benelux</a>' +
          '</div>' +
        '</div>' +
      '</div>' +
      '<div class="pt-4 border-t border-slate-800 text-xs text-slate-400">' +
        'Soporte Institucional: <strong class="text-emerald-400">soporte@mallen.lat</strong>' +
      '</div>';

    document.body.appendChild(backdrop);
    document.body.appendChild(drawer);

    function closeDrawer() {
      drawer.classList.remove('open');
      backdrop.classList.remove('open');
    }
    function openDrawer() {
      drawer.classList.add('open');
      backdrop.classList.add('open');
    }

    const triggerBtn = document.getElementById('mallen-mobile-menu-trigger');
    const closeBtn = document.getElementById('mallen-mobile-drawer-close');
    if (triggerBtn) triggerBtn.addEventListener('click', openDrawer);
    if (closeBtn) closeBtn.addEventListener('click', closeDrawer);
    backdrop.addEventListener('click', closeDrawer);
    drawer.querySelectorAll('.mallen-mobile-link').forEach((lnk) => {
      lnk.addEventListener('click', closeDrawer);
    });
  }
};

document.addEventListener('DOMContentLoaded', () => {
  window.initMobileResponsiveDrawerAndNav();
  window.initInteractiveRiskSandboxCards();
});
