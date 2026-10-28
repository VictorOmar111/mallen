(function () {
    let wasmExports = null;
    let cachedBoxes = [];
    let cardsList = [];
    let spotlightEl = null;

    async function loadRustPhotonWasm() {
        try {
            const response = await fetch('/wasm/photon_engine.wasm?v=2026.6');
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

    // Cachear posiciones sólo en scroll/resize — JAMÁS dentro de mousemove
    function updateGeometryCache() {
        cachedBoxes = new Array(cardsList.length);
        for (let i = 0; i < cardsList.length; i++) {
            const r = cardsList[i].getBoundingClientRect();
            cachedBoxes[i] = {
                left: r.left,
                top: r.top,
                cx: r.left + r.width * 0.5,
                cy: r.top + r.height * 0.5,
                active: false
            };
        }
    }

    async function initQuantumLighting() {
        await loadRustPhotonWasm();

        spotlightEl = document.createElement('div');
        spotlightEl.id = 'mallen-wasm-spotlight';
        document.body.appendChild(spotlightEl);

        const cardSelectors = [
            'a[href^="/normas/"]',
            'a[href^="/validez/"]',
            '.grid > div.rounded-2xl',
            '.grid > div.rounded-xl',
            'main > div.rounded-xl',
            'section.border-l-4'
        ];
        cardsList = Array.from(document.querySelectorAll(cardSelectors.join(',')));
        cardsList.forEach((c) => c.classList.add('quantum-card'));

        const consoles = document.querySelectorAll('#express-check .rounded-2xl, body > div.max-w-3xl.rounded-2xl, body > div.max-w-4xl.rounded-2xl, body > div.max-w-5xl.rounded-2xl');
        consoles.forEach((con) => con.classList.add('quantum-laser-console'));

        updateGeometryCache();
        window.addEventListener('scroll', updateGeometryCache, { passive: true });
        window.addEventListener('resize', updateGeometryCache, { passive: true });

        let rafPending = false;
        const RADIUS_SQ = 340 * 340; // Solo tarjetas a menos de 340px del cursor se actualizan

        window.addEventListener('mousemove', (e) => {
            if (rafPending) return;
            rafPending = true;
            const mx = e.clientX;
            const my = e.clientY;

            requestAnimationFrame(() => {
                // 1. Mover haz de luz en GPU vía translate3d (Cero Layout / Cero Paint)
                if (spotlightEl) {
                    spotlightEl.style.transform = `translate3d(${mx}px, ${my}px, 0)`;
                }

                // 2. Spatial Culling en Wasm: evaluar distancia cuadrática en código máquina
                for (let i = 0; i < cachedBoxes.length; i++) {
                    const box = cachedBoxes[i];
                    const d2 = wasmExports.dist_sq(mx, my, box.cx, box.cy);
                    if (d2 < RADIUS_SQ) {
                        const card = cardsList[i];
                        card.style.setProperty('--cx', (mx - box.left) + 'px');
                        card.style.setProperty('--cy', (my - box.top) + 'px');
                        if (!box.active) {
                            card.style.setProperty('--card-active', '1');
                            box.active = true;
                        }
                    } else if (box.active) {
                        cardsList[i].style.setProperty('--card-active', '0');
                        box.active = false;
                    }
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
