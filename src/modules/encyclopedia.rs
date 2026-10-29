use axum::response::Html;

pub struct NormaEducativa {
    pub codigo: &'static str,
    pub titulo: &'static str,
    pub incumplimiento_pct: u8,
    pub fuente_estadistica: &'static str,
    pub rango_multa: &'static str,
    pub contexto_dof: &'static str,
    pub de_que_trata: &'static str,
    pub beneficio_productividad: &'static str,
    pub pasos_cumplimiento: &'static [&'static str],
    pub entregable_kit: &'static str,
}

fn get_category_data(
    category: &str,
) -> (
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    Vec<NormaEducativa>,
) {
    match category {
        "stps" => (
            "STPS • LEY FEDERAL DEL TRABAJO Y LAS 39 NORMAS OFICIALES MEXICANAS",
            "Secretaría del Trabajo y Previsión Social (Seguridad, Salud y Rendimiento Laboral)",
            "Cobertura integral de las 39 NOMs: porcentaje real de centros multados, impacto en la productividad del trabajador y guía paso a paso.",
            "El marco inspectivo laboral mexicano emana del Artículo 123 Constitucional de 1917, la Ley Federal del Trabajo de 1970 y el Reglamento Federal de Seguridad y Salud en el Trabajo (DOF 13-nov-2014).",
            "blue",
            "border-blue-600",
            vec![
                NormaEducativa {
                    codigo: "NOM-035-STPS-2018 • Art. 132 Fracc. XVI y Art. 994 Fracc. V LFT",
                    titulo: "Factores de Riesgo Psicosocial en el Trabajo — Identificación, Análisis y Prevención",
                    incumplimiento_pct: 71,
                    fuente_estadistica: "71% de los centros de trabajo inspeccionados presentan omisiones en la aplicación de las Guías de Referencia I, II o III, o carecen de evidencia firmada de su Política de Prevención Psicosocial.",
                    rango_multa: "250 a 5,000 UMAs ($28,285 a $565,700 MXN)",
                    contexto_dof: "Publicada en el Diario Oficial de la Federación (DOF) el 23 de octubre de 2018 tras la reforma al RFSST de 2014 para atender el estrés laboral crónico y la violencia organizacional.",
                    de_que_trata: "Obliga a todos los patrones en México a identificar, analizar y prevenir factores de riesgo psicosocial (cargas excesivas, jornadas superiores a la LFT, liderazgo negativo y violencia laboral).",
                    beneficio_productividad: "Disminuye hasta en un 38% el ausentismo laboral y la rotación de personal clave, elimina el agotamiento (burnout) y eleva la concentración diaria del equipo.",
                    pasos_cumplimiento: &[
                        "Emitir, firmar y difundir físicamente la Política de Prevención de Riesgos Psicosociales (Guía de Referencia IV).",
                        "Aplicar al 100% de la plantilla la Guía de Referencia I para identificar trabajadores expuestos a acontecimientos traumáticos severos.",
                        "Aplicar cada 2 años la Guía de Referencia II o III y conservar cuestionarios firmados.",
                        "Habilitar un buzón confidencial de quejas por violencia laboral con bitácora de resolución."
                    ],
                    entregable_kit: "02_Politica_Prevencion_Psicosocial_NOM035_STPS.pdf y Certificado NOM-035",
                },
                NormaEducativa {
                    codigo: "NOM-037-STPS-2023 • Artículos 330-A al 330-K de la LFT",
                    titulo: "Teletrabajo — Condiciones de Seguridad y Salud en el Trabajo Remoto",
                    incumplimiento_pct: 78,
                    fuente_estadistica: "78% de las empresas con esquemas de trabajo remoto o híbrido carecen del adendum contractual, de la lista de verificación ergonómica y del pago proporcional de internet y electricidad.",
                    rango_multa: "250 a 5,000 UMAs ($28,285 a $565,700 MXN)",
                    contexto_dof: "Emana de la reforma al Capítulo XII Bis de la LFT publicada en el DOF el 11 de enero de 2021 y la NOM-037-STPS-2023 publicada el 8 de junio de 2023.",
                    de_que_trata: "Regula las condiciones ergonómicas, psicosociales y de seguridad eléctrica para empleados que laboren más del 40% de su jornada en un domicilio distinto al centro de trabajo.",
                    beneficio_productividad: "Incrementa el cumplimiento de metas por objetivos, evita incapacidades por lesiones ergonómicas en casa y protege la información confidencial.",
                    pasos_cumplimiento: &[
                        "Integrar el listado de teletrabajadores y firmar el adendum contractual que estipule los insumos entregados y el pago proporcional de servicios.",
                        "Publicar la Política de Teletrabajo reconociendo el Derecho a la Desconexión Digital.",
                        "Aplicar la Lista de Verificación de Condiciones de Seguridad y Salud en el espacio remoto."
                    ],
                    entregable_kit: "04_Politica_Teletrabajo_y_Desconexion_NOM037_STPS.pdf y Certificado NOM-037",
                },
                NormaEducativa {
                    codigo: "NOM-036-1-STPS-2018 • Ergonomía y Manejo Manual de Cargas",
                    titulo: "Factores de Riesgo Ergonómico en el Trabajo — Manejo Manual de Cargas",
                    incumplimiento_pct: 68,
                    fuente_estadistica: "68% de los comercios, restaurantes, almacenes y plantas donde el personal levanta objetos mayores a 3 kg carecen del análisis ergonómico y de vigilancia médica.",
                    rango_multa: "250 a 5,000 UMAs ($28,285 a $565,700 MXN)",
                    contexto_dof: "Publicada en el DOF el 23 de noviembre de 2018 para abatir los trastornos musculoesqueléticos lumbares, primera causa de incapacidad permanente en el IMSS.",
                    de_que_trata: "Aplica a todos los centros de trabajo con actividades de levantamiento, bajada, empuje o transporte manual de cargas superiores a 3 kg de manera cotidiana.",
                    beneficio_productividad: "Protege la columna y salud física de los trabajadores, evitando incapacidades prolongadas y alzas en la Prima de Riesgo IMSS de febrero.",
                    pasos_cumplimiento: &[
                        "Respetar las cargas máximas manuales (25 kg hombres, 20 kg mujeres).",
                        "Elaborar el Análisis de Factores de Riesgo Ergonómico con los Apéndices I y II.",
                        "Proporcionar ayudas mecánicas y capacitar en higiene de columna con constancia DC-3."
                    ],
                    entregable_kit: "Diagnóstico Ergonómico NOM-036-1 en Dictamen Pericial Ejecutivo",
                },
                NormaEducativa {
                    codigo: "NOM-019-STPS-2011 y NOM-030-STPS-2009 • Organización de la Seguridad",
                    titulo: "Comisiones Mixtas de Seguridad e Higiene (NOM-019) y Servicios Preventivos SST (NOM-030)",
                    incumplimiento_pct: 62,
                    fuente_estadistica: "62% de los centros de trabajo reciben emplazamientos por no exhibir el Acta Constitutiva de la Comisión Mixta o las actas de recorridos trimestrales.",
                    rango_multa: "250 a 5,000 UMAs por cada norma ($28,285 a $565,700 MXN)",
                    contexto_dof: "NOM-030 (DOF 22-dic-2009) y NOM-019 (DOF 13-abr-2011), consolidando el sistema de autogestión bipartita establecido en la LFT.",
                    de_que_trata: "La NOM-019 exige constituir una Comisión bipartita que realiza recorridos trimestrales; la NOM-030 exige designar a un Responsable de Seguridad y Salud en el Trabajo (SST).",
                    beneficio_productividad: "Involucra a los trabajadores en el cuidado de las instalaciones y detecta fallas antes de que provoquen accidentes.",
                    pasos_cumplimiento: &[
                        "Firmar el Acta Constitutiva de la Comisión Mixta con todos los datos patronales.",
                        "Designar por escrito al Responsable SST y actualizar anualmente el Programa Preventivo.",
                        "Archivar cada 3 meses el Acta de Recorrido de Verificación."
                    ],
                    entregable_kit: "03_Acta_Constitutiva_Comision_Seguridad_NOM019_y_NOM030.pdf",
                },
            ],
        ),
        "sat" => (
            "SAT • CÓDIGO FISCAL DE LA FEDERACIÓN (CFF), LISR Y LIVA",
            "Servicio de Administración Tributaria (Materialidad, Buzón, CFDI 4.0, CSD y Plataformas)",
            "Criterios de la SCJN, porcentajes reales de rechazo en auditoría, blindaje de flujo de efectivo y guía paso a paso por artículo.",
            "A partir de la Reforma Hacendaria de 2014, el SAT fiscaliza mediante algoritmos que cruzan en tiempo real los CFDI 4.0 contra listas negras y declaraciones.",
            "emerald",
            "border-emerald-600",
            vec![
                NormaEducativa {
                    codigo: "Artículo 69-B y Artículo 69 del CFF • Jurisprudencia SCJN 2a./J. 161/2019 (10a.)",
                    titulo: "Materialidad y Sustancia Económica (EFOS/EDOS), Fecha Cierta y Listas Negras",
                    incumplimiento_pct: 81,
                    fuente_estadistica: "81% de las deducciones por servicios intangibles o proveeduría revisadas en auditorías del SAT son rechazadas por carecer de Fecha Cierta y entregables.",
                    rango_multa: "Rechazo del 100% de la deducción (30% ISR + 16% IVA) + Multa del 55% al 75% (Art. 76 CFF)",
                    contexto_dof: "Artículo 69-B (DOF 9-dic-2013) y Jurisprudencia obligatoria 2a./J. 161/2019 (10a.) de la SCJN sobre Fecha Cierta en contratos privados.",
                    de_que_trata: "Faculta al SAT para publicar a quienes emiten comprobantes sin activos (EFOS). Quien dedujo esos CFDI tiene 30 días para acreditar la materialidad real.",
                    beneficio_productividad: "Blinda el 100% de las deducciones de ISR y el acreditamiento de IVA, protegiendo el patrimonio corporativo.",
                    pasos_cumplimiento: &[
                        "Celebrar Contrato Marco con Fecha Cierta Digital inmutable (sello SHA-256 / NOM-151).",
                        "Integrar una Bitácora de Materialidad con órdenes de compra, entregables y opinión 32-D del proveedor.",
                        "Monitorear mensualmente el RFC de proveedores contra las listas del SAT."
                    ],
                    entregable_kit: "05_Contrato_Marco_Materialidad_Fecha_Cierta_y_Bitacora_69B.pdf",
                },
            ],
        ),
        "imss" => (
            "IMSS E INFONAVIT • LEY DEL SEGURO SOCIAL (LSS) Y RACERF",
            "Instituto Mexicano del Seguro Social e INFONAVIT (Seguridad Social y Blindaje Patronal)",
            "Conciliación de nómina vs. SUA, ahorro en Prima de Riesgo y blindaje REPSE.",
            "Regida por la LSS vigente desde julio de 1997 y la reforma de subcontratación de abril de 2021.",
            "purple",
            "border-purple-600",
            vec![
                NormaEducativa {
                    codigo: "Artículo 15-A LSS • Artículos 12 a 15 LFT (ICSOE y SISSUB)",
                    titulo: "Reforma de Subcontratación, Padrón REPSE y Reportes Cuatrimestrales ICSOE / SISSUB",
                    incumplimiento_pct: 38,
                    fuente_estadistica: "38% de las empresas con registro REPSE omiten la presentación puntual de las informativas cuatrimestrales ICSOE y SISSUB.",
                    rango_multa: "500 a 2,000 UMAs por informativa omitida; y hasta 50,000 UMAs (Art. 1004-C LFT)",
                    contexto_dof: "Reforma publicada en el DOF el 23 de abril de 2021 prohibiendo el outsourcing de personal y creando el padrón REPSE.",
                    de_que_trata: "Permite la subcontratación de servicios especializados que no formen parte del objeto social, obligando a presentar informes cuatrimestrales en ICSOE y SISSUB.",
                    beneficio_productividad: "Convierte a la empresa en proveedor elegible para grandes corporativos e industrias.",
                    pasos_cumplimiento: &[
                        "Mantener vigente el folio REPSE y presentar ICSOE y SISSUB a más tardar el 17 de enero, mayo y septiembre.",
                        "Recabar mensualmente los 7 comprobantes del Art. 27 Fracc. V LISR de cada proveedor."
                    ],
                    entregable_kit: "06_Protocolo_Blindaje_Solidario_REPSE_ICSOE_SISSUB.pdf",
                },
            ],
        ),
        "inai" => (
            "SABG / EX-INAI • LEY FEDERAL DE PROTECCIÓN DE DATOS PERSONALES (LFPDPPP)",
            "Autoridad Garante Federal (SABG) • Pilares de Privacidad, ARCO y Biométricos",
            "Supervisión de la SABG, porcentajes de sanción y confianza digital.",
            "Publicada en el DOF el 5 de julio de 2010 y supervisada actualmente por la Secretaría Anticorrupción y Buen Gobierno (SABG).",
            "cyan",
            "border-cyan-600",
            vec![
                NormaEducativa {
                    codigo: "Artículos 8, 15 y 16 de la LFPDPPP y Lineamientos del Aviso de Privacidad",
                    titulo: "Aviso de Privacidad en sus 3 Modalidades: Integral, Simplificado y Corto",
                    incumplimiento_pct: 73,
                    fuente_estadistica: "73% de las empresas verificadas tienen un texto genérico que omite los requisitos del Artículo 16.",
                    rango_multa: "100 a 160,000 UMAs ($11,314 a $18,102,400 MXN — Art. 63 y 64 LFPDPPP)",
                    contexto_dof: "LFPDPPP (DOF 5-jul-2010) y Lineamientos del Aviso de Privacidad (DOF 17-ene-2013).",
                    de_que_trata: "Obliga a poner a disposición del titular el Aviso de Privacidad ANTES de recabar sus datos.",
                    beneficio_productividad: "Genera confianza inmediata en los visitantes del sitio web, incrementando la conversión comercial.",
                    pasos_cumplimiento: &[
                        "Publicar el Aviso de Privacidad Integral en el pie de página web.",
                        "Colocar el Aviso Corto en zonas con videovigilancia CCTV."
                    ],
                    entregable_kit: "01_Aviso_de_Privacidad_Integral_ARCO_y_Biometricos.pdf",
                },
            ],
        ),
        "pld" => (
            "LOS 9 MÓDULOS SECTORIALES Y PREVENCIÓN DE LAVADO DE DINERO (LFPIORPI)",
            "Regulación Sectorial por Giro SCIAN (UIF, COFEPRIS, PIPC, SEMARNAT, SICT, REPSE, SSPC, SEP, SECTUR)",
            "Continuidad de negocio y guía paso a paso para los 9 Módulos Sectoriales.",
            "Obligaciones especiales según el código SCIAN de 6 dígitos y operaciones financieras.",
            "amber",
            "border-amber-600",
            vec![
                NormaEducativa {
                    codigo: "Módulo Sectorial 01 • CNBV / CONDUSEF / LFPIORPI (Artículos 17, 18, 32, 53 y 54)",
                    titulo: "Fintech, Sector Financiero y las 16 Actividades Vulnerables Antilavado (UIF / SAT)",
                    incumplimiento_pct: 69,
                    fuente_estadistica: "69% de las empresas que otorgan préstamos, arriendan inmuebles o desarrollan obras inmobiliarias omiten su alta en el portal SPPLD.",
                    rango_multa: "10,000 a 65,000 UMAs ($1,131,400 a $7,354,100 MXN) por aviso omitido",
                    contexto_dof: "LFPIORPI publicada en el DOF el 17 de octubre de 2012 (en vigor el 17 de julio de 2013).",
                    de_que_trata: "Obliga a realizar Actividades Vulnerables a identificar al cliente, Dueño Beneficiario y enviar Avisos a la UIF.",
                    beneficio_productividad: "Evita el congelamiento de cuentas bancarias y protege la reputación financiera.",
                    pasos_cumplimiento: &[
                        "Darse de alta en el Portal SPPLD del SAT y contar con Manual de Cumplimiento PLD.",
                        "Integrar el Expediente Único de Identificación antes de la operación y presentar Avisos el día 17."
                    ],
                    entregable_kit: "Dictamen de Actividades Vulnerables LFPIORPI en Expediente Ejecutivo",
                },
            ],
        ),
        _ => (
            "PROFECO Y SECRETARÍA DE ECONOMÍA • LEY FEDERAL DE PROTECCIÓN AL CONSUMIDOR (LFPC)",
            "Procuraduría Federal del Consumidor y Secretaría de Economía (Comercio Digital, RRPAC y NOM-151)",
            "Monitoreo de tiendas virtuales, reputación comercial y guía paso a paso por artículo.",
            "Regida por la LFPC de orden público e interés social, PROFECO supervisa establecimientos físicos y tiendas en línea.",
            "teal",
            "border-teal-600",
            vec![
                NormaEducativa {
                    codigo: "Artículo 76 BIS y 76 BIS 1 de la LFPC y Norma Mexicana NMX-COE-001-SCFI-2018",
                    titulo: "Comercio Electrónico, Portales Digitales y Transparencia de Precios y Contacto",
                    incumplimiento_pct: 56,
                    fuente_estadistica: "56% de los sitios web comerciales omiten publicar su domicilio físico real en territorio nacional o teléfonos de atención.",
                    rango_multa: "$701.15 a $3,995,249.26 MXN (Art. 127 y 128 LFPC)",
                    contexto_dof: "Capítulo VIII Bis adicionado a la LFPC en el DOF del 29 de mayo de 2000.",
                    de_que_trata: "Obliga a todo proveedor que venda por medios electrónicos a informar domicilio en México, teléfonos, correo y precios totales con IVA.",
                    beneficio_productividad: "Aumenta la tasa de cierre de ventas en línea al brindar certeza total al comprador.",
                    pasos_cumplimiento: &[
                        "Publicar en el pie de página la Razón Social, RFC, domicilio físico en México y correo.",
                        "Mostrar precios totales con IVA incluido y prohibir propinas obligatorias."
                    ],
                    entregable_kit: "07_Terminos_Condiciones_Ecommerce_Art76BIS_y_Garantias.pdf",
                },
            ],
        ),
    }
}

