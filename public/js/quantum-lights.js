(function () {
    let wasmExports = null;
    let spotlightEl = null;
    let mainConsoleEl = null;

    let curX = -1000, curY = -1000;
    let tgtX = -1000, tgtY = -1000;
    let isAnimating = false;
    let defaultRgb = '59, 130, 246';

    const PALETTE = {
        red:     '239, 68, 68',
        purple:  '168, 85, 247',
        amber:   '245, 158, 11',
        cyan:    '6, 182, 212',
        teal:    '20, 184, 166',
        indigo:  '99, 102, 241',
        sky:     '14, 165, 233',
        emerald: '16, 185, 129',
        blue:    '59, 130, 246'
    };

    const COLOR_REGEX = /(?:text|border|bg)-(red|purple|amber|cyan|teal|indigo|sky|emerald|blue)-(?:300|400|500|600|700|800|900)/;

    async function loadRustPhotonWasm() {
        try {
            const response = await fetch('/wasm/photon_engine.wasm?v=2026.10');
            const bytes = await response.arrayBuffer();
            const { instance } = await WebAssembly.instantiate(bytes);
            wasmExports = instance.exports;
        } catch (_) {
            wasmExports = {
                lerp: (c, t, f) => c + (t - c) * f
            };
        }
    }

    function detectElementRgb(el, fallback) {
        const ownMatch = (el.className || '').match(COLOR_REGEX);
        if (ownMatch && PALETTE[ownMatch[1]]) return PALETTE[ownMatch[1]];

        const accentChild = el.querySelector(
            '[class*="text-red-"], [class*="text-purple-"], [class*="text-amber-"], [class*="text-cyan-"], [class*="text-teal-"], [class*="text-indigo-"], [class*="text-sky-"], [class*="text-emerald-"], [class*="text-blue-"], [class*="border-red-"], [class*="border-purple-"], [class*="border-amber-"]'
        );
        if (accentChild) {
            const childMatch = (accentChild.className || '').match(COLOR_REGEX);
            if (childMatch && PALETTE[childMatch[1]]) return PALETTE[childMatch[1]];
        }
        return fallback || PALETTE.blue;
    }

    function configurePageScene() {
        const path = window.location.pathname;
        const body = document.body;

        if (path.startsWith('/verify')) {
            body.dataset.scene = 'verify';
            defaultRgb = body.innerHTML.includes('border-amber-') ? PALETTE.amber : PALETTE.emerald;
        } else if (path.startsWith('/checkout')) {
            body.dataset.scene = 'checkout';
            defaultRgb = PALETTE.red;
        } else if (path.startsWith('/dashboard')) {
            body.dataset.scene = 'dashboard';
            defaultRgb = PALETTE.purple;
        } else if (path.startsWith('/normas') || path.startsWith('/validez') || path.startsWith('/terms') || path.startsWith('/privacy')) {
            body.dataset.scene = 'forensic';
            defaultRgb = detectElementRgb(body, PALETTE.indigo);
        } else {
            body.dataset.scene = 'home';
            defaultRgb = PALETTE.blue;
        }

        const parts = defaultRgb.split(',');
        document.documentElement.style.setProperty('--scene-r', parts[0].trim());
        document.documentElement.style.setProperty('--scene-g', parts[1].trim());
        document.documentElement.style.setProperty('--scene-b', parts[2].trim());
    }

    function setSpotlightColor(rgb) {
        if (spotlightEl && spotlightEl.dataset.rgb !== rgb) {
            spotlightEl.dataset.rgb = rgb;
            spotlightEl.style.setProperty('--spot-rgb', rgb);
        }
        if (mainConsoleEl && mainConsoleEl.dataset.rgb !== rgb) {
            mainConsoleEl.dataset.rgb = rgb;
            mainConsoleEl.style.setProperty('--console-rgb', rgb);
        }
    }

    function bindCards() {
        const cardSelectors = [
            'a[href^="/normas/"]',
            'a[href^="/validez/"]',
            '.grid > div.rounded-2xl',
            '.grid > div.rounded-xl',
            'main > div.rounded-xl',
            'section.border-l-4',
            '#checkoutForm > div.rounded-xl',
            'body > div > div.rounded-xl',
            '#express-check details',
            '#live-scan-result > div.rounded-xl',
            '.max-w-4xl > div.rounded-2xl',
            '.max-w-5xl > div.rounded-2xl'
        ];

        const cards = document.querySelectorAll(cardSelectors.join(','));
        cards.forEach((c) => {
            if (c.dataset.quantumBound) return;
            c.dataset.quantumBound = '1';
            c.classList.add('quantum-card');

            const rgb = detectElementRgb(c, defaultRgb);
            c.style.setProperty('--card-rgb', rgb);

            // Cambio de color instantáneo solo al entrar/salir del cuadro (Cero costo durante mousemove)
            c.addEventListener('pointerenter', () => setSpotlightColor(rgb), { passive: true });
            c.addEventListener('pointerleave', () => setSpotlightColor(defaultRgb), { passive: true });
        });
    }

    function tickSpotlight() {
        curX = wasmExports.lerp(curX, tgtX, 0.35);
        curY = wasmExports.lerp(curY, tgtY, 0.35);

        if (spotlightEl) {
            spotlightEl.style.transform = `translate3d(${curX.toFixed(1)}px, ${curY.toFixed(1)}px, 0)`;
        }

        if (Math.abs(tgtX - curX) > 0.5 || Math.abs(tgtY - curY) > 0.5) {
            requestAnimationFrame(tickSpotlight);
        } else {
            isAnimating = false;
        }
    }

    async function initQuantumLighting() {
        await loadRustPhotonWasm();
        configurePageScene();

        spotlightEl = document.createElement('div');
        spotlightEl.id = 'mallen-wasm-spotlight';
        document.body.appendChild(spotlightEl);

        const consoles = document.querySelectorAll('#express-check > div > div.rounded-2xl, body > div.max-w-3xl.rounded-2xl, body > div.max-w-5xl.rounded-2xl');
        consoles.forEach((con) => {
            con.classList.add('quantum-laser-console');
            mainConsoleEl = con;
        });

        setSpotlightColor(defaultRgb);
        bindCards();

        const liveRes = document.getElementById('live-scan-result');
        if (liveRes) {
            const obs = new MutationObserver(() => bindCards());
            obs.observe(liveRes, { attributes: true, attributeFilter: ['class'] });
        }

        // En mousemove solo actualizamos las 2 coordenadas numéricas — Cero lecturas ni mutaciones de estilos
        window.addEventListener('pointermove', (e) => {
            tgtX = e.clientX;
            tgtY = e.clientY;
            if (curX < -500) {
                curX = tgtX;
                curY = tgtY;
            }
            if (!isAnimating) {
                isAnimating = true;
                requestAnimationFrame(tickSpotlight);
            }
        }, { passive: true });
    }

    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', initQuantumLighting);
    } else {
        initQuantumLighting();
    }
})();
