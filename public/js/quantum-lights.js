/**
 * FIRMA MALLEN® — Architectural Wall, Kelvin White Light (120 FPS GPU) & Native PWA Installer (v89)
 * - Solo luz blanca modulada en Temperatura Kelvin: Fría (6500K), Neutra (5500K) y Cálida (3200K).
 * - Fondo como pared arquitectónica iluminada con figuras geométricas disruptivas en tonos grisáceos.
 * - Instalador directo de App Web para Android (Chrome/Edge) y Apple iOS/iPadOS (Safari).
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

    // 1. Construir la Pared Arquitectónica con Geometría Disruptiva en Grises y Lámpara GPU
    function mountArchitecturalWallAndLamp() {
        if (document.getElementById("mallen-architectural-wall") || !document.body) return;

        const wall = document.createElement("div");
        wall.id = "mallen-architectural-wall";
        wall.setAttribute("aria-hidden", "true");
        wall.innerHTML =
            '<svg class="wall-geometry-svg" viewBox="0 0 1600 1000" preserveAspectRatio="xMidYMid slice" xmlns="http://www.w3.org/2000/svg">' +
                '<defs>' +
                    '<linearGradient id="gris-disruptivo-1" x1="0%" y1="0%" x2="100%" y2="100%">' +
                        '<stop offset="0%" stop-color="#94a3b8" stop-opacity="0.11"/>' +
                        '<stop offset="50%" stop-color="#64748b" stop-opacity="0.04"/>' +
                        '<stop offset="100%" stop-color="#475569" stop-opacity="0.10"/>' +
                    '</linearGradient>' +
                    '<linearGradient id="gris-disruptivo-2" x1="100%" y1="0%" x2="0%" y2="100%">' +
                        '<stop offset="0%" stop-color="#cbd5e1" stop-opacity="0.09"/>' +
                        '<stop offset="100%" stop-color="#334155" stop-opacity="0.02"/>' +
                    '</linearGradient>' +
                '</defs>' +
                '<!-- Polígono Monolítico Asimétrico (Corte Áureo Izquierdo) -->' +
                '<polygon points="90,60 640,140 510,590 40,440" fill="url(#gris-disruptivo-1)" stroke="#64748b" stroke-opacity="0.16" stroke-width="1"/>' +
                '<!-- Prisma Arquitectónico Disruptivo (Derecha Superior) -->' +
                '<polygon points="1040,40 1540,190 1380,680 910,490" fill="url(#gris-disruptivo-2)" stroke="#94a3b8" stroke-opacity="0.14" stroke-width="1"/>' +
                '<!-- Plano Geométrico Desfasado (Inferior Central) -->' +
                '<polygon points="420,620 1190,520 1310,960 290,940" fill="url(#gris-disruptivo-1)" stroke="#64748b" stroke-opacity="0.12" stroke-width="1"/>' +
                '<!-- Líneas de Tensión y Arcos Técnicos No Concéntricos en Escala de Grises -->' +
                '<line x1="0" y1="820" x2="1600" y2="180" stroke="#94a3b8" stroke-opacity="0.10" stroke-width="1"/>' +
                '<line x1="260" y1="0" x2="1340" y2="1000" stroke="#64748b" stroke-opacity="0.08" stroke-width="1" stroke-dasharray="10 8"/>' +
                '<circle cx="1180" cy="340" r="290" fill="none" stroke="#94a3b8" stroke-opacity="0.10" stroke-width="1"/>' +
                '<circle cx="1205" cy="355" r="210" fill="none" stroke="#64748b" stroke-opacity="0.08" stroke-width="1" stroke-dasharray="4 6"/>' +
                '<circle cx="380" cy="680" r="240" fill="none" stroke="#94a3b8" stroke-opacity="0.09" stroke-width="1"/>' +
            '</svg>' +
            '<div id="mallen-wall-lamp"></div>';

        document.body.insertBefore(wall, document.body.firstChild);
    }

    // 2. Asignar Temperatura de Luz Blanca (Fría 6500K / Neutra 5500K / Cálida 3200K) y Tamaño por Sección
    function assignKelvinAndScale(el, idx) {
        let kelvin = "236, 244, 255"; // 6500K Blanco Frío Cuarzo
        let spotSize = "480px";
        let spotAlpha = "0.15";
        let scaleLamp = 1.0;

        if (el.closest("#express-check") || el.closest("#osint-calibration-engine")) {
            kelvin = "236, 244, 255"; // Blanco Frío Cuarzo (Alta precisión en diagnóstico)
            spotSize = "600px";
            spotAlpha = "0.19";
            scaleLamp = 1.18;
        } else if (el.closest("#risk-sandbox-section") || el.closest("#eu-interactive-sandbox")) {
            kelvin = "255, 255, 255"; // Blanco Puro Neutro (5500K en Simulador Táctico)
            spotSize = "310px";
            spotAlpha = "0.22";
            scaleLamp = 0.82;
        } else if (el.closest("#mallen-vanta-trust-center") || el.closest("#eu-trust-center-section")) {
            // Alternar sutilmente entre blanco cálido (3200K), neutro (5500K) y frío (6500K) en las 3 columnas
            if (idx % 3 === 0) kelvin = "236, 244, 255";
            else if (idx % 3 === 1) kelvin = "255, 255, 255";
            else kelvin = "255, 245, 228";
            spotSize = "460px";
            spotAlpha = "0.17";
            scaleLamp = 1.05;
        } else if (el.closest("#validez-legal") || el.closest("#probative-validity-section") || el.tagName === "DETAILS") {
            kelvin = "255, 245, 228"; // 3200K Blanco Cálido Halógeno (Lectura jurídica prolongada)
            spotSize = "360px";
            spotAlpha = "0.13";
            scaleLamp = 0.90;
        } else {
            kelvin = idx % 2 === 0 ? "236, 244, 255" : "255, 245, 228";
            spotSize = "480px";
            spotAlpha = "0.16";
            scaleLamp = 1.0;
        }

        el.style.setProperty("--card-kelvin", kelvin);
        el.style.setProperty("--spot-size", spotSize);
        el.style.setProperty("--spot-alpha", spotAlpha);
        el.dataset.kelvinRgb = kelvin;
        el.dataset.lampScale = String(scaleLamp);
    }

    let activeCards = [];

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

            if (!card.dataset.kelvinBound) {
                card.dataset.kelvinBound = "1";
                card.addEventListener("pointerenter", function () {
                    card.style.setProperty("--spot-visible", "1");
                    document.documentElement.style.setProperty("--active-kelvin", card.dataset.kelvinRgb || "255, 255, 255");
                }, { passive: true });
                card.addEventListener("pointerleave", function () {
                    card.style.setProperty("--spot-visible", "0");
                }, { passive: true });
            }
        });
        activeCards = Array.from(document.querySelectorAll(".mallen-glass-card"));
    };

    // 3. Instalador de App Web Progresiva (PWA) para Android y Apple iOS (Manzano)
    let deferredPwaPrompt = null;

    window.addEventListener("beforeinstallprompt", function (e) {
        e.preventDefault();
        deferredPwaPrompt = e;
    });

    window.openMallenAppInstaller = async function () {
        if (deferredPwaPrompt) {
            deferredPwaPrompt.prompt();
            try {
                await deferredPwaPrompt.userChoice;
            } catch (_) {}
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

    function mountPwaInstallUiAndServiceWorker() {
        if ("serviceWorker" in navigator) {
            navigator.serviceWorker.register("/sw.js").catch(function () {});
        }

        // Asegurar link rel="manifest" y meta tags de Apple iOS en <head>
        if (document.head && !document.querySelector('link[rel="manifest"]')) {
            const mLink = document.createElement("link");
            mLink.rel = "manifest";
            mLink.href = "/manifest.webmanifest";
            document.head.appendChild(mLink);

            const appleCap = document.createElement("meta");
            appleCap.name = "apple-mobile-web-app-capable";
            appleCap.content = "yes";
            document.head.appendChild(appleCap);

            const appleTitle = document.createElement("meta");
            appleTitle.name = "apple-mobile-web-app-title";
            appleTitle.content = "FIRMA MALLEN®";
            document.head.appendChild(appleTitle);
        }

        // Inyectar botón "Descargar App" en la barra de navegación superior y en el drawer móvil
        const nav = document.querySelector("header nav");
        if (nav && !document.getElementById("mallen-install-app-btn")) {
            const appBtn = document.createElement("button");
            appBtn.type = "button";
            appBtn.id = "mallen-install-app-btn";
            appBtn.onclick = window.openMallenAppInstaller;
            appBtn.className = "mallen-btn-secondary px-3 py-1 text-xs font-semibold border border-slate-600 hover:border-white transition";
            appBtn.textContent = "Descargar App";
            nav.appendChild(appBtn);
        }

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
                        '<span class="text-xs font-mono-data uppercase text-slate-400 block">APLICACIÓN OFICIAL SIN TIENDAS</span>' +
                        '<h3 class="text-lg font-extrabold text-white">Instalar App FIRMA MALLEN®</h3>' +
                    '</div>' +
                    '<button type="button" onclick="window.closeMallenAppInstaller()" class="mallen-btn-secondary px-3 py-1 text-xs">Cerrar ✕</button>' +
                '</div>' +
                '<div class="space-y-4 text-sm text-slate-300">' +
                    '<div class="p-3.5 rounded-xl border border-slate-800 bg-slate-950/60">' +
                        '<div class="font-bold text-white mb-1">En iPhone / iPad (iOS Safari — Manzano):</div>' +
                        '<p class="text-xs text-slate-300">1. Toque el botón <strong>Compartir</strong> (icono del cuadro con flecha hacia arriba en la barra de Safari).<br>2. Seleccione <strong>"Agregar a pantalla de inicio"</strong> y pulse <strong>Agregar</strong>.</p>' +
                    '</div>' +
                    '<div class="p-3.5 rounded-xl border border-slate-800 bg-slate-950/60">' +
                        '<div class="font-bold text-white mb-1">En Android (Chrome / Edge / Samsung):</div>' +
                        '<p class="text-xs text-slate-300">1. Toque el menú del navegador (<strong>⋮</strong> arriba a la derecha).<br>2. Seleccione <strong>"Instalar aplicación"</strong> o <strong>"Agregar a la pantalla principal"</strong>.</p>' +
                    '</div>' +
                '</div>';

            document.body.appendChild(backdrop);
            document.body.appendChild(modal);
        }
    }

    function initFastKelvinEngine() {
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
        mountPwaInstallUiAndServiceWorker();

        const lamp = document.getElementById("mallen-wall-lamp");
        let rafId = 0;
        let lastX = window.innerWidth * 0.5;
        let lastY = window.innerHeight * 0.35;
        let activeHoveredCard = null;

        document.addEventListener("pointerover", function (ev) {
            const card = ev.target && ev.target.closest ? ev.target.closest(".mallen-glass-card") : null;
            if (card) {
                activeHoveredCard = card;
            }
        }, { passive: true });

        window.addEventListener("pointermove", function (ev) {
            lastX = ev.clientX;
            lastY = ev.clientY;
            if (rafId) return;
            rafId = requestAnimationFrame(function () {
                // 1. Mover la lámpara de pared usando únicamente composición GPU (translate3d = 120 FPS reales)
                if (lamp) {
                    const scale = activeHoveredCard ? (activeHoveredCard.dataset.lampScale || "1") : "1";
                    lamp.style.transform = "translate3d(" + lastX + "px, " + lastY + "px, 0) scale(" + scale + ")";
                }

                // 2. Actualizar coordenadas únicamente en la tarjeta bajo el cursor (O(1) en lugar de O(N))
                if (activeHoveredCard) {
                    const rect = activeHoveredCard.getBoundingClientRect();
                    activeHoveredCard.style.setProperty("--mouse-x", (lastX - rect.left) + "px");
                    activeHoveredCard.style.setProperty("--mouse-y", (lastY - rect.top) + "px");
                }
                rafId = 0;
            });
        }, { passive: true });
    }

    if (document.readyState === "loading") {
        document.addEventListener("DOMContentLoaded", initFastKelvinEngine);
    } else {
        initFastKelvinEngine();
    }
})();
