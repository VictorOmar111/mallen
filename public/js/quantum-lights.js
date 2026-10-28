(function () {
    function initQuantumLighting() {
        const cardSelectors = [
            'a[href^="/normas/"]',
            'a[href^="/validez/"]',
            '.grid > div.rounded-2xl',
            '.grid > div.rounded-xl',
            'main > div.rounded-xl',
            'section.border-l-4'
        ];
        const cards = document.querySelectorAll(cardSelectors.join(','));
        cards.forEach((c) => c.classList.add('quantum-card'));

        const consoles = document.querySelectorAll('#express-check .rounded-2xl, body > div.max-w-3xl.rounded-2xl, body > div.max-w-4xl.rounded-2xl, body > div.max-w-5xl.rounded-2xl');
        consoles.forEach((con) => con.classList.add('quantum-laser-console'));

        let rafPending = false;
        window.addEventListener('mousemove', (e) => {
            if (rafPending) return;
            rafPending = true;
            requestAnimationFrame(() => {
                const mx = e.clientX;
                const my = e.clientY;
                document.documentElement.style.setProperty('--mx', mx + 'px');
                document.documentElement.style.setProperty('--my', my + 'px');

                const allCards = document.querySelectorAll('.quantum-card');
                for (let i = 0; i < allCards.length; i++) {
                    const card = allCards[i];
                    const rect = card.getBoundingClientRect();
                    const cx = mx - rect.left;
                    const cy = my - rect.top;
                    card.style.setProperty('--cx', cx + 'px');
                    card.style.setProperty('--cy', cy + 'px');
                    card.style.setProperty('--card-active', '1');
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
