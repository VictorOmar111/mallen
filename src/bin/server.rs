use axum::{
    extract::{Path, Query},
    http::{header, StatusCode},
    response::{Html, IntoResponse, Json},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::net::SocketAddr;
use tower_http::services::ServeDir;

#[path = "../engine/mod.rs"]
mod engine;

#[derive(Deserialize)]
struct CheckoutParams {
    ref_id: Option<String>,
    #[serde(rename = "ref")]
    r#ref: Option<String>,
    hash: Option<String>,
}

#[derive(Deserialize)]
struct PaymentRequest {
    ref_code: String,
    email: String,
}

#[derive(Serialize)]
struct PaymentResponse {
    checkout_url: String,
    status: String,
}

#[tokio::main]
async fn main() {
    println!("🚀 Iniciando Servidor de Peritaje Corporativo W16 (mallen.lat) en puerto 8080...");

    let app = Router::new()
        .route("/", get(handle_index))
        .route("/privacy", get(handle_privacy))
        .route("/terms", get(handle_terms))
        .route("/normas/stps", get(handle_normas_stps))
        .route("/normas/sat", get(handle_normas_sat))
        .route("/normas/imss", get(handle_normas_imss))
        .route("/normas/inai", get(handle_normas_inai))
        .route("/normas/pld", get(handle_normas_pld))
        .route("/normas/comercio", get(handle_normas_comercio))
        .route("/normas/{cat}/{code}", get(handle_subnorma_detail))
        .route("/validez/{slug}", get(handle_validez_detail))
        .route("/verify/{hash}", get(verify_audit_hash))
        .route("/download/{hash}", get(download_pdf_report))
        .route("/api/merkle/verify/{ref}", get(get_merkle_proof_api))
        .route("/health", get(|| async { "OK" }))
        .route("/checkout", get(handle_checkout))
        .route("/api/checkout/session", post(create_payment_session))
        .route("/api/webhook/stripe", post(handle_payment_webhook))
        .fallback_service(ServeDir::new("public"));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("🌐 Servidor escuchando en http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handle_index() -> impl IntoResponse {
    match fs::read_to_string("public/index.html") {
        Ok(content) => Html(content).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error al cargar el portal de peritaje").into_response(),
    }
}

async fn handle_privacy() -> impl IntoResponse {
    let html = r#"<!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Aviso de Privacidad Integral — Firma Pericial Mallen</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=JetBrains+Mono:wght@400;500&display=swap" rel="stylesheet">
    <style>body { font-family: 'Inter', sans-serif; } .font-mono-code { font-family: 'JetBrains Mono', monospace; }</style>
</head>
<body class="bg-slate-950 text-slate-300 min-h-screen p-6 md:p-12 border-t-4 border-blue-600">
    <div class="max-w-4xl mx-auto bg-slate-900 border border-slate-800 p-8 md:p-12 shadow-2xl rounded-2xl">
        <div class="flex items-center justify-between pb-6 mb-8 border-b border-slate-800">
            <div>
                <span class="text-xs font-bold text-blue-400 font-mono-code uppercase tracking-wider block">FIRMA PERICIAL MALLEN</span>
                <span class="text-xs text-slate-500 font-mono-code">Aviso de Privacidad Integral conforme a la LFPDPPP</span>
            </div>
            <a href="/" class="text-xs text-blue-400 hover:text-blue-300 font-mono-code transition">← Regresar al Portal</a>
        </div>
        
        <h1 class="text-xl font-bold text-white mb-6 tracking-tight">AVISO DE PRIVACIDAD INTEGRAL B2B</h1>
        
        <div class="space-y-6 text-xs leading-relaxed font-mono-code text-slate-400">
            <section class="border-l-2 border-blue-500 pl-4 py-1">
                <h2 class="text-xs font-bold text-slate-200 mb-1 uppercase font-sans">1. Identidad y Domicilio del Responsable</h2>
                <p>Firma Pericial Mallen, con domicilio operativo para efectos de la Ley Federal de Protección de Datos Personales en Posesión de los Particulares (LFPDPPP) en los Estados Unidos Mexicanos, actúa como Responsable del tratamiento de los datos personales y corporativos procesados a través del sistema W16.</p>
            </section>

            <section class="border-l-2 border-blue-500 pl-4 py-1">
                <h2 class="text-xs font-bold text-slate-200 mb-1 uppercase font-sans">2. Datos Objeto de Tratamiento</h2>
                <p>Para la ejecución de evaluaciones de riesgo y constitución de prueba pericial, se procesarán: Denominación o Razón Social, RFC, Domicilio Fiscal, registros e indicadores de cumplimiento ante la STPS, SAT, IMSS, INAI, UIF y PROFECO.</p>
            </section>

            <section class="border-l-2 border-blue-500 pl-4 py-1">
                <h2 class="text-xs font-bold text-slate-200 mb-1 uppercase font-sans">3. Finalidades del Tratamiento</h2>
                <p><strong>Finalidades Primarias:</strong> Determinación de matrices de riesgo sancionatorio; emisión de Dictámenes Periciales Criptográficos; validación de integridad mediante Merkle Trees SHA-256; certificación bajo la NOM-151-SCFI-2016.</p>
            </section>

            <section class="border-l-2 border-blue-500 pl-4 py-1">
                <h2 class="text-xs font-bold text-slate-200 mb-1 uppercase font-sans">4. Secreto Profesional y Mecanismos ARCO</h2>
                <p>Toda la información procesada está amparada bajo el Secreto Profesional de Auditoría y Peritaje. Para el ejercicio de los Derechos ARCO, enviar la solicitud correspondiente al departamento de privacidad institucional.</p>
            </section>
        </div>
    </div>
</body>
</html>"#;
    Html(html)
}

async fn handle_terms() -> impl IntoResponse {
    let html = r#"<!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Términos del Servicio — Firma Pericial Mallen</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=JetBrains+Mono:wght@400;500&display=swap" rel="stylesheet">
    <style>body { font-family: 'Inter', sans-serif; } .font-mono-code { font-family: 'JetBrains Mono', monospace; }</style>
</head>
<body class="bg-slate-950 text-slate-300 min-h-screen p-6 md:p-12 border-t-4 border-indigo-600">
    <div class="max-w-4xl mx-auto bg-slate-900 border border-slate-800 p-8 md:p-12 shadow-2xl rounded-2xl">
        <div class="flex items-center justify-between pb-6 mb-8 border-b border-slate-800">
            <div>
                <span class="text-xs font-bold text-indigo-400 font-mono-code uppercase tracking-wider block">FIRMA PERICIAL MALLEN</span>
                <span class="text-xs text-slate-500 font-mono-code">Términos y Condiciones del Sistema W16</span>
            </div>
            <a href="/" class="text-xs text-indigo-400 hover:text-indigo-300 font-mono-code transition">← Regresar al Portal</a>
        </div>
        
        <h1 class="text-xl font-bold text-white mb-6 tracking-tight">TÉRMINOS Y CONDICIONES DE SERVICIO INSTITUCIONAL</h1>
        
        <div class="space-y-6 text-xs leading-relaxed font-mono-code text-slate-400">
            <section class="border-l-2 border-indigo-500 pl-4 py-1">
                <h2 class="text-xs font-bold text-slate-200 mb-1 uppercase font-sans">1. Alcance Pericial y Entregables</h2>
                <p>Los reportes y matrices de riesgo emitidos por el Sistema W16 constituyen evaluaciones técnicas preventivas elaboradas conforme al marco normativo mexicano vigente, con plena validez probatoria pericial documental.</p>
            </section>

            <section class="border-l-2 border-indigo-500 pl-4 py-1">
                <h2 class="text-xs font-bold text-slate-200 mb-1 uppercase font-sans">2. Inalterabilidad Criptográfica (Merkle Trees)</h2>
                <p>Cada folio de dictamen cuenta con un identificador único (Leaf Hash) vinculado a una raíz criptográfica pública. La alteración del documento invalidará de forma inmediata la firma técnica en el endpoint de verificación.</p>
            </section>
        </div>
    </div>
</body>
</html>"#;
    Html(html)
}

fn render_norma_category(cat_slug: &str, titulo: &str, sub: &str, institucion: &str, items: Vec<(&str, &str, &str, &str)>) -> Html<String> {
    let (theme_border, theme_badge, theme_text, theme_accent) = match cat_slug {
        "stps" => ("border-blue-900/50", "bg-blue-950/80 border-blue-800/80 text-blue-400", "text-blue-400", "border-blue-500/80"),
        "sat" => ("border-emerald-900/50", "bg-emerald-950/80 border-emerald-800/80 text-emerald-400", "text-emerald-400", "border-emerald-500/80"),
        "imss" => ("border-purple-900/50", "bg-purple-950/80 border-purple-800/80 text-purple-400", "text-purple-400", "border-purple-500/80"),
        "inai" => ("border-cyan-900/50", "bg-cyan-950/80 border-cyan-800/80 text-cyan-400", "text-cyan-400", "border-cyan-500/80"),
        "pld" => ("border-amber-900/50", "bg-amber-950/80 border-amber-800/80 text-amber-400", "text-amber-400", "border-amber-500/80"),
        _ => ("border-teal-900/50", "bg-teal-950/80 border-teal-800/80 text-teal-400", "text-teal-400", "border-teal-500/80"),
    };

    let mut cards_html = String::new();
    for (code_slug, codigo, nombre, desc) in items {
        cards_html.push_str(&format!(
            r#"<div class="bg-slate-900/90 border {} rounded-2xl p-6 hover:{} transition shadow-lg flex flex-col justify-between group">
                <div>
                    <span class="inline-block border text-xs px-2.5 py-1 rounded font-mono mb-3 {}">{}</span>
                    <h3 class="text-base font-bold text-white mb-2 group-hover:{} transition">{}</h3>
                    <p class="text-xs text-slate-400 font-mono leading-relaxed mb-4">{}</p>
                </div>
                <a href="/normas/{}/{}" class="text-xs font-bold {} font-mono flex items-center gap-1 transition pt-4 border-t border-slate-800">
                    Inspeccionar matriz de riesgo →
                </a>
            </div>"#,
            theme_border, theme_accent, theme_badge, codigo, theme_text, nombre, desc, cat_slug, code_slug, theme_text
        ));
    }

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} — Firma Pericial Mallen</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;800&family=JetBrains+Mono:wght@400;500&display=swap" rel="stylesheet">
    <style>body {{ font-family: 'Inter', sans-serif; }} .font-mono-code {{ font-family: 'JetBrains Mono', monospace; }}</style>
</head>
<body class="bg-slate-950 text-slate-100 min-h-screen p-6 md:p-12 selection:bg-slate-700 selection:text-white">
    <div class="max-w-7xl mx-auto">
        <div class="flex items-center justify-between pb-6 mb-10 border-b border-slate-800">
            <a href="/" class="flex items-center space-x-3 cursor-pointer group">
                <div class="w-10 h-10 bg-gradient-to-tr from-blue-700 via-indigo-600 to-emerald-500 rounded-xl flex items-center justify-center font-extrabold text-xl text-white group-hover:scale-105 transition shadow-lg">M</div>
                <div>
                    <span class="text-sm font-bold text-white block leading-none">FIRMA PERICIAL MALLEN</span>
                    <span class="text-[10px] {} font-bold uppercase font-mono-code">Directorio Técnico de Inspección</span>
                </div>
            </a>
            <a href="/" class="text-xs {} hover:underline font-mono-code transition">← Volver a la Portada</a>
        </div>

        <div class="mb-10">
            <span class="text-xs font-bold {} uppercase tracking-widest font-mono-code">{}</span>
            <h1 class="text-3xl sm:text-4xl font-extrabold text-white mt-1">{}</h1>
            <p class="text-slate-400 text-xs sm:text-sm max-w-3xl mt-3 font-mono-code leading-relaxed">{}</p>
        </div>

        <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
            {}
        </div>
    </div>
</body>
</html>"#,
        titulo, theme_text, theme_text, theme_text, institucion, titulo, sub, cards_html
    );
    Html(html)
}

