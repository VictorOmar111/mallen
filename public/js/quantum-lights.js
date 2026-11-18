/**
 * FIRMA MALLEN® — Chromatic Forensic Prism & Multi-Section Specular Engine (60/120 FPS)
 * - Traduce el conmutador de tema al 100% (es, de, fr, it, nl, en).
 * - Detecta automáticamente todas las tarjetas y sub-bloques en cualquier página.
 * - Asigna a cada tarjeta su color cromático (--card-accent) según su órgano regulador o tipografía,
 *   y varía el diámetro del haz (--spot-size) y el brillo (--spot-alpha) según la sección.
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
        try {
            localStorage.setItem("mallen_theme_mode", isLight ? "light" : "dark");
        } catch (e) {}
        document.querySelectorAll("[data-mallen-theme-btn]").forEach(function (btn) {
            btn.textContent = window.getMallenThemeLabel();
        });
    };

    // Detecta el color RGB de una tarjeta analizando sus clases o su contenido regulatorio
    function inferCardAccentRgb(el, idx) {
        const explicit = el.getAttribute("data-agency");
        if (explicit === "stps") return "59, 130, 246";
        if (explicit === "sat") return "16, 185, 129";
        if (explicit === "imss") return "168, 85, 247";
        if (explicit === "inai") return "6, 182, 212";
        if (explicit === "sectorial") return "245, 158, 11";
        if (explicit === "vemp") return "239, 68, 68";

        const innerHtml = el.innerHTML || "";
        const txt = (el.textContent || "").toUpperCase();

        if (innerHtml.indexOf("text-red-") !== -1 || innerHtml.indexOf("text-rose-") !== -1 || txt.indexOf("EXPOSICIÓN RESIDUAL") !== -1) {
            return "239, 68, 68"; // Rojo Carmesí (Contingencia / VEMP)
        }
        if (innerHtml.indexOf("text-purple-") !== -1 || innerHtml.indexOf("text-violet-") !== -1 || txt.indexOf("IMSS") !== -1 || txt.indexOf("MERKLE") !== -1) {
            return "168, 85, 247"; // Púrpura Real (IMSS / Criptografía Merkle)
        }
        if (innerHtml.indexOf("text-amber-") !== -1 || innerHtml.indexOf("text-yellow-") !== -1 || txt.indexOf("INVERSIÓN") !== -1 || txt.indexOf("TARIF") !== -1 || txt.indexOf("UIF") !== -1 || txt.indexOf("PLD") !== -1) {
            return "245, 158, 11"; // Ámbar Institucional (Inversión / Sectorial / PLD)
        }
        if (innerHtml.indexOf("text-cyan-") !== -1 || txt.indexOf("LFPDPPP") !== -1 || txt.indexOf("INAI") !== -1 || txt.indexOf("DSGVO") !== -1 || txt.indexOf("AVG") !== -1) {
            return "6, 182, 212"; // Cian Forense (Privacidad / Datos)
        }
        if (innerHtml.indexOf("text-sky-") !== -1 || innerHtml.indexOf("text-blue-") !== -1 || txt.indexOf("STPS") !== -1 || txt.indexOf("EXPEDIENTE RESERVADO") !== -1 || txt.indexOf("ARBSCHG") !== -1) {
            return "59, 130, 246"; // Azul Eléctrico (STPS / Expediente)
        }
        if (innerHtml.indexOf("text-emerald-") !== -1 || txt.indexOf("SAT") !== -1 || txt.indexOf("69-B") !== -1 || txt.indexOf("DIAGNÓSTICO") !== -1) {
            return "16, 185, 129"; // Verde Esmeralda (SAT / Diagnóstico)
        }

        const paletteCycle = [
            "16, 185, 129",  // Esmeralda
            "59, 130, 246",  // Azul Eléctrico
            "168, 85, 247", // Púrpura
            "6, 182, 212",   // Cian
            "245, 158, 11"   // Ámbar
        ];
        return paletteCycle[idx % paletteCycle.length];
    }

    // Asigna el tamaño del haz (--spot-size) y el brillo (--spot-alpha) según la sección donde vive la tarjeta
    function assignSectionScaleAndBrightness(el) {
        if (el.closest("#express-check") || el.closest("#osint-calibration-engine")) {
            // Sección Hero / Diagnóstico: Haz grande (580px) y brillo alto (0.24)
            el.style.setProperty("--spot-size", "580px");
            el.style.setProperty("--spot-alpha", "0.24");
            el.style.setProperty("--border-alpha", "0.78");
            el.style.setProperty("--beam-alpha", "1.0");
        } else if (el.closest("#risk-sandbox-section") || el.closest("#eu-interactive-sandbox")) {
            // Sección Simulador In Situ: Haz concentrado (300px) y brillo intenso (0.28)
            el.style.setProperty("--spot-size", "300px");
            el.style.setProperty("--spot-alpha", "0.28");
            el.style.setProperty("--border-alpha", "0.86");
            el.style.setProperty("--beam-alpha", "0.96");
        } else if (el.closest("#mallen-vanta-trust-center") || el.closest("#eu-trust-center-section")) {
            // Sección Arquitectura de Blindaje 360°: Haz medio-amplio (460px) y brillo elegante (0.19)
            el.style.setProperty("--spot-size", "460px");
            el.style.setProperty("--spot-alpha", "0.19");
            el.style.setProperty("--border-alpha", "0.68");
            el.style.setProperty("--beam-alpha", "0.92");
        } else if (el.closest("#validez-legal") || el.closest("#probative-validity-section") || el.tagName === "DETAILS") {
            // Sección Garantías Probatorias y FAQs: Haz focal suave (340px) y brillo sutil (0.13)
            el.style.setProperty("--spot-size", "340px");
            el.style.setProperty("--spot-alpha", "0.13");
            el.style.setProperty("--border-alpha", "0.50");
            el.style.setProperty("--beam-alpha", "0.76");
        } else {
            // Vistas dedicadas (/normativa, /radar-dof, /checkout, /normas/*, /verify/*): Haz de 480px y brillo 0.21
            el.style.setProperty("--spot-size", "480px");
            el.style.setProperty("--spot-alpha", "0.21");
            el.style.setProperty("--border-alpha", "0.72");
            el.style.setProperty("--beam-alpha", "0.92");
        }
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
            "main > div.rounded-xl",
            "main > div.rounded-2xl",
            "details.rounded-xl"
        ];
        const nodes = document.querySelectorAll(selectors.join(", "));
        nodes.forEach(function (card, idx) {
            if (!card.classList.contains("mallen-glass-card")) {
                card.classList.add("mallen-glass-card");
            }
            const rgb = inferCardAccentRgb(card, idx);
            card.style.setProperty("--card-accent", rgb);
            assignSectionScaleAndBrightness(card);

            if (!card.dataset.prismBound) {
                card.dataset.prismBound = "1";
                card.addEventListener("pointerenter", function () {
                    card.style.setProperty("--spot-visible", "1");
                    // Sincronizar también el halo ambiental de fondo con el color y escala de esta sección
                    document.documentElement.style.setProperty("--page-ambient-rgb", rgb);
                    const sSize = card.style.getPropertyValue("--spot-size") || "460px";
                    const numPx = parseInt(sSize, 10) || 460;
                    document.documentElement.style.setProperty("--page-ambient-size", (numPx + 180) + "px");
                }, { passive: true });

                card.addEventListener("pointerleave", function () {
                    card.style.setProperty("--spot-visible", "0");
                }, { passive: true });
            }
        });
    };

    function initChromaticPrismEngine() {
        try {
            if (localStorage.getItem("mallen_theme_mode") === "light") {
                document.documentElement.classList.add("light-mode");
            }
        } catch (e) {}

        document.querySelectorAll("[data-mallen-theme-btn]").forEach(function (btn) {
            btn.textContent = window.getMallenThemeLabel();
        });

        window.hydrateMallenChromaticCards();

        // Re-hidratar automáticamente cuando se carguen resultados dinámicos (ej. Radar DOF, Leyes EU o Diagnóstico)
        const observer = new MutationObserver(function () {
            window.hydrateMallenChromaticCards();
        });
        if (document.body) {
            observer.observe(document.body, { childList: true, subtree: true });
        }

        let rafPending = false;
        window.addEventListener("pointermove", function (ev) {
            if (rafPending) return;
            rafPending = true;
            requestAnimationFrame(function () {
                const mx = ev.clientX;
                const my = ev.clientY;
                document.documentElement.style.setProperty("--page-mouse-x", mx + "px");
                document.documentElement.style.setProperty("--page-mouse-y", my + "px");

                const cards = document.querySelectorAll(".mallen-glass-card");
                for (let i = 0; i < cards.length; i++) {
                    const c = cards[i];
                    const rect = c.getBoundingClientRect();
                    // Solo actualizar tarjetas cercanas al puntero para mantener 120 FPS constantes
                    if (mx >= rect.left - 160 && mx <= rect.right + 160 && my >= rect.top - 160 && my <= rect.bottom + 160) {
                        c.style.setProperty("--mouse-x", (mx - rect.left) + "px");
                        c.style.setProperty("--mouse-y", (my - rect.top) + "px");
                        c.style.setProperty("--spot-visible", "1");
                    } else {
                        c.style.setProperty("--spot-visible", "0");
                    }
                }
                rafPending = false;
            });
        }, { passive: true });
    }

    if (document.readyState === "loading") {
        document.addEventListener("DOMContentLoaded", initChromaticPrismEngine);
    } else {
        initChromaticPrismEngine();
    }
})();
