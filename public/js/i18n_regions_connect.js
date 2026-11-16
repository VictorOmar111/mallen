/**
 * FIRMA W16 MALLEN® — Passive Jurisdiction & Top-Bar Switcher (Zero Entry-Wall)
 * Cumple con estándares UX B2B (Linear/Stripe): acceso inmediato al buscador sin
 * pantallas negras de bloqueo inicial. El cambio de país se realiza bajo demanda.
 */
(function () {
    "use strict";

    function initPassiveRegionRouting() {
        try {
            if (!localStorage.getItem("mallen_country")) {
                localStorage.setItem("mallen_country", "MX");
            }
            if (!localStorage.getItem("mallen_consented_locale_v3")) {
                localStorage.setItem("mallen_consented_locale_v3", "es-MX");
            }
        } catch (e) {}
    }

    if (document.readyState === "loading") {
        document.addEventListener("DOMContentLoaded", initPassiveRegionRouting);
    } else {
        initPassiveRegionRouting();
    }
})();
