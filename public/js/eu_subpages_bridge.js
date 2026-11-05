/**
 * FIRMA W16 MALLEN® — Puente Directo al Directorio Minucioso de DACH & Benelux
 * - Botones limpios sin el contador "6 Dossiers".
 * - Desplegables (<summary>) dinámicos y específicos para cada expediente y precepto legal,
 *   sin repetir frases genéricas ni contadores "2 Étapes Clés".
 */
(function () {
    "use strict";

    function getActiveLocale() {
        const urlParams = new URLSearchParams(window.location.search);
        return urlParams.get("lang") || localStorage.getItem("mallen_locale") || document.documentElement.lang || "de-DE";
    }

    function getLangPrefix() {
        return (getActiveLocale().split("-")[0] || "de").toLowerCase();
    }

    // Genera un título de desplegable dinámico y único para cada expediente usando su ley y su enfoque
    function buildDynamicAccordionLabel(langPrefix, dossier, idx) {
        const ref = dossier.lawRef ? dossier.lawRef.split("•")[0].trim() : ("#" + (idx + 1));
        const shortTitle = (dossier.title || "").split("&")[0].split("(")[0].trim();

        const prefixes = {
            "de": [
                "Prüfprotokoll & Schutzmaßnahmen nach " + ref + " (" + shortTitle + ") öffnen →",
                "Operativer ROI & Umsetzungsschritte gemäß " + ref + " einsehen →",
                "Gesetzlicher Rahmen & Nachweisführung (" + ref + " — " + shortTitle + ") →",
                "Haftungsprävention & Dokumentationsleitfaden nach " + ref + " →",
                "Behördliche Prüfkriterien & Abhilfemaßnahmen (" + ref + ") →",
                "Technischer Compliance-Plan für " + shortTitle + " (" + ref + ") →"
            ],
            "fr": [
                "Protocole de conformité & mesures selon " + ref + " (" + shortTitle + ") →",
                "ROI opérationnel & plan d'action réglementaire (" + ref + ") →",
                "Cadre légal & preuves documentaires requises (" + ref + " — " + shortTitle + ") →",
                "Prévention des sanctions & guide d'exécution selon " + ref + " →",
                "Critères d'inspection officielle & mise en conformité (" + ref + ") →",
                "Feuille de route technique pour " + shortTitle + " (" + ref + ") →"
            ],
            "it": [
                "Protocollo di conformità & misure secondo " + ref + " (" + shortTitle + ") →",
                "ROI operativo & piano d'azione normativo (" + ref + ") →",
                "Quadro legale & documentazione probatoria richiesta (" + ref + " — " + shortTitle + ") →",
                "Prevenzione delle sanzioni & guida attuativa secondo " + ref + " →",
                "Criteri d'ispezione ufficiale & adeguamento (" + ref + ") →",
                "Piano tecnico di protezione per " + shortTitle + " (" + ref + ") →"
            ],
            "nl": [
                "Toetsingsprotocol & maatregelen conform " + ref + " (" + shortTitle + ") →",
                "Operationele ROI & wettelijk stappenplan (" + ref + ") →",
                "Wettelijk kader & vereiste bewijsstukken (" + ref + " — " + shortTitle + ") →",
                "Boetepreventie & implementatiegids volgens " + ref + " →",
                "Inspectiecriteria & nalevingsmaatregelen (" + ref + ") →",
                "Technisch compliance-plan voor " + shortTitle + " (" + ref + ") →"
            ],
            "lb": [
                "Prüfprotokoll & Moossnamen no " + ref + " (" + shortTitle + ") →",
                "Operative ROI & gesetzleche Plang (" + ref + ") →",
                "Rechtskader & erfuerderlech Dokumenter (" + ref + " — " + shortTitle + ") →",
                "Strofepräventioun & Ëmsetzungsguide no " + ref + " →",
                "Inspektiounskriterien & Konformitéit (" + ref + ") →",
                "Technesche Schutzplang fir " + shortTitle + " (" + ref + ") →"
            ]
        };

        const list = prefixes[langPrefix] || prefixes["de"];
        return list[idx % list.length];
    }

    function bindAndRenderDeepDachCards() {
        const locale = getActiveLocale();
        const deep = (window.MALLEN_DACH_DEEP && window.MALLEN_DACH_DEEP[locale]) ? window.MALLEN_DACH_DEEP[locale] : null;
        const container = document.getElementById("eu-laws-cards-container");
        if (!container) return;

        if (deep && Array.isArray(deep.pillars) && deep.pillars.length > 0) {
            const ui = deep.ui || {};
            container.innerHTML = "";
            deep.pillars.forEach(function (pillar, idx) {
                const card = document.createElement("div");
                card.className = "quantum-card rounded-2xl p-6 border border-slate-800 hover:border-sky-500/60 transition flex flex-col justify-between cursor-pointer";
                card.innerHTML = `
                    <div>
                        <div class="flex items-center justify-between gap-2 mb-3">
                            <span class="text-[10px] font-mono-code font-bold uppercase px-2.5 py-1 rounded bg-slate-900 border border-slate-700 text-sky-400">${pillar.code}</span>
                            <span class="text-[10px] bg-red-950/80 text-red-300 border border-red-800/60 px-2.5 py-0.5 rounded font-mono-code font-bold">${pillar.nonComplianceBadge}</span>
                        </div>
                        <h3 class="text-base font-extrabold text-white mb-2">${pillar.title}</h3>
                        <p class="text-xs text-slate-300 leading-relaxed mb-4">${pillar.desc}</p>
                        <div class="text-[11px] text-slate-300 font-mono-code bg-slate-950/65 border border-slate-800 rounded-xl p-3 mb-4">
                            <strong class="text-emerald-400 block mb-0.5">${ui.directBenefitLbl || "Direkter Nutzen:"}</strong>
                            ${pillar.benefit}
                        </div>
                    </div>
                    <button type="button" class="w16-action-btn w-full px-4 py-2.5 rounded-xl bg-gradient-to-r from-blue-600 to-emerald-600 text-white text-xs font-mono-code font-bold text-left transition shadow-lg">
                        ${pillar.linkText}
                    </button>
                `;
                card.addEventListener("click", function (ev) {
                    ev.preventDefault();
                    ev.stopPropagation();
                    window.openEuLawSubpage(idx);
                });
                container.appendChild(card);
            });
            return;
        }

        // Para Benelux (NL, BE, LU): quitar cualquier badge "6 Dossiers" si existiera y enlazar clic
        Array.from(container.children).forEach(function (card, idx) {
            if (card.getAttribute("data-bound-sub") === "1") return;
            card.setAttribute("data-bound-sub", "1");
            card.style.cursor = "pointer";
            card.addEventListener("click", function (ev) {
                ev.preventDefault();
                ev.stopPropagation();
                window.openEuLawSubpage(idx);
            });
        });
    }

    window.openEuLawSubpage = function (pillarIdx) {
        const locale = getActiveLocale();
        const langPrefix = getLangPrefix();
        const deep = (window.MALLEN_DACH_DEEP && window.MALLEN_DACH_DEEP[locale])
            ? window.MALLEN_DACH_DEEP[locale]
            : (window.MALLEN_DACH_DEEP ? window.MALLEN_DACH_DEEP["de-DE"] : null);

        let subpageView = document.getElementById("eu-law-deep-subpage-view");
        if (!subpageView) {
            subpageView = document.createElement("div");
            subpageView.id = "eu-law-deep-subpage-view";
            subpageView.className = "mb-14 gpu-smooth";
            const lawsSection = document.getElementById("laws-grid");
            if (lawsSection && lawsSection.parentNode) {
                lawsSection.parentNode.insertBefore(subpageView, lawsSection.nextSibling);
            }
        }

        const lawsSection = document.getElementById("laws-grid");
        if (lawsSection) lawsSection.classList.add("hidden");
        subpageView.classList.remove("hidden");

        if (deep && Array.isArray(deep.pillars) && deep.pillars[pillarIdx]) {
            const pillar = deep.pillars[pillarIdx];
            const sp = pillar.subpage;
            const ui = deep.ui || {};
            const tabs = ui.tabs || ["ROI & Gesundheit", "Operativer Vorteil", "Rechtskontext", "Rahmen & Pflichtige", "Schlüsselschritte", "Schutzleitfaden"];

            let dossiersHtml = "";
            (sp.dossiers || []).forEach(function (d, dIdx) {
                const stepsList = (d.steps || []).map(function (st, sIdx) {
                    return `<li class="flex items-start gap-2.5 text-xs text-slate-200 leading-relaxed">
                        <span class="px-2 py-0.5 rounded bg-emerald-950 border border-emerald-700 text-emerald-300 font-mono-code font-bold text-[10px] shrink-0">0${sIdx + 1}</span>
                        <span>${st}</span>
                    </li>`;
                }).join("");

                const lowRange = Math.max(18, d.pct - 4);
                const highRange = Math.min(92, d.pct + 4);
                const dynamicSummaryTitle = buildDynamicAccordionLabel(langPrefix, d, dIdx);

                dossiersHtml += `
                    <div class="quantum-card rounded-2xl p-6 border border-slate-800 flex flex-col justify-between">
                        <div>
                            <div class="flex flex-wrap items-center justify-between gap-2 mb-3">
                                <span class="text-[10px] font-mono-code font-bold uppercase px-2.5 py-1 rounded bg-blue-950 border border-blue-800 text-sky-300">${d.num} • ${d.lawRef}</span>
                                <span class="text-[10px] font-mono-code px-2.5 py-0.5 rounded bg-slate-900 border border-slate-700 text-emerald-400 font-bold">SHA-256</span>
                            </div>
                            <h4 class="text-base md:text-lg font-extrabold text-white mb-3">${d.title}</h4>

                            <div class="bg-slate-950/90 border border-red-900/50 rounded-xl p-3.5 mb-4">
                                <div class="flex items-center justify-between text-[11px] font-mono-code mb-1.5">
                                    <span class="text-red-400 font-bold">${ui.nonComplianceLabel || "BEANSTANDUNGSQUOTE:"} ${lowRange}% – ${highRange}% (${d.pct}%)</span>
                                    <span class="text-slate-400">${ui.inspectedLabel || ""}</span>
                                </div>
                                <div class="w-full h-2.5 bg-slate-900 rounded-full overflow-hidden border border-slate-800 mb-2">
                                    <div class="h-full bg-gradient-to-r from-red-600 via-rose-500 to-amber-500 rounded-full" style="width: ${d.pct}%;"></div>
                                </div>
                                <div class="text-[11px] font-mono-code text-amber-300">
                                    <strong class="text-red-400">${ui.sanctionRiskLabel || "Sanktionsrisiko:"}</strong> ${d.sanction}
                                </div>
                            </div>

                            <!-- Desplegable con Título Dinámico Específico al Precepto Legal (Sin contador genérico) -->
                            <details class="bg-slate-950/85 border border-slate-800 rounded-xl p-4 mb-4" ${dIdx === 0 ? "open" : ""}>
                                <summary class="cursor-pointer text-xs font-mono-code font-bold text-sky-400 hover:text-emerald-400 transition">
                                    ${dynamicSummaryTitle}
                                </summary>
                                <div class="mt-4 pt-3 border-t border-slate-800 space-y-3 text-xs">
                                    <div class="bg-emerald-950/25 border border-emerald-700/40 rounded-lg p-3">
                                        <strong class="text-emerald-400 font-mono-code block text-[10px] uppercase mb-1">1. ${tabs[0]} & ${tabs[1]}:</strong>
                                        <p class="text-slate-200 leading-relaxed">${d.roi}</p>
                                    </div>
                                    <div class="bg-slate-900/80 border border-slate-800 rounded-lg p-3">
                                        <strong class="text-sky-400 font-mono-code block text-[10px] uppercase mb-1">2. ${tabs[2]} & ${tabs[3]}:</strong>
                                        <p class="text-slate-300 leading-relaxed mb-1.5">${d.statText}</p>
                                        <p class="text-slate-400 leading-relaxed">${d.context}</p>
                                    </div>
                                    <div class="bg-slate-900/80 border border-slate-800 rounded-lg p-3">
                                        <strong class="text-amber-400 font-mono-code block text-[10px] uppercase mb-2">3. ${tabs[4]} & ${tabs[5]}:</strong>
                                        <ul class="space-y-2">${stepsList}</ul>
                                    </div>
                                </div>
                            </details>
                        </div>

                        <div class="pt-3 border-t border-slate-800 flex items-center justify-between text-[11px] font-mono-code">
                            <span class="text-slate-400">${d.lawRef}</span>
                            <a href="#osint-calibration-engine" onclick="window.closeEuLawSubpage()" class="text-emerald-400 hover:text-emerald-300 font-bold">${ui.auditTaxBtn || "Prüfung starten →"}</a>
                        </div>
                    </div>
                `;
            });

            subpageView.innerHTML = `
                <div class="quantum-card border border-sky-500/40 rounded-3xl p-6 md:p-10 shadow-2xl">
                    <div class="flex flex-wrap items-center justify-between gap-4 pb-6 mb-6 border-b border-slate-800">
                        <div>
                            <span class="text-[11px] font-mono-code font-bold uppercase px-3 py-1 rounded-full bg-blue-950 border border-blue-700 text-sky-300">${sp.headerTag}</span>
                            <h2 class="text-2xl md:text-3xl font-extrabold text-white mt-3">${sp.headerTitle}</h2>
                            <p class="text-xs md:text-sm text-slate-300 mt-1">${sp.headerSub}</p>
                        </div>
                        <button type="button" onclick="window.closeEuLawSubpage()" class="w16-action-btn bg-gradient-to-r from-blue-600 to-emerald-600 text-white px-5 py-2.5 rounded-xl text-xs font-mono-code font-bold transition shrink-0 shadow-lg">
                            ${ui.backToPortal || "← Zurück"}
                        </button>
                    </div>
                    <p class="text-xs md:text-sm text-slate-300 font-mono-code bg-slate-950/90 border border-slate-800 rounded-xl p-4 mb-8 leading-relaxed">
                        ${sp.introText}
                    </p>
                    <div class="grid md:grid-cols-2 gap-6">
                        ${dossiersHtml}
                    </div>
                </div>
            `;
            subpageView.scrollIntoView({ behavior: "smooth", block: "start" });
        }
    };

    window.closeEuLawSubpage = function () {
        const subpageView = document.getElementById("eu-law-deep-subpage-view");
        const lawsSection = document.getElementById("laws-grid");
        if (subpageView) subpageView.classList.add("hidden");
        if (lawsSection) {
            lawsSection.classList.remove("hidden");
            lawsSection.scrollIntoView({ behavior: "smooth", block: "start" });
        }
    };

    if (document.readyState === "loading") {
        document.addEventListener("DOMContentLoaded", function () {
            setTimeout(bindAndRenderDeepDachCards, 90);
        });
    } else {
        setTimeout(bindAndRenderDeepDachCards, 90);
    }
})();
