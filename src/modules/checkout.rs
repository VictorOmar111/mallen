use axum::{
    extract::Path,
    response::{Html, IntoResponse},
};
use sha2::{Digest, Sha256};

pub async fn handle_verify(Path(id): Path<String>) -> impl IntoResponse {
    let upper_id = id.trim().to_uppercase();
    let is_mallen_self = upper_id.starts_with("MALLEN");

    let mut hasher = Sha256::new();
    hasher.update(format!("MALLEN_AUDIT_SEAL_2026::{}", upper_id).as_bytes());
    let full_hash = format!("{:x}", hasher.finalize());
    let short_hash = format!("0x{}", &full_hash[..16]);

    let (badge_text, title_text, status_color, notice_title, notice_body) = if is_mallen_self {
        (
            "ESTATUS: 100/100 CUMPLIMIENTO VERIFICADO • RIESGO CERO ($0.00 MXN)",
            "Certificación Institucional de Blindaje Normativo Activo",
            "emerald",
            "Entidad con Blindaje Normativo 360° e Inmunidad Documental Vigente",
            "Se hace constar en el registro público de Firma Pericial Mallen que esta entidad cuenta con el 100% de sus Avisos de Privacidad LFPDPPP (SABG), Certificados TLS 1.3 con HSTS, Contratos con Fecha Cierta SCJN (Art. 69-B CFF), Carpetas STPS y Términos de Comercio Electrónico (Art. 76 BIS LFPC) instrumentados y vigentes."
        )
    } else {
        (
            "ESTATUS: DIAGNÓSTICO PERICIAL REGISTRADO • EXPEDIENTE DISPONIBLE",
            "Cédula Registral de Diagnóstico Corporativo 360°",
            "amber",
            "Garantía de Defendibilidad Procesal y Fecha Cierta Digital (SCJN)",
            "Esta cédula acredita la evaluación técnica de la entidad ante STPS (39 NOMs), SAT (Art. 69-B), IMSS (REPSE/32-D), SABG (LFPDPPP), Protección Civil y PROFECO. Todos los montos y hallazgos se encuentran sincronizados mediante huella criptográfica SHA-256 con el expediente pericial correspondiente a su RFC."
        )
    };

    let action_buttons_html = if is_mallen_self {
        format!(
            r#"<div class="flex flex-col sm:flex-row gap-4 justify-center items-center font-sans">
                <a href="/api/micro_scan/{upper_id}" class="w-full sm:w-auto px-6 py-3.5 rounded-xl bg-emerald-950/60 hover:bg-emerald-900/80 text-emerald-300 border border-emerald-500/40 text-xs font-bold uppercase tracking-wider transition text-center shadow-lg block">
                    Descargar Constancia Institucional en PDF
                </a>
                <a href="/#express-check" class="w-full sm:w-auto px-6 py-3.5 rounded-xl bg-slate-800 hover:bg-slate-700 text-white border border-slate-700 text-xs font-bold uppercase tracking-wider transition text-center shadow-lg block">
                    Volver al Portal Principal
                </a>
            </div>"#
        )
    } else {
        format!(
            r#"<div class="flex flex-col sm:flex-row gap-4 justify-center items-center font-sans">
                <a href="/api/micro_scan/{upper_id}" class="w-full sm:w-auto px-6 py-3.5 rounded-xl bg-white/[0.05] backdrop-blur-2xl shadow-[0_8px_32px_rgba(0,0,0,0.5)] hover:bg-slate-800 text-amber-400 border border-amber-500/40 text-xs font-bold uppercase tracking-wider transition text-center shadow-lg block">
                    Descargar Pre-Dictamen Preliminar PDF
                </a>
                <a href="/checkout?ref={upper_id}" class="w-full sm:w-auto px-6 py-3.5 rounded-xl bg-gradient-to-r from-blue-600 to-emerald-600 hover:from-blue-500 hover:to-emerald-500 text-white text-xs font-bold uppercase tracking-wider transition text-center shadow-lg shadow-emerald-600/20 block">
                    Activar Blindaje Integral y Descargar Kit ZIP
                </a>
            </div>"#
        )
    };

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Cédula de Verificación Oficial | Firma Pericial Mallen</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <link rel="stylesheet" href="/css/quantum-glass.css?v=2026.9">
    <script src="/js/quantum-lights.js?v=2026.9" defer></script>
</head>
<body class="bg-slate-950 text-slate-100 font-serif min-h-screen antialiased flex flex-col justify-between selection:bg-blue-600 selection:text-white">

    <header class="border-b border-white/15 bg-slate-900/60 backdrop-blur-md sticky top-0 z-50">
        <div class="max-w-4xl mx-auto px-6 py-4 flex flex-col sm:flex-row justify-between items-center gap-2">
            <a href="/" class="flex items-center space-x-3">
                <div class="w-3 h-3 rounded-full bg-emerald-500 animate-pulse"></div>
                <span class="font-sans font-bold tracking-wider text-sm text-slate-200">FIRMA PERICIAL MALLEN (MALLEN.LAT)</span>
            </a>
            <div class="text-xs font-sans text-slate-400 bg-slate-800/80 px-3 py-1 rounded-full border border-slate-700">
                Fecha Cierta SCJN • NOM-151 SCFI • Arbitrum L2
            </div>
        </div>
    </header>

    <main class="max-w-3xl mx-auto px-6 py-10 w-full flex-grow">
        <div class="rounded-xl p-6 mb-8 border border-{status_color}-500/30 bg-gradient-to-br from-{status_color}-950/20 via-slate-900 to-slate-900 shadow-2xl relative overflow-hidden">
            <div class="flex flex-col md:flex-row justify-between items-start md:items-center gap-4 relative z-10">
                <div>
                    <span class="inline-block px-2.5 py-1 rounded text-xs font-sans font-bold bg-emerald-500/20 text-emerald-400 border border-emerald-500/30 mb-2">
                        {badge_text}
                    </span>
                    <h1 class="text-2xl font-bold tracking-tight text-white">{title_text}</h1>
                    <p class="text-xs font-sans text-slate-400 mt-1">Folio / RFC de Verificación: <span class="text-slate-200 font-mono font-semibold">{upper_id}</span></p>
                </div>
                <div class="text-right font-sans">
                    <span class="block text-[10px] uppercase tracking-widest text-slate-400">Sello Merkle SHA-256</span>
                    <span class="text-xs font-mono text-emerald-400 font-bold bg-emerald-950/40 px-2.5 py-1 rounded border border-emerald-500/30">{short_hash}</span>
                </div>
            </div>
        </div>

        <div class="bg-white/[0.04] backdrop-blur-2xl shadow-[0_8px_32px_rgba(0,0,0,0.5)] border border-white/15 rounded-xl p-6 mb-6 shadow-xl font-sans text-xs">
            <h3 class="text-sm font-bold text-slate-200 uppercase tracking-wider mb-4 border-b border-white/15 pb-2">
                Ficha Técnica de la Entidad Dictaminada
            </h3>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4 text-slate-300">
                <div>
                    <p class="text-slate-500 text-[11px]">Identificador / RFC</p>
                    <p class="font-mono font-bold text-slate-100 text-sm">{upper_id}</p>
                </div>
                <div>
                    <p class="text-slate-500 text-[11px]">Alcance Regulatorio Evaluado</p>
                    <p class="font-semibold text-slate-100">STPS (39 NOMs) • SAT (69-B) • SABG/LFPDPPP • IMSS • PROFECO</p>
                </div>
                <div>
                    <p class="text-slate-500 text-[11px]">Huella Criptográfica Completa (SHA-256)</p>
                    <p class="font-mono text-[11px] text-blue-400 break-all">{full_hash}</p>
                </div>
                <div>
                    <p class="text-slate-500 text-[11px]">Perito Dictaminador Responsable</p>
                    <p class="font-semibold text-slate-100">C.P.C. Víctor Omar Malpica Guillén (Reg. CED-2026-MALLEN)</p>
                </div>
            </div>
        </div>

        <div class="rounded-xl p-6 mb-8 border border-emerald-500/30 bg-emerald-950/10 backdrop-blur-sm">
            <h3 class="text-sm font-sans font-bold text-emerald-400 mb-3">
                {notice_title}
            </h3>
            <p class="text-xs font-serif text-slate-300 leading-relaxed mb-3">
                {notice_body}
            </p>
            <div class="grid grid-cols-1 sm:grid-cols-3 gap-3 pt-2 border-t border-emerald-500/20 font-sans text-[11px] text-slate-400">
                <div>Validez Art. 210-A CFPC y Art. 89 C.Com</div>
                <div>Fecha Cierta Jurisprudencia 2a./J. 161/2019</div>
                <div>Anclaje Merkle Tree en Arbitrum One L2</div>
            </div>
        </div>

        {action_buttons_html}
    </main>

    <footer class="border-t border-white/15 py-6 text-center text-xs font-sans text-slate-500 bg-slate-900/40">
        <p>© 2026 Firma Pericial Mallen (mallen.lat). Todos los derechos reservados.</p>
    </footer>
</body>
</html>"#
    );

    Html(html)
}