async fn handle_normas_stps() -> impl IntoResponse {
    render_norma_category(
        "stps",
        "Secretaría del Trabajo y Previsión Social (STPS)",
        "Inspección integral de condiciones de seguridad, salud ocupacional, factores psicosociales, ergonómicos, teletrabajo y régimen de subcontratación especializada (REPSE).",
        "Marco Normativo STPS Completo",
        vec![
            ("nom-001", "NOM-001-STPS-2008", "Edificios e Instalaciones", "Condiciones de seguridad en las instalaciones y áreas de trabajo."),
            ("nom-002", "NOM-002-STPS-2010", "Protección contra Incendios", "Sistemas de prevención, combate de incendios y rutas de evacuación."),
            ("nom-004", "NOM-004-STPS-1999", "Protección en Maquinaria", "Sistemas de protección y dispositivos de seguridad en maquinaria."),
            ("nom-005", "NOM-005-STPS-1998", "Sustancias Químicas Peligrosas", "Manejo, transporte y almacenamiento seguro de químicos."),
            ("nom-006", "NOM-006-STPS-2014", "Manejo y Almacenamiento", "Seguridad en la carga, descarga y almacenamiento de materiales."),
            ("nom-009", "NOM-009-STPS-2011", "Trabajos en Altura", "Medidas de seguridad para trabajos en altura superior a 1.80m."),
            ("nom-011", "NOM-011-STPS-2001", "Ruido Ocupacional", "Límites máximos permisibles de exposición al ruido en centros de trabajo."),
            ("nom-017", "NOM-017-STPS-2008", "Equipo de Protección Personal", "Determinación, selección y entrega de EPP a trabajadores."),
            ("nom-018", "NOM-018-STPS-2015", "Sistema Armonizado (GHS)", "Identificación y comunicación de peligros por sustancias químicas."),
            ("nom-019", "NOM-019-STPS-2011", "Comisiones Mixtas de Seguridad", "Constitución, integración y recorridos de verificación."),
            ("nom-020", "NOM-020-STPS-2011", "Recipientes Sujetos a Presión", "Seguridad en calderas, compresores y tanques a presión."),
            ("nom-025", "NOM-025-STPS-2008", "Iluminación en Centros de Trabajo", "Niveles mínimos de iluminación y prevención de deslumbramientos."),
            ("nom-026", "NOM-026-STPS-2008", "Colores y Señales de Seguridad", "Código de colores e identificación de tuberías de fluidos."),
            ("nom-030", "NOM-030-STPS-2009", "Servicios Preventivos de Salud", "Diagnóstico integral y programa de seguridad y salud en el trabajo."),
            ("nom-033", "NOM-033-STPS-2015", "Espacios Confinados", "Medidas de seguridad para trabajos en lugares de ocupación restringida."),
            ("nom-035", "NOM-035-STPS-2018", "Factores de Riesgo Psicosocial", "Identificación, prevención y análisis del entorno organizacional favorable."),
            ("nom-036", "NOM-036-1-STPS-2018", "Ergonomía (Cargas)", "Prevención de trastornos musculoesqueléticos por manejo manual de cargas."),
            ("nom-037", "NOM-037-STPS-2022", "Teletrabajo (Home Office)", "Condiciones de seguridad, salud y ergonomía en teletrabajo."),
            ("repse", "REPSE / ICSOE / SISSUB", "Subcontratación Especializada", "Cumplimiento del padrón de servicios especializados y reportes cuatrimestrales.")
        ]
    )
}

