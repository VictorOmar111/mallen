/**
 * FIRMA MALLEN® — Passive Accept-Language Detection & Top-Bar Region Switcher
 * Elimina el doble muro modal de entrada y habilita interacción inmediata.
 */
(function () {
    "use strict";

    function detectDefaultRegionFromBrowser() {
        try {
            const savedCountry = localStorage.getItem("mallen_country");
            if (savedCountry) return savedCountry;
            const navLang = (navigator.language || "es-MX").toLowerCase();
            let detected = "MX";
            let locale = "es-MX";
            if (navLang.startsWith("de-at")) { detected = "AT"; locale = "de-AT"; }
            else if (navLang.startsWith("de-ch")) { detected = "CH"; locale = "de-CH"; }
            else if (navLang.startsWith("fr-ch")) { detected = "CH"; locale = "fr-CH"; }
            else if (navLang.startsWith("it-ch")) { detected = "CH"; locale = "it-CH"; }
            else if (navLang.startsWith("de")) { detected = "DE"; locale = "de-DE"; }
            else if (navLang.startsWith("nl-be")) { detected = "BE"; locale = "nl-BE"; }
            else if (navLang.startsWith("fr-be")) { detected = "BE"; locale = "fr-BE"; }
            else if (navLang.startsWith("nl")) { detected = "NL"; locale = "nl-NL"; }
            else if (navLang.startsWith("fr-lu") || navLang.startsWith("lb")) { detected = "LU"; locale = "fr-LU"; }
            localStorage.setItem("mallen_country", detected);
            localStorage.setItem("mallen_consented_locale_v3", locale);
            return detected;
        } catch (e) {
            return "MX";
        }
    }

    window.switchMallenRegionDirect = function (code) {
        const c = String(code || "MX").toUpperCase();
        try {
            localStorage.setItem("mallen_country", c);
        } catch (e) {}
        if (c === "MX") {
            window.location.href = "/?country=MX";
        } else {
            window.location.href = "/region/" + c.toLowerCase();
        }
    };

    if (document.readyState === "loading") {
        document.addEventListener("DOMContentLoaded", detectDefaultRegionFromBrowser);
    } else {
        detectDefaultRegionFromBrowser();
    }
})();
