(function () {
    let wasmExports = null;
    let cachedBoxes = [];
    let cardsList = [];
    let spotlightEl = null;
    let mainConsoleEl = null;

    let curR = 59, curG = 130, curB = 246;
    let tgtR = 59, tgtG = 130, tgtB = 246;
    let defaultR = 59, defaultG = 130, defaultB = 246;

    const PALETTE = {
        red:     { r: 239, g: 68,  b: 68  },
        purple:  { r: 168, g: 85,  b: 247 },
        amber:   { r: 245, g: 158, b: 11  },
        cyan:    { r: 6,   g: 182, b: 212 },
        teal:    { r: 20,  g: 184, b: 166 },
        indigo:  { r: 99,  g: 102, b: 241 },
        sky:     { r: 14,  g: 165, b: 233 },
        emerald: { r: 16,  g: 185, b: 129 },
        blue:    { r: 59,  g: 130, b: 246 }
    };

    const COLOR_REGEX = /(?:text|border|bg)-(red|purple|amber|cyan|teal|indigo|sky|emerald|blue)-(?:300|400|500|600|700|800|900)/;

    async function loadRustPhotonWasm() {
        try {
            const response = await fetch('/wasm/photon_engine.wasm?v=2026.9');
            const bytes = await response.arrayBuffer();
            const { instance } = await WebAssembly.instantiate(bytes);
            wasmExports = instance.exports;
        } catch (_) {
            wasmExports = {
                lerp: (c, t, f) => c + (t - c) * f,
                dist_sq: (x1, y1, x2, y2) => (x1 - x2) ** 2 + (y1 - y2) ** 2
            };
        }
    }

    // Extrae el color del propio borde del cuadro o de su primer título/etiqueta (sin confundirse con íconos internos)
    function detectElementPalette(el, fallbackPal) {
        const ownMatch = (el.className || '').match(COLOR_REGEX);
        if (ownMatch && PALETTE[ownMatch[1]]) {
            return PALETTE[ownMatch[1]];
        }
        const accentChild = el.querySelector(
            '[class*="text-red-"], [class*="text-purple-"], [class*="text-amber-"], [class*="text-cyan-"], [class*="text-teal-"], [class*="text-indigo-"], [class*="text-sky-"], [class*="text-emerald-"], [class*="text-blue-"], [class*="border-red-"], [class*="border-purple-"], [class*="border-amber-"]'
        );
        if (accentChild) {
            const childMatch = (accentChild.className || '').match(COLOR_REGEX);
            if (childMatch && PALETTE[childMatch[1]]) {
                return PALETTE[childMatch[1]];
            }
        }
        return fallbackPal || PALETTE.blue;
    }

    function configurePageScene() {
        const path = window.location.pathname;
        const body = document.body;
        let scenePal = PALETTE.blue;

        if (path.startsWith('/verify')) {
            body.dataset.scene = 'verify';
            scenePal = body.innerHTML.includes('border-amber-') ? PALETTE.amber : PALETTE.emerald;
        } else if (path.startsWith('/checkout')) {
            body.dataset.scene = 'checkout';
            scenePal = PALETTE.red;
        } else if (path.startsWith('/dashboard')) {
            body.dataset.scene = 'dashboard';
            scenePal = PALETTE.purple;
        } else if (path.startsWith('/normas') || path.startsWith('/validez') || path.startsWith('/terms') || path.startsWith('/privacy')) {
            body.dataset.scene = 'forensic';
            scenePal = detectElementPalette(body, PALETTE.indigo);
        } else {
            body.dataset.scene = 'home';
            scenePal = PALETTE.blue;
        }

        defaultR = scenePal.r;
        defaultG = scenePal.g;
        defaultB = scenePal.b;
        curR = tgtR = defaultR;
        curG = tgtG = defaultG;
        curB = tgtB = defaultB;

        document.documentElement.style.setProperty('--scene-r', defaultR);
        document.documentElement.style.setProperty('--scene-g', defaultG);
        document.documentElement.style.setProperty('--scene-b', defaultB);
    }

    function registerInteractiveCards() {
        const pageFallback = { r: defaultR, g: defaultG, b: defaultB };
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

        cardsList = Array.from(document.querySelectorAll(cardSelectors.join(',')));
        cardsList.forEach((c) => {
            c.classList.add('quantum-card');
            const pal = detectElementPalette(c, pageFallback);
            c.dataset.r = pal.r;
            c.dataset.g = pal.g;
            c.dataset.b = pal.b;
            c.style.setProperty('--card-rgb', `${pal.r}, ${pal.g}, ${pal.b}`);
        });
        updateGeometryCache();
    }

    function updateGeometryCache() {
        cachedBoxes = new Array(cardsList.length);
        for (let i = 0; i < cardsList.length; i++) {
            const el = cardsList[i];
            const r = el.getBoundingClientRect();
            cachedBoxes[i] = {
                left: r.left,
                top: r.top,
                width: r.width,
                height: r.height,
                cx: r.left + r.width * 0.5,
                cy: r.top + r.height * 0.5,
                r: parseFloat(el.dataset.r || defaultR),
                g: parseFloat(el.dataset.g || defaultG),
                b: parseFloat(el.dataset.b || defaultB),
                active: false
            };
        }
    }

    async function initQuantumLighting() {
        await loadRustPhotonWasm();
        configurePageScene();

        spotlightEl = document.createElement('div');
        spotlightEl.id = 'mallen-wasm-spotlight';
        spotlightEl.style.setProperty('--spot-rgb', `${Math.round(curR)}, ${Math.round(curG)}, ${Math.round(curB)}`);
        document.body.appendChild(spotlightEl);

        const consoles = document.querySelectorAll('#express-check > div > div.rounded-2xl, body > div.max-w-3xl.rounded-2xl, body > div.max-w-5xl.rounded-2xl');
        consoles.forEach((con) => {
            con.classList.add('quantum-laser-console');
            con.style.setProperty('--console-rgb', `${defaultR}, ${defaultG}, ${defaultB}`);
            mainConsoleEl = con;
        });

        registerInteractiveCards();

        const liveRes = document.getElementById('live-scan-result');
        if (liveRes) {
            const obs = new MutationObserver(() => registerInteractiveCards());
            obs.observe(liveRes, { attributes: true, attributeFilter: ['class'] });
        }

        document.querySelectorAll('details').forEach((det) => {
            det.addEventListener('toggle', () => setTimeout(updateGeometryCache, 50));
        });

        window.addEventListener('scroll', updateGeometryCache, { passive: true });
        window.addEventListener('resize', updateGeometryCache, { passive: true });

        let rafPending = false;
        const RADIUS_SQ = 380 * 380;

        window.addEventListener('mousemove', (e) => {
            if (rafPending) return;
            rafPending = true;
            const mx = e.clientX;
            const my = e.clientY;

            requestAnimationFrame(() => {
                let minD2 = Infinity;
                let closestBox = null;

                for (let i = 0; i < cachedBoxes.length; i++) {
                    const box = cachedBoxes[i];
                    if (box.width === 0) continue;
                    const d2 = wasmExports.dist_sq(mx, my, box.cx, box.cy);

                    if (d2 < RADIUS_SQ) {
                        const card = cardsList[i];
                        card.style.setProperty('--cx', (mx - box.left) + 'px');
                        card.style.setProperty('--cy', (my - box.top) + 'px');
                        if (!box.active) {
                            card.style.setProperty('--card-active', '1');
                            box.active = true;
                        }
                        if (d2 < minD2) {
                            minD2 = d2;
                            closestBox = box;
                        }
                    } else if (box.active) {
                        cardsList[i].style.setProperty('--card-active', '0');
                        box.active = false;
                    }
                }

                if (closestBox) {
                    tgtR = closestBox.r;
                    tgtG = closestBox.g;
                    tgtB = closestBox.b;
                } else {
                    tgtR = defaultR;
                    tgtG = defaultG;
                    tgtB = defaultB;
                }

                curR = wasmExports.lerp(curR, tgtR, 0.22);
                curG = wasmExports.lerp(curG, tgtG, 0.22);
                curB = wasmExports.lerp(curB, tgtB, 0.22);

                const rgbStr = `${Math.round(curR)}, ${Math.round(curG)}, ${Math.round(curB)}`;
                if (spotlightEl) {
                    spotlightEl.style.transform = `translate3d(${mx}px, ${my}px, 0)`;
                    spotlightEl.style.setProperty('--spot-rgb', rgbStr);
                }
                if (mainConsoleEl) {
                    mainConsoleEl.style.setProperty('--console-rgb', rgbStr);
                }

                rafPending = false;
            });
        }, { passive: true });
    }

    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', initQuantumLighting);
    } else {
        initQuantumLighting();
    }
})();