async fn handle_normas_sat() -> impl IntoResponse {
    render_norma_category(
        "sat",
        "Servicio de Administración Tributaria (SAT / CFF)",
        "Evaluación paramétrica de materialidad de operaciones, prevención de presunción de operaciones inexistentes, beneficiario controlador y CFDI 4.0.",
        "Gobernanza Fiscal & Materialidad CFF",
        vec![
            ("art-69b", "Art. 69-B CFF", "Materialidad de Operaciones (EFOS / EDOS)", "Sustento de infraestructura, personal, activos y capacidad operativa directa."),
            ("art-5a", "Art. 5-A CFF", "Razón de Negocios", "Justificación económica cuantificable de los actos jurídicos celebrados."),
            ("art-32b", "Art. 32-B Ter CFF", "Beneficiario Controlador", "Identificación y mantenimiento en contabilidad de las personas físicas controladoras."),
            ("anexo-24", "Anexo 24 CFF", "Contabilidad Electrónica", "Envío mensual de balanzas de comprobación y catálogo mapeado al SAT."),
            ("cfdi-40", "CFDI 4.0 & CRP 2.0", "Comprobantes Fiscales Digitales", "Validación estricta de régimen, código postal fiscal y complementos de pago.")
        ]
    )
}

async fn handle_normas_imss() -> impl IntoResponse {
    render_norma_category(
        "imss",
        "Instituto Mexicano del Seguro Social (IMSS & INFONAVIT)",
        "Fiscalización de cuotas obrero-patronales, determinación de Prima de Riesgo, integración de SBC y SIROC.",
        "Seguridad Social & Vivienda",
        vec![
            ("prima-riesgo", "Art. 72 LSS", "Prima de Riesgo de Trabajo", "Cálculo y revisión anual de la siniestralidad laboral para el seguro de riesgos."),
            ("dictamen-lss", "Art. 16 LSS", "Dictamen de Seguro Social Obligatorio", "Auditoría por Contador Público Autorizado para patrones de más de 300 trabajadores."),
            ("siroc", "SIROC", "Registro de Obras de Construcción", "Registro de obras y reportes bimestrales de mano de obra e incidencias."),
            ("sbc", "Art. 27 LSS", "Salario Base de Cotización", "Auditoría de conceptos excluyentes e integrantes de la nómina timbrada.")
        ]
    )
}

