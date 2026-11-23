/**
 * FIRMA MALLEN® — Smooth RAF Pointer Light, Deeper Bottom Dim & Boosted Lamp (v95)
 */
(function () {
    "use strict";

    const THEME_LABELS = {
        "es": { light: "Modo Claro", dark: "Modo Oscuro" },
        "de": { light: "Hell", dark: "Dunkel" },
        "fr": { light: "Clair", dark: "Sombre" },
        "it": { light: "Chiaro", dark: "Scuro" },
        "nl": { light: "Licht", dark: "Donker" },
        "en": { light: "Light", dark: "Dark" }
    };

    window.getMallenThemeLabel = function () {
        const lang = (document.documentElement.lang || localStorage.getItem("mallen_consented_locale_v3") || "es").slice(0, 2).toLowerCase();
        const dict = THEME_LABELS[lang] || THEME_LABELS.es;
        const isLight = document.documentElement.classList.contains("light-mode");
        return isLight ? dict.dark : dict.light;
    };

    window.toggleMallenTheme = function () {
        const root = document.documentElement;
        const isLight = root.classList.toggle("light-mode");
        try { localStorage.setItem("mallen_theme_mode", isLight ? "light" : "dark"); } catch (e) {}
        document.querySelectorAll("[data-mallen-theme-btn]").forEach(function (btn) {
            btn.textContent = window.getMallenThemeLabel();
        });
    };

    function mountArchitecturalWallAndLamp() {
        if (document.getElementById("mallen-architectural-wall") || !document.body) return;

        const wall = document.createElement("div");
        wall.id = "mallen-architectural-wall";
        wall.setAttribute("aria-hidden", "true");
        wall.innerHTML =
            '<svg id="mallen-wall-geometry-layer" viewBox="0 0 1600 1000" preserveAspectRatio="xMidYMid slice" xmlns="http://www.w3.org/2000/svg">' +
                '<line x1="0" y1="840" x2="1600" y2="140" stroke="#94a3b8" stroke-opacity="0.11" stroke-width="1"/>' +
                '<line x1="220" y1="0" x2="1380" y2="1000" stroke="#64748b" stroke-opacity="0.08" stroke-width="1" stroke-dasharray="10 8"/>' +
                '<polygon points="70,55 660,135 515,610 35,450" fill="#94a3b8" fill-opacity="0.035" stroke="#94a3b8" stroke-opacity="0.16" stroke-width="1"/>' +
                '<polygon points="1020,30 1555,185 1390,705 895,500" fill="#cbd5e1" fill-opacity="0.03" stroke="#cbd5e1" stroke-opacity="0.14" stroke-width="1"/>' +
                '<polygon points="410,610 1210,510 1335,965 275,945" fill="#64748b" fill-opacity="0.03" stroke="#94a3b8" stroke-opacity="0.12" stroke-width="1"/>' +
                '<circle cx="1185" cy="345" r="305" fill="none" stroke="#94a3b8" stroke-opacity="0.11" stroke-width="1"/>' +
                '<circle cx="375" cy="685" r="250" fill="none" stroke="#94a3b8" stroke-opacity="0.09" stroke-width="1"/>' +
            '</svg>' +
            '<div id="mallen-wall-vignette"></div>' +
            '<div id="mallen-wall-lamp"></div>';

        document.body.insertBefore(wall, document.body.firstChild);
    }

    function assignKelvinAndScale(el, idx) {
        let kelvin = "236, 244, 255";
        let spotSize = "380px";
        let spotAlpha = "0.18";

        if (el.closest("#express-check") || el.closest("#osint-calibration-engine")) {
            kelvin = "236, 244, 255";
            spotSize = "460px";
            spotAlpha = "0.20";
        } else if (el.closest("#risk-sandbox-section") || el.closest("#eu-interactive-sandbox")) {
            kelvin = "255, 255, 255";
            spotSize = "290px";
            spotAlpha = "0.24";
        } else if (el.closest("#mallen-vanta-trust-center") || el.closest("#eu-trust-center-section")) {
            kelvin = idx % 3 === 0 ? "236, 244, 255" : (idx % 3 === 1 ? "255, 255, 255" : "255, 244, 224");
            spotSize = "410px";
            spotAlpha = "0.19";
        } else if (el.closest("#validez-legal") || el.closest("#probative-validity-section") || el.tagName === "DETAILS" || el.closest("#faq")) {
            kelvin = "255, 244, 224";
            spotSize = "340px";
            spotAlpha = "0.16";
        } else {
            kelvin = idx % 2 === 0 ? "236, 244, 255" : "255, 244, 224";
            spotSize = "390px";
            spotAlpha = "0.18";
        }

        el.style.setProperty("--card-kelvin", kelvin);
        el.style.setProperty("--spot-size", spotSize);
        el.style.setProperty("--spot-alpha", spotAlpha);
        el.dataset.kelvinRgb = kelvin;
    }

    window.hydrateMallenChromaticCards = function () {
        const selectors = [
            ".mallen-glass-card",
            ".quantum-card",
            ".mallen-surface-card",
            "#mallen-vanta-trust-center .grid > div",
            "#eu-trust-center-section .grid > div",
            "#risk-sandbox-section label",
            "#eu-interactive-sandbox label",
            "#validez-legal .grid > div",
            "#probative-validity-section .grid > div",
            "#laws-grid .grid > div",
            "main .grid > a",
            "main .grid > div.rounded-xl",
            "main .grid > div.rounded-2xl",
            "details.rounded-xl"
        ];
        const nodes = document.querySelectorAll(selectors.join(", "));
        nodes.forEach(function (card, idx) {
            if (!card.classList.contains("mallen-glass-card")) {
                card.classList.add("mallen-glass-card");
            }
            assignKelvinAndScale(card, idx);
        });
    };

    let deferredPwaPrompt = null;
    window.addEventListener("beforeinstallprompt", function (e) {
        e.preventDefault();
        deferredPwaPrompt = e;
    });

    window.openMallenAppInstaller = async function () {
        if (deferredPwaPrompt) {
            deferredPwaPrompt.prompt();
            try { await deferredPwaPrompt.userChoice; } catch (_) {}
            deferredPwaPrompt = null;
            return;
        }
        const modal = document.getElementById("mallen-pwa-install-modal");
        const backdrop = document.getElementById("mallen-pwa-modal-backdrop");
        if (modal && backdrop) {
            modal.classList.add("open");
            backdrop.classList.add("open");
        }
    };

    window.closeMallenAppInstaller = function () {
        const modal = document.getElementById("mallen-pwa-install-modal");
        const backdrop = document.getElementById("mallen-pwa-modal-backdrop");
        if (modal && backdrop) {
            modal.classList.remove("open");
            backdrop.classList.remove("open");
        }
    };

    window.ensureMallenAppDownloadButtons = function () {
        if ("serviceWorker" in navigator) {
            navigator.serviceWorker.register("/sw.js").catch(function () {});
        }
        // Boton de Descargar App movido exclusivamente al footer
        if (document.body && !document.getElementById("mallen-pwa-install-modal")) {
            const backdrop = document.createElement("div");
            backdrop.id = "mallen-pwa-modal-backdrop";
            backdrop.onclick = window.closeMallenAppInstaller;

            const modal = document.createElement("div");
            modal.id = "mallen-pwa-install-modal";
            modal.setAttribute("role", "dialog");
            modal.setAttribute("aria-label", "Instalar Aplicación Web FIRMA MALLEN");
            modal.innerHTML =
                '<div class="flex items-center justify-between pb-3 mb-4 border-b border-slate-800">' +
                    '<div>' +
                        '<span class="text-xs font-mono-data uppercase text-slate-400 block">APP WEB NATIVA SIN TIENDAS</span>' +
                        '<h3 class="text-lg font-extrabold text-white">Instalar App FIRMA MALLEN®</h3>' +
                    '</div>' +
                    '<button type="button" onclick="window.closeMallenAppInstaller()" class="mallen-btn-secondary px-3 py-1 text-xs">Cerrar ✕</button>' +
                '</div>' +
                '<div class="space-y-4 text-sm text-slate-300">' +
                    '<div class="p-3.5 rounded-xl border border-slate-800 bg-slate-950/70">' +
                        '<div class="font-bold text-white mb-1">En iPhone / iPad (iOS Safari):</div>' +
                        '<p class="text-xs text-slate-300">1. Toque <strong>Compartir</strong> (cuadro con flecha arriba).<br>2. Seleccione <strong>"Agregar a pantalla de inicio"</strong> y pulse <strong>Agregar</strong>.</p>' +
                    '</div>' +
                    '<div class="p-3.5 rounded-xl border border-slate-800 bg-slate-950/70">' +
                        '<div class="font-bold text-white mb-1">En Android (Chrome / Edge / Samsung):</div>' +
                        '<p class="text-xs text-slate-300">1. Toque el menú del navegador (<strong>⋮</strong> arriba a la derecha).<br>2. Seleccione <strong>"Instalar aplicación"</strong> o <strong>"Agregar a la pantalla principal"</strong>.</p>' +
                    '</div>' +
                '</div>';
            document.body.appendChild(backdrop);
            document.body.appendChild(modal);
        }
    };

    function syncScrollDimAndCursorBoost() {
        const docH = Math.max(1, document.documentElement.scrollHeight - window.innerHeight);
        const ratio = Math.min(1, Math.max(0, window.scrollY / docH));
        document.documentElement.style.setProperty("--scroll-dim-alpha", "0");
        document.documentElement.style.setProperty("--cursor-beam-boost", (1.0 + ratio * 0.65).toFixed(3));
    }

    function initSmoothKelvinEngine() {
        try {
            if (localStorage.getItem("mallen_theme_mode") === "light") {
                document.documentElement.classList.add("light-mode");
            }
        } catch (e) {}

        document.querySelectorAll("[data-mallen-theme-btn]").forEach(function (btn) {
            btn.textContent = window.getMallenThemeLabel();
        });

        mountArchitecturalWallAndLamp();
        window.hydrateMallenChromaticCards();
        window.ensureMallenAppDownloadButtons();
        setTimeout(window.hydrateMallenChromaticCards, 400);
        setTimeout(window.ensureMallenAppDownloadButtons, 400);

        const lamp = document.getElementById("mallen-wall-lamp");
        let lastCard = null;
        let lastRect = null;
        let currentKelvin = "";
        let pendingX = window.innerWidth * 0.5;
        let pendingY = window.innerHeight * 0.35;
        let rafScheduled = false;

        syncScrollDimAndCursorBoost();
        window.addEventListener("scroll", function () {
            syncScrollDimAndCursorBoost();
            if (lastCard) lastRect = lastCard.getBoundingClientRect();
        }, { passive: true });

        document.addEventListener("pointerover", function (ev) {
            const card = ev.target && ev.target.closest ? ev.target.closest(".mallen-glass-card") : null;
            if (card !== lastCard) {
                if (lastCard) {
                    lastCard.style.setProperty("--mouse-x", "-999px");
                    lastCard.style.setProperty("--mouse-y", "-999px");
                }
                lastCard = card;
                lastRect = card ? card.getBoundingClientRect() : null;
                if (card) {
                    const k = card.dataset.kelvinRgb || "236, 244, 255";
                    if (k !== currentKelvin) {
                        currentKelvin = k;
                        document.documentElement.style.setProperty("--active-kelvin", k);
                    }
                }
            }
        }, { passive: true });

        function renderPointerStep() {
            rafScheduled = false;
            if (lamp) {
                lamp.style.transform = "translate3d(" + pendingX + "px, " + pendingY + "px, 0)";
            }
            if (lastCard && lastRect) {
                lastCard.style.setProperty("--mouse-x", (pendingX - lastRect.left) + "px");
                lastCard.style.setProperty("--mouse-y", (pendingY - lastRect.top) + "px");
            }
        }

        window.addEventListener("pointermove", function (ev) {
            pendingX = ev.clientX;
            pendingY = ev.clientY;
            if (!rafScheduled) {
                rafScheduled = true;
                requestAnimationFrame(renderPointerStep);
            }
        }, { passive: true });
    }

    if (document.readyState === "loading") {
        document.addEventListener("DOMContentLoaded", initSmoothKelvinEngine);
    } else {
        initSmoothKelvinEngine();
    }
})();
