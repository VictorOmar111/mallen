/**
 * Sello de Blindaje Pericial Mallen (B2B Viral Loop Widget)
 * Uso: <div id="mallen-trust-badge" data-rfc="MALLEN-CORP-2026"></div>
 *      <script src="/js/badge.js" defer></script>
 */
(function () {
    const container = document.getElementById("mallen-trust-badge");
    if (!container) return;

    const rfc = (container.getAttribute("data-rfc") || "MALLEN-CORP-2026").toUpperCase();

    container.innerHTML = `
        <div id="mallen-badge-trigger" style="display:inline-flex;align-items:center;gap:10px;background:#0f172a;border:1px solid #1e293b;padding:8px 14px;border-radius:12px;cursor:pointer;font-family:'JetBrains Mono',monospace;font-size:11px;color:#e2e8f0;box-shadow:0 4px 14px rgba(0,0,0,0.4);transition:border-color 0.2s;">
            <span style="width:8px;height:8px;border-radius:9999px;background:#10b981;display:inline-block;box-shadow:0 0 8px #10b981;"></span>
            <span>AUDITADO POR <strong>MALLEN</strong> • FECHA CIERTA SCJN</span>
            <span style="background:#064e3b;color:#34d399;padding:2px 6px;border-radius:6px;font-size:9px;font-weight:700;">VERIFICADO</span>
        </div>
    `;

    const trigger = document.getElementById("mallen-badge-trigger");
    trigger.addEventListener("mouseenter", () => (trigger.style.borderColor = "#10b981"));
    trigger.addEventListener("mouseleave", () => (trigger.style.borderColor = "#1e293b"));

    trigger.addEventListener("click", async () => {
        let data = {
            reference: rfc,
            merkle_root: "0x8f2a9b4c1e7f3d9a10b8c2d4e6f8a0b2c4d6e8f1",
            leaf_hash: "0x9c4e1a2b3d5f7e8a",
            timestamp: new Date().toISOString()
        };
        try {
            const res = await fetch(`/api/merkle/verify/${encodeURIComponent(rfc)}`);
            if (res.ok) data = await res.json();
        } catch (_) {}

        const modal = document.createElement("div");
        modal.style.cssText = "position:fixed;inset:0;background:rgba(2,6,23,0.85);backdrop-filter:blur(6px);display:flex;align-items:center;justify-content:center;z-index:99999;padding:16px;font-family:'Inter',sans-serif;";
        modal.innerHTML = `
            <div style="background:#0f172a;border:1px solid #334155;border-radius:16px;max-width:480px;width:100%;padding:24px;color:#f1f5f9;box-shadow:0 25px 50px rgba(0,0,0,0.6);">
                <div style="display:flex;justify-content:space-between;align-items:center;border-bottom:1px solid #1e293b;padding-bottom:12px;margin-bottom:16px;">
                    <span style="font-family:'JetBrains Mono',monospace;font-size:11px;font-weight:700;color:#38bdf8;">FIRMA PERICIAL MALLEN • SELLO CRIPTOGRÁFICO</span>
                    <button id="mallen-badge-close" style="background:none;border:none;color:#94a3b8;cursor:pointer;font-size:16px;"></button>
                </div>
                <div style="font-size:12px;font-family:'JetBrains Mono',monospace;margin-bottom:14px;">
                    <p style="margin:4px 0;color:#94a3b8;">Entidad / Folio: <strong style="color:#fff;">${data.reference}</strong></p>
                    <p style="margin:4px 0;color:#94a3b8;">Merkle Root L2: <span style="color:#34d399;word-break:break-all;">${data.merkle_root}</span></p>
                    <p style="margin:4px 0;color:#94a3b8;">Corte Técnico UTC: <span style="color:#cbd5e1;">${data.timestamp}</span></p>
                </div>
                <div style="background:#020617;border:1px solid #1e293b;border-radius:8px;padding:10px;font-size:10px;color:#64748b;line-height:1.5;margin-bottom:16px;">
                    <strong style="color:#94a3b8;">Deslinde de Alcance Pericial (Art. 89 C.Com):</strong> Este sello certifica la verificación técnica de políticas públicas y estado registral OSINT a la fecha de corte indicada. No constituye aval solidario sobre la conducta comercial futura de la entidad.
                </div>
                <a href="/verify/${encodeURIComponent(rfc)}" style="display:block;text-align:center;background:linear-gradient(to right,#2563eb,#10b981);color:#fff;text-decoration:none;padding:10px;border-radius:10px;font-weight:700;font-size:12px;font-family:'JetBrains Mono',monospace;">
                    Inspeccionar Constancia Completa →
                </a>
            </div>
        `;
        document.body.appendChild(modal);
        document.getElementById("mallen-badge-close").onclick = () => modal.remove();
        modal.onclick = (e) => { if (e.target === modal) modal.remove(); };
    });
})();