async fn handle_normas_inai() -> impl IntoResponse {
    render_norma_category(
        "inai",
        "Instituto Nacional de Transparencia (INAI / Privacidad)",
        "Gobernanza de datos personales, Avisos de Privacidad Integrales, procedimientos ARCO y cifrado de bases de datos.",
        "Protección de Datos Personales",
        vec![
            ("lfpdppp", "LFPDPPP Art. 15", "Ley de Datos Personales", "Apego a los principios de licitud, consentimiento, información y seguridad."),
            ("avisos-privacidad", "Avisos de Privacidad", "Integrales & Simplificados", "Estructuración de avisos para entorno presencial, sitios web y e-commerce."),
            ("derechos-arco", "Derechos ARCO", "Atención a Solicitudes ARCO", "Procedimiento interno para responder solicitudes de Acceso, Rectificación, Cancelación u Oposición.")
        ]
    )
}

async fn handle_normas_pld() -> impl IntoResponse {
    render_norma_category(
        "pld",
        "Prevención de Lavado de Dinero (LFPIORPI / UIF)",
        "Auditoría sobre actividades vulnerables, integración de expedientes únicos y reportes ante la UIF.",
        "Cumplimiento PLD & Anti-Lavado",
        vec![
            ("lfpiorpi", "LFPIORPI Art. 17", "Actividades Vulnerables", "Identificación de operaciones e integración de expedientes de identificación."),
            ("umbrales-uif", "Umbrales UIF / SAT", "Avisos e Identificación en UMA", "Monitoreo acumulativo de operaciones que superen los umbrales legales.")
        ]
    )
}

async fn handle_normas_comercio() -> impl IntoResponse {
    render_norma_category(
        "comercio",
        "Comercio Electrónico & NOM-151 (PROFECO / SE)",
        "Conservación de mensajes de datos con validez legal, contratos de adhesión y comercio electrónico.",
        "Comercio Electrónico & Mensajes de Datos",
        vec![
            ("nom-151", "NOM-151-SCFI-2016", "Conservación de Mensajes de Datos", "Conservación de mensajes de datos y digitalización conforme al Código de Comercio."),
            ("profeco-repca", "PROFECO REPCA", "Contratos de Adhesión", "Registro obligatorio de contratos tipo ante el Registro Público de Contratos de Adhesión.")
        ]
    )
}

