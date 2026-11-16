/**
 * FIRMA MALLEN® — Lightweight Theme & Surface Controller (Zero GPU Overdraw)
 * Sustituye los reflectores multicolor por conmutación semántica limpia.
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

    function initSavedTheme() {
        try {
            if (localStorage.getItem("mallen_theme_mode") === "light") {
                document.documentElement.classList.add("light-mode");
            }
        } catch (e) {}
        document.querySelectorAll("[data-mallen-theme-btn]").forEach(function (btn) {
            btn.textContent = window.getMallenThemeLabel();
        });
    }

    if (document.readyState === "loading") {
        document.addEventListener("DOMContentLoaded", initSavedTheme);
    } else {
        initSavedTheme();
    }
})();
