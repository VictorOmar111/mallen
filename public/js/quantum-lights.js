(function () {
    let spotlightEl = null;
    let mainConsoleEl = null;
    let redWaveEl = null;
    let greenWaveEl = null;

    let lastX = window.innerWidth * 0.5;
    let lastY = window.innerHeight * 0.5;
    let currentScale = 1.0;
    let defaultRgb = '59, 130, 246';
    let burstActive = false;

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

    function applySpotlightTransform() {
        if (!spotlightEl) return;
        spotlightEl.style.transform = `translate3d(${lastX}px, ${lastY}px, 0) scale3d(${currentScale}, ${currentScale}, 1)`;
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

    // Dispara la explosión Roja -> Haz Verde Expandido -> Retorno a la página (100% GPU)
    window.triggerPhotonicBurst = function () {
        if (!redWaveEl || !greenWaveEl) return;

        redWaveEl.classList.remove('fire-wave');
        greenWaveEl.classList.remove('fire-wave');
        void redWaveEl.offsetWidth; // Reiniciar animación en compositor GPU
        redWaveEl.classList.add('fire-wave');
        greenWaveEl.classList.add('fire-wave');

        burstActive = true;
        // Fase 1: Destello Rojo en el haz del puntero
        setSpotlightColor(PALETTE.red);
        currentScale = 1.45;
        applySpotlightTransform();

        // Fase 2 (220ms): Transformación a Haz Verde Más Grande
        setTimeout(() => {
            setSpotlightColor(PALETTE.emerald);
            currentScale = 1.95;
            applySpotlightTransform();
        }, 220);

        // Fase 3 (950ms): Regreso fluido al tamaño y color de la página
        setTimeout(() => {
            currentScale = 1.0;
            burstActive = false;
            setSpotlightColor(defaultRgb);
            applySpotlightTransform();
        }, 950);
    };

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

            c.addEventListener('pointerenter', () => {
                if (!burstActive) setSpotlightColor(rgb);
            }, { passive: true });
            c.addEventListener('pointerleave', () => {
                if (!burstActive) setSpotlightColor(defaultRgb);
            }, { passive: true });
        });
    }

    function initQuantumLighting() {
        configurePageScene();

        spotlightEl = document.createElement('div');
        spotlightEl.id = 'mallen-wasm-spotlight';
        spotlightEl.style.transition = 'transform 0.015s linear';
        document.body.appendChild(spotlightEl);

        redWaveEl = document.createElement('div');
        redWaveEl.className = 'photonic-wave-red';
        document.body.appendChild(redWaveEl);

        greenWaveEl = document.createElement('div');
        greenWaveEl.className = 'photonic-wave-green';
        document.body.appendChild(greenWaveEl);

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

        // SEGUIMIENTO 1:1 INSTANTÁNEO CON EL PUNTERO (Cero retraso, misma velocidad física que el mouse)
        window.addEventListener('pointermove', (e) => {
            lastX = e.clientX;
            lastY = e.clientY;
            applySpotlightTransform();
        }, { passive: true });
    }

    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', initQuantumLighting);
    } else {
        initQuantumLighting();
    }
})();