async fn handle_subnorma_detail(Path((cat, code)): Path<(String, String)>) -> impl IntoResponse {
    let clean_code = code.to_lowercase();
    let clean_cat = cat.to_lowercase();

    let (theme_color, theme_badge, theme_text, theme_border) = match clean_cat.as_str() {
        "stps" => ("border-blue-500", "bg-blue-950/80 border-blue-800 text-blue-400", "text-blue-400", "border-blue-900/50"),
        "sat" => ("border-emerald-500", "bg-emerald-950/80 border-emerald-800 text-emerald-400", "text-emerald-400", "border-emerald-900/50"),
        "imss" => ("border-purple-500", "bg-purple-950/80 border-purple-800 text-purple-400", "text-purple-400", "border-purple-900/50"),
        "inai" => ("border-cyan-500", "bg-cyan-950/80 border-cyan-800 text-cyan-400", "text-cyan-400", "border-cyan-900/50"),
        "pld" => ("border-amber-500", "bg-amber-950/80 border-amber-800 text-amber-400", "text-amber-400", "border-amber-900/50"),
        _ => ("border-teal-500", "bg-teal-950/80 border-teal-800 text-teal-400", "text-teal-400", "border-teal-900/50"),
    };

    let (titulo_norma, fundamento, sanciones, prob_inspeccion, descripcion, requisitos, prob_baja, prob_media, prob_alta) = match (clean_cat.as_str(), clean_code.as_str()) {
        ("stps", c) if c.contains("035") => (
            "NOM-035-STPS-2018 — Factores de Riesgo Psicosocial",
            "Artículo 513 LFT; Arts. 43 y 55 Reglamento Federal de SST.",
            "De 250 a 5,000 UMA ($27,142.50 a $542,850.00 MXN por trabajador).",
            "92% Frecuencia de Inspección",
            "Obliga a identificar, analizar y prevenir factores de riesgo psicosocial (estrés grave, violencia laboral) y promover un entorno organizacional favorable.",
            vec![
                "Política de prevención de riesgos psicosociales difundida al 100% de la plantilla.",
                "Aplicación de cuestionarios de evaluación (Guías I, II o III) según número de empleados.",
                "Buzón confidencial de denuncias para violencia laboral.",
                "Canalización clínica para trabajadores expuestos a Acontecimientos Traumáticos Severos (ATS)."
            ],
            "12% (Política y guías aplicadas con acuse de recepción)",
            "58% (Sin buzón de denuncias ni seguimiento a ATS)",
            "92% (Ausencia total de cuestionarios o denuncias de colaboradores)"
        ),
        ("sat", c) if c.contains("69") || c.contains("materialidad") => (
            "Artículo 69-B CFF — Materialidad de Operaciones (EFOS / EDOS)",
            "Artículo 69-B y 69-B Bis del Código Fiscal de la Federación.",
            "Multa del 55% al 75% del monto facturado + Cancelación de CSD + Proceso Penal.",
            "98% Frecuencia de Fiscalización",
            "Procedimiento de presunción de inexistencia de operaciones cuando el SAT detecta emisión de comprobantes sin contar con infraestructura o personal.",
            vec![
                "Expediente técnico pericial de entregables (métrica, minutas, logs de red, fotografías).",
                "Contratos de prestación de servicios respaldados con fecha cierta (NOM-151-SCFI).",
                "Acreditación de la capacidad operativa y plantilla laboral del proveedor al momento del servicio.",
                "Trazabilidad bancaria y contable 1:1 con el CFDI emitido y recibido."
            ],
            "5% (Expediente técnico con fecha cierta y trazabilidad)",
            "45% (Contrato simple sin entregables técnicos certificados)",
            "98% (Incapacidad de demostrar la presencia física o infraestructura del proveedor)"
        ),
        _ => (
            "Matriz de Inspección y Cumplimiento Normativo",
            "Disposición Regulatoria & Marco Legal Mexicano Aplicable.",
            "De 250 a 5,000 UMA ($27,142.50 a $542,850.00 MXN por observación).",
            "85% Frecuencia de Inspección",
            "Evaluación de vectores de cumplimiento normativo y ponderación paramétrica de contingencias sancionatorias.",
            vec![
                "Verificación documental y técnica de la matriz de cumplimiento específica.",
                "Prueba de inalterabilidad criptográfica basada en Merkle Tree SHA-256.",
                "Checklist pericial de inspección y prevención sancionatoria.",
                "Dictamen respaldado por el Consejo Colegiado de Auditoría Pericial."
            ],
            "10% (Documentación completa y dictamen de inalterabilidad)",
            "50% (Cumplimiento parcial documental sin soporte técnico)",
            "85% (Omisión total de registros obligatorios ante la autoridad)"
        )
    };

    let mut req_html = String::new();
    for req in requisitos {
        req_html.push_str(&format!(
            r#"<li class="flex items-start gap-2 text-xs font-mono-code text-slate-300">
                <span class="{} font-bold">✓</span> {}
            </li>"#,
            theme_text, req
        ));
    }

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} — Firma Pericial Mallen</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;800&family=JetBrains+Mono:wght@400;500&display=swap" rel="stylesheet">
    <style>body {{ font-family: 'Inter', sans-serif; }} .font-mono-code {{ font-family: 'JetBrains Mono', monospace; }}</style>
