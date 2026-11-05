(function () {
    "use strict";

    // Detect if we are on the main landing page and no country has been selected yet.
    function checkAndShowGateway() {
        const urlParams = new URLSearchParams(window.location.search);
        // Force gateway open if explicitly requested, or if no country is set yet
        const countryUrl = urlParams.get('country');
        const countryStorage = localStorage.getItem('mallen_country');

        if (!countryUrl && !countryStorage && window.location.pathname === '/') {
            if (typeof window.openMallenGateway === 'function') {
                window.openMallenGateway();
            }
        }
    }

    // Attach to DOMContentLoaded or run immediately
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', checkAndShowGateway);
    } else {
        checkAndShowGateway();
    }
})();