pub fn render_norma_category(category: &str) -> Html<String> {
    let (siglas, titulo, subtitulo, resumen_ejecutivo, color_tw, border_top, normas) =
        get_category_data(category);

    let mut modulos_html = String::from(r#"<div class="grid grid-cols-1 md:grid-cols-2 gap-5 mb-8">"#);
    for (idx, n) in normas.iter().enumerate() {
        let num_exp = idx + 1;
        let pasos_count = n.pasos_cumplimiento.len();

        modulos_html.push_str(&format!(
            r##"<div class="quantum-card rounded-xl p-5 border border-{color_tw}-500/30 flex flex-col justify-between group">
                <div>
                    <div class="flex items-center justify-between gap-2 mb-2">
                        <span class="inline-block text-[10px] font-bold font-mono-code text-{color_tw}-400 bg-{color_tw}-950/60 border border-{color_tw}-500/30 px-2.5 py-0.5 rounded">
                            {codigo}
                        </span>
                        <span class="text-[10px] font-mono-code text-slate-400">Expediente #{num_exp:02}</span>
                    </div>

                    <h2 class="text-base sm:text-lg font-extrabold text-white tracking-tight leading-snug mb-3 group-hover:text-{color_tw}-300 transition">
                        {titulo_norma}
                    </h2>

                    <div class="w-full bg-slate-950/90 border border-red-500/30 rounded-lg p-3 mb-3.5">
                        <div class="flex items-center justify-between text-[10px] font-mono-code mb-1">
                            <span class="uppercase text-slate-300 font-semibold">Incumplimiento en México:</span>
                            <span class="text-xs font-extrabold text-red-400">{pct}% de empresas inspeccionadas</span>
                        </div>
                        <div class="w-full bg-slate-800 h-2.5 rounded-full overflow-hidden border border-white/10">
                            <div class="bg-gradient-to-r from-amber-500 via-red-500 to-rose-600 h-full rounded-full" style="width: {pct}% !important;"></div>
                        </div>
                        <div class="flex items-center justify-between text-[9px] font-mono-code text-amber-300 mt-1.5 pt-1.5 border-t border-white/5">
                            <span>Riesgo de Sanción:</span>
                            <strong class="text-amber-200">{multa}</strong>
                        </div>
                    </div>

                    <p class="text-xs text-slate-300 font-mono-code leading-relaxed mb-4 line-clamp-2">
                        {estadistica}
                    </p>

                    <div class="grid grid-cols-3 gap-2 mb-4 text-center font-mono-code">
                        <div class="bg-slate-950/60 border border-white/10 rounded-lg py-1.5 px-2">
                            <span class="text-[9px] text-emerald-400 font-bold block">ROI & Salud</span>
                            <span class="text-[9px] text-slate-400">Ventaja Operativa</span>
                        </div>
                        <div class="bg-slate-950/60 border border-white/10 rounded-lg py-1.5 px-2">
                            <span class="text-[9px] text-{color_tw}-400 font-bold block">Contexto DOF</span>
                            <span class="text-[9px] text-slate-400">Marco y Sujetos</span>
                        </div>
                        <div class="bg-slate-950/60 border border-white/10 rounded-lg py-1.5 px-2">
                            <span class="text-[9px] text-amber-400 font-bold block">{pasos_count} Pasos Clave</span>
                            <span class="text-[9px] text-slate-400">Guía de Blindaje</span>
                        </div>
                    </div>
                </div>

                <div class="flex flex-col sm:flex-row items-stretch sm:items-center justify-between gap-2.5 pt-3 border-t border-white/10 text-[11px] font-mono-code">
                    <a href="/normas/{cat_slug}/{idx}" class="flex-1 bg-gradient-to-r from-{color_tw}-600/30 to-slate-900 hover:from-{color_tw}-600/50 hover:to-slate-800 text-white border border-{color_tw}-500/50 font-bold px-3.5 py-2.5 rounded-lg transition flex items-center justify-between group/btn">
                        <span>Descubrir cómo cumplir y proteger a su equipo</span>
                        <span class="text-{color_tw}-300 group-hover/btn:translate-x-1 transition font-extrabold ml-2">→</span>
                    </a>
                    <a href="/#express-check" class="shrink-0 bg-slate-900 hover:bg-slate-800 text-slate-300 hover:text-white border border-white/15 font-bold px-3 py-2.5 rounded-lg transition text-center">
                        Auditar RFC
                    </a>
                </div>
            </div>"##,
            color_tw = color_tw,
            codigo = n.codigo,
            num_exp = num_exp,
            titulo_norma = n.titulo,
            pct = n.incumplimiento_pct,
            multa = n.rango_multa,
            estadistica = n.fuente_estadistica,
            pasos_count = pasos_count,
            cat_slug = category,
            idx = idx
        ));
    }
    modulos_html.push_str("</div>");

    let html = format!(
        r##"<!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{titulo} — Cobertura Normativa | Firma Pericial Mallen</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;800&family=JetBrains+Mono:wght@400;500;600&display=swap" rel="stylesheet">
    <style>body {{ font-family: 'Inter', sans-serif; }} .font-mono-code {{ font-family: 'JetBrains Mono', monospace; }}</style>
    <link rel="stylesheet" href="/css/quantum-glass.css?v=2026.20">
    <script src="/js/quantum-lights.js?v=2026.20" defer></script>
</head>
<body class="bg-slate-950 text-slate-100 min-h-screen p-4 sm:p-8 border-t-4 {border_top}">
    <div class="max-w-7xl mx-auto">
        <div class="flex flex-col sm:flex-row sm:items-center justify-between pb-5 mb-6 border-b border-white/15 gap-4">
            <div>
                <span class="text-xs font-bold text-{color_tw}-400 font-mono-code uppercase tracking-wider block">FIRMA PERICIAL MALLEN • CENTRO DE CUMPLIMIENTO Y RENDIMIENTO EMPRESARIAL</span>
                <span class="text-xs text-slate-400 font-mono-code">{siglas}</span>
            </div>
            <div class="flex flex-wrap gap-2 text-[11px] font-mono-code">
                <a href="/normas/stps" class="px-3 py-1.5 rounded-lg bg-slate-900 border border-white/10 hover:border-blue-400 text-slate-300 transition">STPS (39 NOMs)</a>
                <a href="/normas/sat" class="px-3 py-1.5 rounded-lg bg-slate-900 border border-white/10 hover:border-emerald-400 text-slate-300 transition">SAT / CFF</a>
                <a href="/normas/imss" class="px-3 py-1.5 rounded-lg bg-slate-900 border border-white/10 hover:border-purple-400 text-slate-300 transition">IMSS / INFONAVIT</a>
                <a href="/normas/inai" class="px-3 py-1.5 rounded-lg bg-slate-900 border border-white/10 hover:border-cyan-400 text-slate-300 transition">LFPDPPP / SABG</a>
                <a href="/normas/pld" class="px-3 py-1.5 rounded-lg bg-slate-900 border border-white/10 hover:border-amber-400 text-slate-300 transition">9 Sectoriales & PLD</a>
                <a href="/normas/comercio" class="px-3 py-1.5 rounded-lg bg-slate-900 border border-white/10 hover:border-teal-400 text-slate-300 transition">PROFECO & NOM-151</a>
                <a href="/" class="px-3 py-1.5 rounded-lg bg-{color_tw}-600 text-white font-bold transition">Portal Principal</a>
            </div>
        </div>

        <div class="quantum-laser-console rounded-2xl p-6 mb-8">
            <span class="text-xs font-bold font-mono-code text-{color_tw}-400 uppercase tracking-widest block mb-1.5">{siglas}</span>
            <h1 class="text-xl sm:text-3xl font-extrabold text-white mb-2 tracking-tight">{titulo}</h1>
            <p class="text-xs sm:text-sm text-slate-300 font-mono-code mb-3">{subtitulo}</p>
            <p class="text-xs text-slate-400 font-mono-code leading-relaxed border-t border-white/10 pt-3">
                {resumen_ejecutivo} — <strong>Seleccione cualquier expediente inferior para abrir su desglose profundo, criterios de inspección y guía práctica paso a paso.</strong>
            </p>
        </div>

        {modulos_html}

        <div class="quantum-card rounded-2xl p-6 text-center border border-emerald-500/30 mt-6">
            <h3 class="text-lg font-extrabold text-white mb-1.5">Proteja a su equipo de trabajo, eleve su productividad y blinde el patrimonio de su empresa</h3>
            <p class="text-xs text-slate-300 font-mono-code max-w-2xl mx-auto mb-5">
                Ingrese su RFC y sitio web en nuestro motor pericial para cruzar sus datos contra los padrones nacionales y obtener sus 10 Certificados y 7 Instrumentos Jurídicos listos para firma.
            </p>
            <a href="/#express-check" class="inline-block bg-gradient-to-r from-blue-600 to-emerald-600 hover:from-blue-500 hover:to-emerald-500 text-white font-bold text-xs font-mono-code px-7 py-3.5 rounded-xl transition uppercase tracking-wider shadow-lg shadow-emerald-600/20">
                Ejecutar Diagnóstico Pericial 360° con mi RFC
            </a>
        </div>
    </div>
</body>
</html>"##,
        titulo = titulo,
        border_top = border_top,
        color_tw = color_tw,
        siglas = siglas,
        subtitulo = subtitulo,
        resumen_ejecutivo = resumen_ejecutivo,
        modulos_html = modulos_html
    );

    Html(html)
}

pub fn render_subnorma_detail(category: &str, code_idx: &str) -> Html<String> {
    let (siglas, titulo, _sub, _res, color_tw, border_top, normas) =
        get_category_data(category);

    let idx = code_idx.parse::<usize>().unwrap_or(0);
    let n = normas.get(idx).unwrap_or(&normas[0]);

    let mut pasos_li = String::new();
    for (p_i, paso) in n.pasos_cumplimiento.iter().enumerate() {
        pasos_li.push_str(&format!(
            r##"<li class="flex items-start gap-3 text-xs sm:text-sm text-slate-200 leading-relaxed bg-slate-900/75 border border-white/10 rounded-xl p-3.5">
                <span class="shrink-0 w-6 h-6 rounded-full bg-{color_tw}-500/20 border border-{color_tw}-400/50 text-{color_tw}-300 font-bold flex items-center justify-center text-xs">{num}</span>
                <span>{paso}</span>
            </li>"##,
            color_tw = color_tw,
            num = p_i + 1,
            paso = paso
        ));
    }

    let mut otros_html = String::new();
    for (o_idx, o_norm) in normas.iter().enumerate() {
        if o_idx == idx {
            continue;
        }
        otros_html.push_str(&format!(
            r##"<a href="/normas/{cat}/{o_idx}" class="quantum-card rounded-xl p-4 border border-white/10 hover:border-{color_tw}-400/60 transition flex flex-col justify-between group">
                <div>
                    <div class="flex items-center justify-between gap-2 mb-1.5">
                        <span class="text-[10px] font-mono-code font-bold text-{color_tw}-400">{cod}</span>
                        <span class="text-[10px] font-mono-code font-bold text-red-400">{pct}% Incumplimiento</span>
                    </div>
                    <h4 class="text-xs sm:text-sm font-bold text-white group-hover:text-{color_tw}-300 transition leading-snug mb-2">{tit}</h4>
                </div>
                <span class="text-[10px] font-mono-code text-{color_tw}-400 font-bold flex items-center justify-between pt-2 border-t border-white/10">
                    <span>Abrir expediente técnico completo</span>
                    <span class="group-hover:translate-x-1 transition">→</span>
                </span>
            </a>"##,
            cat = category,
            o_idx = o_idx,
            color_tw = color_tw,
            cod = o_norm.codigo,
            pct = o_norm.incumplimiento_pct,
            tit = o_norm.titulo
        ));
    }

    let html = format!(
        r##"<!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{titulo_norma} — Expediente Técnico | Firma Pericial Mallen</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;800&family=JetBrains+Mono:wght@400;500;600&display=swap" rel="stylesheet">
    <style>body {{ font-family: 'Inter', sans-serif; }} .font-mono-code {{ font-family: 'JetBrains Mono', monospace; }}</style>
    <link rel="stylesheet" href="/css/quantum-glass.css?v=2026.20">
    <script src="/js/quantum-lights.js?v=2026.20" defer></script>
</head>
<body class="bg-slate-950 text-slate-100 min-h-screen p-4 sm:p-8 border-t-4 {border_top}">
    <div class="max-w-5xl mx-auto">
        <div class="flex flex-col sm:flex-row sm:items-center justify-between pb-5 mb-6 border-b border-white/15 gap-4">
            <div>
                <a href="/normas/{cat}" class="inline-flex items-center gap-2 text-xs font-bold text-{color_tw}-400 hover:text-{color_tw}-300 font-mono-code mb-1 transition">
                    <span>← Volver al Tablero de {titulo_cat}</span>
                </a>
                <span class="text-[11px] text-slate-400 font-mono-code block">{siglas}</span>
            </div>
            <div class="flex flex-wrap gap-2 text-[11px] font-mono-code">
                <a href="/normas/stps" class="px-2.5 py-1 rounded-lg bg-slate-900 border border-white/10 hover:border-blue-400 text-slate-300 transition">STPS</a>
                <a href="/normas/sat" class="px-2.5 py-1 rounded-lg bg-slate-900 border border-white/10 hover:border-emerald-400 text-slate-300 transition">SAT</a>
                <a href="/normas/imss" class="px-2.5 py-1 rounded-lg bg-slate-900 border border-white/10 hover:border-purple-400 text-slate-300 transition">IMSS</a>
                <a href="/normas/inai" class="px-2.5 py-1 rounded-lg bg-slate-900 border border-white/10 hover:border-cyan-400 text-slate-300 transition">LFPDPPP</a>
                <a href="/normas/pld" class="px-2.5 py-1 rounded-lg bg-slate-900 border border-white/10 hover:border-amber-400 text-slate-300 transition">Sectoriales</a>
                <a href="/normas/comercio" class="px-2.5 py-1 rounded-lg bg-slate-900 border border-white/10 hover:border-teal-400 text-slate-300 transition">PROFECO</a>
                <a href="/" class="px-3 py-1 rounded-lg bg-{color_tw}-600 text-white font-bold transition">Portal Principal</a>
            </div>
        </div>

        <div class="quantum-laser-console rounded-2xl p-6 sm:p-8 mb-8">
            <div class="flex flex-wrap items-center justify-between gap-2 mb-3">
                <span class="inline-block text-xs font-bold font-mono-code text-{color_tw}-400 bg-{color_tw}-950/70 border border-{color_tw}-500/40 px-3 py-1 rounded-lg">
                    {codigo}
                </span>
                <span class="text-xs font-mono-code text-amber-300 bg-amber-950/50 border border-amber-500/40 px-3 py-1 rounded-lg font-bold">
                    Sanción Oficial: {multa}
                </span>
            </div>

            <h1 class="text-xl sm:text-3xl font-extrabold text-white tracking-tight leading-snug mb-5">
                {titulo_norma}
            </h1>

            <div class="w-full bg-slate-950/90 border border-red-500/35 rounded-xl p-4 mb-6">
                <div class="flex items-center justify-between text-xs font-mono-code mb-2">
                    <span class="uppercase text-slate-300 font-bold">Índice Real de Incumplimiento en México:</span>
                    <span class="text-sm font-extrabold text-red-400">{pct}% de los centros inspeccionados</span>
                </div>
                <div class="w-full bg-slate-800 h-3 rounded-full overflow-hidden border border-white/10">
                    <div class="bg-gradient-to-r from-amber-500 via-red-500 to-rose-600 h-full rounded-full" style="width: {pct}% !important;"></div>
                </div>
                <div class="flex justify-between text-[9px] font-mono-code text-slate-500 mt-1.5">
                    <span>0%</span><span>25%</span><span>50%</span><span>75%</span><span>100%</span>
                </div>
            </div>

            <div class="bg-red-950/25 border border-red-500/35 rounded-xl p-4 mb-5 font-mono-code">
                <strong class="text-red-400 uppercase block text-[11px] mb-1 tracking-wider">DIAGNÓSTICO DE INSPECCIÓN FEDERAL:</strong>
                <p class="text-xs sm:text-sm text-slate-200 leading-relaxed">{estadistica}</p>
            </div>

            <div class="grid sm:grid-cols-2 gap-4 mb-5 font-mono-code">
                <div class="bg-slate-950/70 border border-white/15 rounded-xl p-5">
                    <span class="text-[11px] font-bold text-{color_tw}-400 uppercase tracking-wider block mb-2">
                        MARCO LEGAL Y CONTEXTO (DOF)
                    </span>
                    <p class="text-xs text-slate-300 leading-relaxed">{contexto}</p>
                </div>
                <div class="bg-slate-950/70 border border-white/15 rounded-xl p-5">
                    <span class="text-[11px] font-bold text-sky-400 uppercase tracking-wider block mb-2">
                        ALCANCE Y SUJETOS OBLIGADOS
                    </span>
                    <p class="text-xs text-slate-300 leading-relaxed">{trata}</p>
                </div>
            </div>

            <div class="bg-emerald-950/30 border border-emerald-500/40 rounded-xl p-5 mb-6 font-mono-code">
                <strong class="text-emerald-400 uppercase block text-[11px] mb-1.5 tracking-wider">
                    VENTAJA COMPETITIVA, SEGURIDAD DEL PERSONAL Y RENDIMIENTO:
                </strong>
                <p class="text-xs sm:text-sm text-emerald-100 leading-relaxed">{beneficio}</p>
            </div>

            <div class="bg-slate-950/85 border border-{color_tw}-500/35 rounded-xl p-5 mb-6 font-mono-code">
                <span class="text-xs font-bold text-white uppercase tracking-wider block mb-3.5">
                    GUÍA PRÁCTICA PARA CUMPLIR AL 100%:
                </span>
                <ul class="space-y-2.5">{pasos_li}</ul>
            </div>

            <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 pt-5 border-t border-white/15 font-mono-code text-xs">
                <div>
                    <span class="text-slate-400 block text-[10px] uppercase">Instrumento incluido en su Kit Mallen 360°:</span>
                    <strong class="text-emerald-400 text-sm">{entregable}</strong>
                </div>
                <div class="flex flex-wrap gap-3">
                    <a href="/normas/{cat}" class="px-4 py-3 rounded-xl bg-slate-900 hover:bg-slate-800 text-slate-200 border border-white/15 font-bold transition">
                        ← Ver más normas de {cat_upper}
                    </a>
                    <a href="/#express-check" class="px-6 py-3 rounded-xl bg-gradient-to-r from-blue-600 to-emerald-600 hover:from-blue-500 hover:to-emerald-500 text-white font-bold uppercase shadow-lg shadow-emerald-600/25 transition">
                        Auditar este precepto con mi RFC →
                    </a>
                </div>
            </div>
        </div>

        <div class="mb-8">
            <div class="flex items-center justify-between mb-4">
                <div>
                    <span class="text-[10px] font-mono-code font-bold text-{color_tw}-400 uppercase tracking-widest block">Exploración Continua de Cumplimiento</span>
                    <h3 class="text-lg font-extrabold text-white">Otros Expedientes Críticos en {titulo_cat}</h3>
                </div>
                <a href="/normas/{cat}" class="text-xs font-mono-code text-{color_tw}-400 hover:underline">Ver todos →</a>
            </div>
            <div class="grid sm:grid-cols-2 lg:grid-cols-3 gap-4">{otros_html}</div>
        </div>
    </div>
</body>
</html>"##,
        titulo_norma = n.titulo,
        border_top = border_top,
        cat = category,
        color_tw = color_tw,
        titulo_cat = titulo,
        siglas = siglas,
        codigo = n.codigo,
        multa = n.rango_multa,
        pct = n.incumplimiento_pct,
        estadistica = n.fuente_estadistica,
        contexto = n.contexto_dof,
        trata = n.de_que_trata,
        beneficio = n.beneficio_productividad,
        pasos_li = pasos_li,
        entregable = n.entregable_kit,
        cat_upper = category.to_uppercase(),
        otros_html = otros_html
    );

    Html(html)
}