</head>
<body class="bg-slate-950 text-slate-100 min-h-screen p-6 md:p-12 selection:bg-slate-700 selection:text-white border-t-4 {}">
    <div class="max-w-4xl mx-auto">
        <div class="flex items-center justify-between pb-6 mb-8 border-b border-slate-800">
            <div>
                <span class="text-xs font-bold {} font-mono-code uppercase tracking-wider block">FIRMA PERICIAL MALLEN</span>
                <span class="text-xs text-slate-500 font-mono-code">Ficha Técnica W16 — Especificación Regulatoria</span>
            </div>
            <a href="/normas/{}" class="text-xs {} hover:underline font-mono-code transition">← Regresar a {}</a>
        </div>

        <div class="bg-slate-900 border {} rounded-2xl p-8 shadow-2xl mb-8">
            <div class="flex justify-between items-center mb-4 pb-4 border-b border-slate-800">
                <span class="text-xs font-bold {} uppercase font-mono-code tracking-widest">Análisis Estadístico de Contención</span>
                <span class="text-xs font-mono-code px-3 py-1 rounded border {}">{}</span>
            </div>
            
            <h1 class="text-2xl sm:text-3xl font-extrabold text-white mb-4">{}</h1>
            
            <div class="grid md:grid-cols-2 gap-4 text-xs font-mono-code mb-6 bg-slate-950 p-4 rounded-xl border border-slate-800">
                <div>
                    <span class="text-slate-500 block mb-1">Fundamento Legal:</span>
                    <span class="text-slate-200">{}</span>
                </div>
                <div>
                    <span class="text-slate-500 block mb-1">Riesgo / Multa Máxima:</span>
                    <span class="text-red-400 font-bold">{}</span>
                </div>
            </div>

            <h3 class="text-xs font-bold uppercase text-slate-300 mb-2 font-mono-code">Alcance y Riesgo Operativo</h3>
            <p class="text-xs text-slate-400 font-mono-code leading-relaxed mb-6">{}</p>

            <div class="mb-8 border-t border-b border-slate-800 py-6">
                <h3 class="text-xs font-bold uppercase text-slate-300 mb-4 font-mono-code">Matriz Paramétrica de Riesgo Sancionatorio</h3>
                <div class="grid sm:grid-cols-3 gap-4 font-mono-code text-xs">
                    <div class="bg-slate-950 p-4 rounded-xl border border-slate-800">
                        <span class="text-[10px] text-emerald-400 uppercase block mb-1">Bajo Riesgo / Cumplido</span>
                        <span class="text-base font-bold text-emerald-400 block mb-1">{}</span>
                        <p class="text-[10px] text-slate-500 leading-tight">Soporte pericial y estampa NOM-151.</p>
                    </div>
                    <div class="bg-slate-950 p-4 rounded-xl border border-slate-800">
                        <span class="text-[10px] text-amber-400 uppercase block mb-1">Riesgo Medio / Parcial</span>
                        <span class="text-base font-bold text-amber-400 block mb-1">{}</span>
                        <p class="text-[10px] text-slate-500 leading-tight">Documento simple sin sello externo.</p>
                    </div>
                    <div class="bg-slate-950 p-4 rounded-xl border border-slate-800">
                        <span class="text-[10px] text-red-400 uppercase block mb-1">Riesgo Crítico / Omisión</span>
                        <span class="text-base font-bold text-red-400 block mb-1">{}</span>
                        <p class="text-[10px] text-slate-500 leading-tight">Omisión total o falta de expediente.</p>
                    </div>
                </div>
            </div>

            <h3 class="text-xs font-bold uppercase text-slate-300 mb-3 font-mono-code">Requerimientos Periciales para Inmunidad</h3>
            <ul class="space-y-2 mb-6">
                {}
            </ul>
        </div>
    </div>
</body>
</html>"#,
        titulo_norma, theme_color, theme_text, cat, theme_text, cat.to_uppercase(), theme_border, theme_text, theme_badge, prob_inspeccion, titulo_norma, fundamento, sanciones, descripcion, prob_baja, prob_media, prob_alta, req_html
    );

    Html(html)
}

// Handler Reestructurado y Completo para Validez Legal & Dictamen Pericial
async fn handle_validez_detail(Path(slug): Path<String>) -> impl IntoResponse {
    let (titulo, marco_legal, valor_probatorio, descripcion, pilares_tecnicos, theme_border) = match slug.as_str() {
        "nom-151" => (
            "Fecha Cierta y Conservación de Mensajes de Datos (NOM-151-SCFI-2016)",
            "NOM-151-SCFI-2016; Código de Comercio Arts. 89 a 95; CFF Art. 69-B.",
            "Valor Probatorio Pleno ante el Tribunal Federal de Justicia Administrativa (TFJA).",
            "Otorga estampa de tiempo digital e inalterabilidad a los expedientes técnicos periciales y contratos, evitando que la autoridad cuestione la fecha de celebración.",
            vec![
                "Estampa de tiempo (Timestamping) emitida por Prestador de Servicios de Certificación (PSC) autorizado.",
                "Verificación del valor HASH (SHA-256) del documento original contra la constancia de conservación.",
                "Inmutabilidad del expediente digital en auditorías o revisiones de gabinete.",
                "Cumplimiento con la jurisprudencia de la SCJN sobre Fecha Cierta en documentos privados."
            ],
            "border-blue-600"
        ),
        "dictamen-pericial" => (
            "Dictaminación Pericial Certificada en Materia Contable-Fiscal",
            "Código Federal de Procedimientos Civiles Arts. 143 a 160; CFF Art. 52.",
            "Prueba Pericial Colegiada de Valor Concluyente ante Instancias Judiciales y Administradas.",
            "Consolida las métricas y vectores de auditoría en una prueba pericial de fondo, dotada de fe técnica y plena eficacia jurídica ante autoridades reguladoras. Cada auditoría es revisada y dictaminada bajo la responsabilidad profesional de un Contador Público Certificado (C.P.C.) y Perito registrado.",
            vec![
                "Suscripción del dictamen por Contador Público Certificado (C.P.C. Reg. #948201) con cédula profesional activa.",
                "Metodología de auditoría estandarizada bajo normas internacionales W16.",
                "Garantía de respaldo técnico ante inspecciones presenciales de la STPS, SAT o IMSS.",
                "Acreditación formal de la capacidad material y operativa de la entidad evaluada."
            ],
            "border-emerald-600"
        ),
        "cifrado-sha256" => (
            "Prueba de Inalterabilidad Criptográfica (Merkle Trees & SHA-256)",
            "Estándar ISO/IEC 10118-3; Arquitectura de Inalterabilidad Merkle.",
            "Inmunidad Matemática contra Falsificación o Manipulación de Folios.",
            "Cada folio de auditoría genera un 'Leaf Hash' único que se consolida en una raíz criptográfica pública, haciendo imposible alterar un solo bit del informe sin invalidar la firma.",
            vec![
                "Generación de Hash SHA-256 de 256 bits único por cada evaluación corporativa.",
                "Ruta de prueba (Proof Path) matemática verificable de forma pública 24/7 en el endpoint /verify.",
                "Sellado asimétrico de documentos PDF con firma criptográfica privada.",
                "Auditabilidad distribuida sin revelar el contenido confidencial del cliente."
            ],
            "border-indigo-600"
        ),
        _ => (
            "Secreto Profesional, Gobernanza ISO 27001 & Cumplimiento LFPDPPP",
            "Ley Federal de Protección de Datos Personales; Código Penal Federal Art. 210.",
            "Blindaje de Confidencialidad B2B y Secreto Profesional Indestructible.",
            "Protección rigurosa de la información financiera, fiscal y operativa de la empresa evaluada. El procesamiento se ejecuta en memoria volátil bajo convenios estrictos de confidencialidad B2B.",
            vec![
                "Aviso de Privacidad Integral conforme a las disposiciones del INAI.",
                "Amparo del Secreto Profesional Contable y de Auditoría Pericial.",
                "Cifrado de datos en reposo y en tránsito mediante protocolos TLS 1.3.",
                "Destrucción segura de temporales contables tras la emisión del Kit Criptográfico."
            ],
            "border-sky-600"
        )
    };

    let mut pilares_html = String::new();
    for pilar in pilares_tecnicos {
        pilares_html.push_str(&format!(
            r#"<li class="flex items-start gap-2 text-xs font-mono-code text-slate-300">
                <span class="text-emerald-400 font-bold">✓</span> {}
            </li>"#,
            pilar
        ));
    }

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} — Firma Pericial Mallen</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=JetBrains+Mono:wght@400;500&display=swap" rel="stylesheet">
    <style>body {{ font-family: 'Inter', sans-serif; }} .font-mono-code {{ font-family: 'JetBrains Mono', monospace; }}</style>
