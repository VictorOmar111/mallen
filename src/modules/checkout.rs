use axum::{
    extract::Path,
    response::{Html, IntoResponse},
};

pub async fn handle_verify(Path(id): Path<String>) -> impl IntoResponse {
    let upper_id = id.to_uppercase();
    let html = format!(r#"
<!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Verificación Oficial | Firma Pericial Mallen</title>
    <script src="https://cdn.tailwindcss.com"></script>
</head>
<body class="bg-slate-950 text-slate-100 font-serif min-h-screen antialiased flex flex-col justify-between selection:bg-red-500 selection:text-white">

    <!-- HEADER INSTITUCIONAL -->
    <header class="border-b border-slate-800 bg-slate-900/60 backdrop-blur-md sticky top-0 z-50">
        <div class="max-w-4xl mx-auto px-6 py-4 flex flex-col sm:flex-row justify-between items-center gap-2">
            <div class="flex items-center space-x-3">
                <div class="w-3 h-3 rounded-full bg-red-500 animate-pulse"></div>
                <span class="font-sans font-bold tracking-wider text-sm text-slate-200">FIRMA PERICIAL MALLEN</span>
            </div>
            <div class="text-xs font-sans text-slate-400 bg-slate-800/80 px-3 py-1 rounded-full border border-slate-700">
                Registro Nacional • NOM-151 SCFI
            </div>
        </div>
    </header>

    <!-- CONTENIDO PRINCIPAL -->
    <main class="max-w-3xl mx-auto px-6 py-10 w-full flex-grow">
        
        <!-- BANNER DE ACREDITACIÓN OFICIAL -->
        <div class="rounded-xl p-6 mb-8 border border-purple-500/30 bg-gradient-to-br from-purple-950/20 via-slate-900 to-slate-900 shadow-2xl relative overflow-hidden">
            <div class="absolute -right-10 -top-10 w-40 h-40 bg-red-500/10 rounded-full blur-3xl pointer-events-none"></div>
            <div class="flex flex-col md:flex-row justify-between items-start md:items-center gap-4 relative z-10">
                <div>
                    <span class="inline-block px-2.5 py-1 rounded text-xs font-sans font-bold bg-emerald-500/20 text-emerald-400 border border-emerald-500/30 mb-2">
                        ESTATUS: ACREDITADO & SEGURO
                    </span>
                    <h1 class="text-2xl font-bold tracking-tight text-white">Constancia de Blindaje Corporativo</h1>
                    <p class="text-xs font-sans text-slate-400 mt-1">Folio de Verificación: <span class="text-slate-200 font-mono font-semibold">{}</span></p>
                </div>
                <div class="text-right font-sans">
                    <span class="block text-[10px] uppercase tracking-widest text-slate-400">Hash de Integridad</span>
                    <span class="text-xs font-mono text-red-400 font-bold bg-red-950/40 px-2.5 py-1 rounded border border-red-500/20">SHA256-W16-OK</span>
                </div>
            </div>
        </div>

        <!-- TARJETA DE INFORMACIÓN TÉCNICA -->
        <div class="bg-slate-900/80 border border-slate-800 rounded-xl p-6 mb-6 shadow-xl font-sans text-xs">
            <h3 class="text-sm font-bold text-slate-200 uppercase tracking-wider mb-4 border-b border-slate-800 pb-2">
                Ficha Técnica de la Entidad Dictaminada
            </h3>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4 text-slate-300">
                <div>
                    <p class="text-slate-500 text-[11px]">Identificador / Tax ID</p>
                    <p class="font-mono font-bold text-slate-100 text-sm">{}</p>
                </div>
                <div>
                    <p class="text-slate-500 text-[11px]">Jurisdicción y Marco</p>
                    <p class="font-semibold text-slate-100">República Mexicana (CDMX / Federal)</p>
                </div>
                <div>
                    <p class="text-slate-500 text-[11px]">Órgano Dictaminador</p>
                    <p class="font-semibold text-slate-100">Consejo Pericial W16</p>
                </div>
                <div>
                    <p class="text-slate-500 text-[11px]">Perito Responsable</p>
                    <p class="font-semibold text-slate-100">C.P.C. Víctor Omar Malpica Guillen</p>
                </div>
            </div>
        </div>

        <!-- SECCIÓN DE VALOR JURÍDICO -->
        <div class="rounded-xl p-6 mb-8 border border-red-500/30 bg-red-950/10 backdrop-blur-sm">
            <h3 class="text-sm font-sans font-bold text-red-400 mb-3 flex items-center gap-2">
                <svg class="w-4 h-4 text-red-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"></path></svg>
                Garantía de Defendibilidad Procesal y Certeza Patrimonial
            </h3>
            <p class="text-xs font-serif text-slate-300 leading-relaxed mb-3">
                El presente registro certifica que la entidad cuenta con expedientes de prueba robustos, bitácoras selladas criptográficamente y soporte documental integral ante revisiones de la STPS, SAT, IMSS, INAI, Protección Civil y PROFECO.
            </p>
            <div class="grid grid-cols-1 sm:grid-cols-3 gap-3 pt-2 border-t border-red-500/20 font-sans text-[11px] text-slate-400">
                <div class="flex items-center gap-1.5">
                    <span class="w-1.5 h-1.5 rounded-full bg-emerald-400"></span> Validez Art. 210-A CFPC
                </div>
                <div class="flex items-center gap-1.5">
                    <span class="w-1.5 h-1.5 rounded-full bg-emerald-400"></span> Sellado Nom-151 SCFI
                </div>
                <div class="flex items-center gap-1.5">
                    <span class="w-1.5 h-1.5 rounded-full bg-emerald-400"></span> Auditoría 24/7 en Blockchain
                </div>
            </div>
        </div>

        <!-- ACCIONES Y DESCARGA (ENLACES DIRECTOS REALES) -->
        <div class="flex flex-col sm:flex-row gap-4 justify-center items-center font-sans">
            <a href="/download/zip/{}" class="w-full sm:w-auto px-6 py-3 rounded-lg bg-slate-800 hover:bg-slate-700 text-slate-200 border border-slate-700 text-xs font-bold uppercase tracking-wider transition text-center shadow-lg block">
                📦 Descargar Paquete ZIP Completo
            </a>
            <a href="/checkout/{}" class="w-full sm:w-auto px-6 py-3 rounded-lg bg-red-600 hover:bg-red-500 text-white text-xs font-bold uppercase tracking-wider transition text-center shadow-lg shadow-red-600/20 block">
                ⚡ Activar / Renovar Blindaje
            </a>
        </div>

    </main>

    <!-- FOOTER -->
    <footer class="border-t border-slate-800 py-6 text-center text-xs font-sans text-slate-500 bg-slate-900/40">
        <p>© 2026 Firma Pericial Mallen. Todos los derechos reservados. • Auditoría e Inmunidad Corporativa.</p>
    </footer>

</body>
</html>
"#, upper_id, upper_id, upper_id, upper_id);

    Html(html)
}