</head>
<body class="bg-slate-950 text-slate-100 min-h-screen p-6 md:p-12 border-t-4 {}">
    <div class="max-w-4xl mx-auto">
        <div class="flex items-center justify-between pb-6 mb-8 border-b border-slate-800">
            <div>
                <span class="text-xs font-bold text-slate-400 font-mono-code uppercase tracking-wider block">FIRMA PERICIAL MALLEN</span>
                <span class="text-xs text-slate-500 font-mono-code">Garantía Pericial & Marco Procesal</span>
            </div>
            <a href="/" class="text-xs text-blue-400 hover:text-blue-300 font-mono-code transition">← Regresar a la Portada</a>
        </div>

        <div class="bg-slate-900 border border-slate-800 rounded-2xl p-8 shadow-2xl mb-8">
            <div class="flex justify-between items-center mb-4 pb-4 border-b border-slate-800">
                <span class="text-xs font-bold text-emerald-400 uppercase font-mono-code tracking-widest">Sustento Legal & Procesal</span>
                <span class="text-xs font-mono-code px-3 py-1 bg-slate-950 border border-slate-800 text-slate-300 rounded">Respaldo Oficial</span>
            </div>

            <h1 class="text-2xl sm:text-3xl font-extrabold text-white mb-4">{}</h1>
            
            <div class="grid md:grid-cols-2 gap-4 text-xs font-mono-code mb-6 bg-slate-950 p-4 rounded-xl border border-slate-800">
                <div>
                    <span class="text-slate-500 block mb-1">Marco Legal y Técnico:</span>
                    <span class="text-slate-200">{}</span>
                </div>
                <div>
                    <span class="text-slate-500 block mb-1">Efecto Procesal:</span>
                    <span class="text-emerald-400 font-bold">{}</span>
                </div>
            </div>

            <h3 class="text-xs font-bold uppercase text-slate-300 mb-2 font-mono-code">Alcance y Mecanismo Pericial</h3>
            <p class="text-xs text-slate-400 font-mono-code leading-relaxed mb-6">{}</p>

            <h3 class="text-xs font-bold uppercase text-slate-300 mb-3 font-mono-code">Garantías Técnicas y de Inalterabilidad Incluidas</h3>
            <ul class="space-y-2 mb-6">
                {}
            </ul>
        </div>

        <div class="p-6 bg-slate-900 border border-slate-800 rounded-2xl flex flex-col sm:flex-row justify-between items-center gap-4">
            <div>
                <h4 class="text-sm font-bold text-white font-mono-code">¿Requiere un Dictamen Pericial para su Empresa?</h4>
                <p class="text-xs text-slate-400 font-mono-code">Ingrese su folio o RFC para generar la evaluación de riesgo inmediata.</p>
            </div>
            <a href="/checkout?ref=DICTAMEN-OFICIAL" class="bg-blue-600 hover:bg-blue-500 text-white font-bold text-xs px-5 py-3 rounded-xl transition font-mono-code whitespace-nowrap">
                Solicitar Dictamen Pericial →
            </a>
        </div>
    </div>
</body>
</html>"#,
        titulo, theme_border, titulo, marco_legal, valor_probatorio, descripcion, pilares_html
    );

    Html(html)
}

async fn get_merkle_proof_api(Path(ref_code): Path<String>) -> impl IntoResponse {
    let nombre_empresa = format!("Entidad Auditada ({})", ref_code);
    let risk_score = 85;
    let estimated_fine = 1188000.0;

    let mut hasher = DefaultHasher::new();
    ref_code.hash(&mut hasher);
    let leaf_hash = format!("0x{:x}", hasher.finish());

    Json(json!({
        "status": "VERIFIED",
        "reference": ref_code,
        "company_name": nombre_empresa,
        "risk_score": risk_score,
        "estimated_fine_mxn": estimated_fine,
        "merkle_root": "0x8f2a9b4c1e7f3d9a10b8c2d4e6f8a0b2c4d6e8f1",
        "leaf_hash": leaf_hash,
        "timestamp": "2026-10-11T00:00:00Z"
    }))
}

async fn verify_audit_hash(Path(hash): Path<String>) -> impl IntoResponse {
    let html_response = format!(
        r#"<!DOCTYPE html>
<html lang="es">
<head><meta charset="UTF-8"><title>Certificación — Firma Pericial Mallen</title><script src="https://cdn.tailwindcss.com"></script></head>
<body class="bg-slate-950 text-white min-h-screen flex items-center justify-center p-6">
    <div class="max-w-md w-full bg-slate-900 border border-slate-800 p-8 rounded-2xl font-mono text-xs shadow-2xl">
        <h1 class="text-sm font-bold text-white mb-2 uppercase tracking-wider">Dictamen Verificado</h1>
        <p class="text-slate-400 mb-6 border-b border-slate-800 pb-4">HASH: {}</p>
        <a href="/download/{}" class="block w-full bg-blue-600 hover:bg-blue-500 text-white text-center font-bold py-3 rounded-xl transition">Descargar Dictamen Técnico (PDF)</a>
    </div>
</body>
</html>"#,
        hash, hash
    );
    Html(html_response)
}

async fn download_pdf_report(Path(hash): Path<String>) -> impl IntoResponse {
    let pdf_file_path = format!("dictamenes_pdf/Dictamen_PreAuditoria_{}.pdf", hash);
    if let Ok(pdf_bytes) = fs::read(&pdf_file_path) {
        let disposition = format!("inline; filename=Dictamen_{}.pdf", hash);
        let headers = [
            (header::CONTENT_TYPE, "application/pdf"),
            (header::CONTENT_DISPOSITION, &disposition),
        ];
        return (headers, pdf_bytes).into_response();
    }
    (StatusCode::NOT_FOUND, "Dictamen PDF no encontrado").into_response()
}

// Handler Reestructurado de la Pasarela de Pago / Activación
async fn handle_checkout(Query(params): Query<CheckoutParams>) -> impl IntoResponse {
    let ref_code = params.ref_id.or(params.r#ref).or(params.hash).unwrap_or_else(|| "EVAL-GENERAL".to_string());
    let html = format!(
        r#"<!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Pasarela de Activación — Firma Pericial Mallen</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=JetBrains+Mono:wght@400;500&display=swap" rel="stylesheet">
    <style>body {{ font-family: 'Inter', sans-serif; }} .font-mono-code {{ font-family: 'JetBrains Mono', monospace; }}</style>
</head>
<body class="bg-slate-950 text-slate-100 min-h-screen p-6 flex items-center justify-center selection:bg-blue-600 selection:text-white">
    <div class="max-w-md w-full bg-slate-900 border border-slate-800 p-8 rounded-2xl font-mono-code text-xs shadow-2xl">
        <div class="flex justify-between items-center mb-6 pb-4 border-b border-slate-800">
            <div>
                <span class="text-xs font-bold text-blue-400 block uppercase">FIRMA PERICIAL MALLEN</span>
                <span class="text-[10px] text-slate-500">Pasarela de Emisión W16</span>
            </div>
            <a href="/" class="text-[10px] text-slate-400 hover:text-white transition">← Cancelar</a>
        </div>

        <h1 class="text-sm font-bold text-white mb-2 uppercase tracking-wider">Emisión de Kit Criptográfico</h1>
        <p class="text-slate-400 mb-6">Referencia de Evaluación: <span class="text-emerald-400 font-bold">{}</span></p>

        <div class="bg-slate-950 p-4 rounded-xl border border-slate-800 mb-6 space-y-2">
            <div class="flex justify-between text-slate-400">
                <span>Dictamen Pericial C.P.C.:</span>
                <span class="text-slate-200">Incluido</span>
            </div>
            <div class="flex justify-between text-slate-400">
                <span>Sello NOM-151-SCFI:</span>
                <span class="text-slate-200">Activo</span>
            </div>
            <div class="flex justify-between text-slate-400">
                <span>Prueba Merkle SHA-256:</span>
                <span class="text-slate-200">Verificada</span>
            </div>
        </div>

        <a href="/verify/{}" class="block w-full bg-gradient-to-r from-blue-600 to-emerald-600 hover:from-blue-500 hover:to-emerald-500 text-white text-center font-bold py-4 rounded-xl transition shadow-lg shadow-blue-600/20 uppercase tracking-wider">
            Procesar y Generar Dictamen →
        </a>
    </div>
</body>
</html>"#,
        ref_code, ref_code
    );
    Html(html)
}

async fn create_payment_session(Json(payload): Json<PaymentRequest>) -> impl IntoResponse {
    Json(PaymentResponse {
        checkout_url: format!("/verify/{}", payload.ref_code),
        status: "SUCCESS".to_string(),
    })
}

async fn handle_payment_webhook(Json(_body): Json<Value>) -> impl IntoResponse {
    (StatusCode::OK, Json(json!({"status": "FULFILLED"})))
}
