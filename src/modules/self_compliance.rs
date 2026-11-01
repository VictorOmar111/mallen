use axum::response::Html;

/// 1. Aviso de Privacidad Integral Oficial de mallen.lat (Arts. 8, 15, 16 y 17 LFPDPPP / Autoridad Garante SABG)
pub fn render_mallen_privacy_notice() -> Html<String> {
    Html(r##"<!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Aviso de Privacidad Integral — Firma Pericial Mallen (mallen.lat)</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;800&family=JetBrains+Mono:wght@400;500;600&display=swap" rel="stylesheet">
    <style>body { font-family: 'Inter', sans-serif; } .font-mono-code { font-family: 'JetBrains Mono', monospace; }</style>
    <link rel="stylesheet" href="/css/quantum-glass.css?v=2026.27">
</head>
<body class="bg-slate-950 text-slate-100 min-h-screen p-6 sm:p-10 border-t-4 border-emerald-500">
    <div class="max-w-4xl mx-auto">
        <div class="flex items-center justify-between pb-5 mb-6 border-b border-white/15">
            <div>
                <span class="text-xs font-bold text-emerald-400 font-mono-code uppercase block">CUMPLIMIENTO NORMATIVO PROPIO • LFPDPPP Y LINEAMIENTOS DOF</span>
                <h1 class="text-2xl sm:text-3xl font-extrabold text-white mt-1">Aviso de Privacidad Integral de mallen.lat</h1>
            </div>
            <a href="/" class="px-4 py-2 rounded-xl bg-slate-900 border border-white/15 text-xs font-mono-code text-emerald-300 hover:bg-slate-800 transition">← Portal Principal</a>
        </div>

        <div class="quantum-laser-console rounded-2xl p-6 sm:p-8 space-y-5 text-xs sm:text-sm text-slate-300 leading-relaxed font-mono-code">
            <div>
                <h2 class="text-sm font-bold text-emerald-400 uppercase mb-1">I. Identidad y Domicilio del Responsable (Art. 16 Fracc. I LFPDPPP)</h2>
                <p><strong>Firma Pericial Mallen — Sistemas de Inteligencia Regulatoria y Auditoría Preventiva 360°</strong> (en lo sucesivo, "EL RESPONSABLE"), con portal oficial en <strong>https://mallen.lat</strong>, domicilio para oír y recibir notificaciones de privacidad en Ciudad de México, Estados Unidos Mexicanos, y Oficialía de Datos Personales en <strong>privacidad@mallen.lat</strong>, es responsable del uso, tratamiento y protección de los datos que se recaben a través de esta plataforma, en estricto apego a la Ley Federal de Protección de Datos Personales en Posesión de los Particulares (LFPDPPP), su Reglamento y los Lineamientos del Aviso de Privacidad publicados en el Diario Oficial de la Federación.</p>
            </div>

            <div>
                <h2 class="text-sm font-bold text-emerald-400 uppercase mb-1">II. Naturaleza de los Datos Tratados y Fuentes Públicas (Art. 10 Fracc. II LFPDPPP)</h2>
                <p>1. <strong>Datos Corporativos de Personas Morales y Fuentes de Acceso Público:</strong> De conformidad con los Artículos 2 y 10 Fracción II de la LFPDPPP, los datos relativos a denominaciones o razones sociales, claves del Registro Federal de Contribuyentes (RFC) de personas morales, giros SCIAN, domicilios de establecimientos comerciales y telemetría técnica de dominios web públicos provienen de registros de acceso público (Directorio Estadístico Nacional de Unidades Económicas DENUE del INEGI, publicaciones del Artículo 69 y 69-B del SAT en el DOF y cabeceras HTTP/TLS públicas).</p>
                <p class="mt-2">2. <strong>Datos Proporcionados por el Usuario en la Cédula de Personalización:</strong> Nombre de contacto o representante legal, correo electrónico corporativo, teléfono empresarial y datos de facturación. <strong>EL RESPONSABLE NO recaba ni almacena datos personales sensibles ni biométricos de los visitantes de mallen.lat</strong>; los pagos con tarjeta son procesados de forma tokenizada directamente bajo estándar PCI-DSS Nivel 1.</p>
            </div>

            <div>
                <h2 class="text-sm font-bold text-emerald-400 uppercase mb-1">III. Finalidades Primarias y Secundarias (Art. 16 Fracc. II LFPDPPP)</h2>
                <p><strong>Finalidades Primarias (Necesarias para el servicio solicitado):</strong> (a) Ejecutar el diagnóstico automatizado de exposición regulatoria (STPS, SAT, IMSS, LFPDPPP, PLD y PROFECO); (b) Compilar en memoria RAM los dictámenes periciales, certificados y plantillas de contratos personalizados solicitados por el usuario; (c) Emitir el sello criptográfico SHA-256 / Árbol de Merkle de integridad documental y su Acuse de Entrega Electrónico; y (d) Emitir el Comprobante Fiscal Digital por Internet (CFDI 4.0) correspondiente.</p>
                <p class="mt-2"><strong>Finalidades Secundarias (Opcionales):</strong> Enviar boletines de reformas publicadas en el Diario Oficial de la Federación (DOF). Si usted no desea que sus datos de contacto se traten para finalidades secundarias, puede manifestarlo en cualquier momento en nuestra <a href="/derechos-arco" class="text-emerald-400 underline">Ventanilla Digital ARCO (/derechos-arco)</a>.</p>
            </div>

            <div>
                <h2 class="text-sm font-bold text-emerald-400 uppercase mb-1">IV. Ejercicio de Derechos ARCO, Revocación y Purga Inmediata (Arts. 22 a 35 LFPDPPP)</h2>
                <p>Cualquier titular (incluyendo Personas Físicas con Actividad Empresarial) tiene derecho a conocer qué datos obran en el sistema (Acceso), solicitar su corrección (Rectificación), pedir la eliminación inmediata de su diagnóstico del caché del servidor (Cancelación) u oponerse a su uso (Oposición). Para su comodidad, hemos habilitado un <strong>Módulo Automatizado de Purga Instantánea y Derechos ARCO en <a href="/derechos-arco" class="text-emerald-400 font-bold underline">https://mallen.lat/derechos-arco</a></strong> que elimina de forma inmediata cualquier registro en caché y emite su Folio Oficial de Cancelación en menos de 5 segundos, o bien mediante solicitud al correo <strong>privacidad@mallen.lat</strong> (plazo máximo de respuesta: 20 días hábiles conforme al Art. 32 LFPDPPP).</p>
            </div>

            <div>
                <h2 class="text-sm font-bold text-emerald-400 uppercase mb-1">V. Medidas de Seguridad y Uso de Cookies Técnicas (Art. 19 LFPDPPP)</h2>
                <p>Esta plataforma opera bajo cifrado TLS 1.3, aislamiento de procesos en memoria RAM volátil y controles Anti-SSRF. Utilizamos únicamente almacenamiento local técnico (<code>localStorage</code>) para recordar la aceptación del presente aviso y mantener la sesión activa del diagnóstico, sin emplear rastreadores publicitarios invasivos de terceros.</p>
            </div>
        </div>
    </div>
</body>
</html>"##.to_string())
}

/// 2. Términos y Condiciones, Alcance Pericial, Anti-Contracargos y Fuentes Abiertas (Art. 76 BIS LFPC y Art. 89 CCom)
pub fn render_mallen_terms() -> Html<String> {
    Html(r##"<!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Términos y Condiciones de Uso y Alcance Pericial — Firma Pericial Mallen</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;800&family=JetBrains+Mono:wght@400;500;600&display=swap" rel="stylesheet">
    <style>body { font-family: 'Inter', sans-serif; } .font-mono-code { font-family: 'JetBrains Mono', monospace; }</style>
    <link rel="stylesheet" href="/css/quantum-glass.css?v=2026.27">
</head>
<body class="bg-slate-950 text-slate-100 min-h-screen p-6 sm:p-10 border-t-4 border-blue-500">
    <div class="max-w-4xl mx-auto">
        <div class="flex items-center justify-between pb-5 mb-6 border-b border-white/15">
            <div>
                <span class="text-xs font-bold text-blue-400 font-mono-code uppercase block">CÓDIGO DE COMERCIO ARTS. 89–93 • LFPC ART. 76 BIS • DATOS ABIERTOS</span>
                <h1 class="text-2xl sm:text-3xl font-extrabold text-white mt-1">Términos de Servicio, Alcance Pericial y Propiedad Intelectual</h1>
            </div>
            <a href="/" class="px-4 py-2 rounded-xl bg-slate-900 border border-white/15 text-xs font-mono-code text-blue-300 hover:bg-slate-800 transition">← Portal Principal</a>
        </div>

        <div class="quantum-laser-console rounded-2xl p-6 sm:p-8 space-y-5 text-xs sm:text-sm text-slate-300 leading-relaxed font-mono-code">
            <div>
                <h2 class="text-sm font-bold text-blue-400 uppercase mb-1">1. Naturaleza Preventiva del Diagnóstico y Deslinde de Responsabilidad Oficial</h2>
                <p>Los diagnósticos, matrices de multas en UMAs, dictámenes ejecutivos, certificados privados de auditoría y modelos contractuales generados por <strong>Firma Pericial Mallen (mallen.lat)</strong> constituyen una <strong>herramienta tecnológica de autodiagnóstico preventivo y de apoyo al cumplimiento corporativo (Compliance GRC)</strong> basada en la telemetría pública del dominio evaluado, los padrones públicos de datos abiertos y las declaraciones proporcionadas bajo protesta de decir verdad por el propio usuario en la Cédula de Calibración.</p>
                <p class="mt-2"><strong>Firma Pericial Mallen es un ente privado independiente y NO actúa como unidad de verificación gubernamental ni autoridad administrativa.</strong> La implementación física en sitio, la recabación de firmas autógrafas de trabajadores y comisiones mixtas, y la presentación de avisos en portales gubernamentales (IDSE, SUA, Buzón Tributario, REPSE, DIGIPRIS, RRPAC) son responsabilidad exclusiva del patrón o contribuyente.</p>
            </div>

            <div>
                <h2 class="text-sm font-bold text-blue-400 uppercase mb-1">2. Validez Criptográfica SHA-256 / Árbol de Merkle vs. PSC NOM-151</h2>
                <p>Cada expediente emitido por la plataforma genera una huella digital criptográfica inmutable bajo el algoritmo <strong>SHA-256</strong> y una raíz de <strong>Árbol de Merkle</strong> verificable públicamente conforme a los principios de integridad y atribución de Mensajes de Datos de los <strong>Artículos 89, 89 bis, 90 y 93 del Código de Comercio</strong>. Cuando el cliente requiera adicionalmente una constancia de conservación emitida por un Prestador de Servicios de Certificación (PSC) acreditado ante la Secretaría de Economía bajo la NOM-151-SCFI-2016, los documentos del Kit se entregan en formato estándar listos para su timbrado ante el PSC de su elección.</p>
            </div>

            <div>
                <h2 class="text-sm font-bold text-blue-400 uppercase mb-1">3. Entrega Inmediata de Contenidos Digitales y Política de Reembolsos (Arts. 56 y 76 BIS LFPC)</h2>
                <p>Todos los precios exhibidos en el portal se expresan en <strong>Moneda Nacional (Pesos Mexicanos MXN)</strong>. Tratándose de <strong>bienes digitales intangibles compilados a la medida de la Razón Social y RFC del usuario (archivos PDF y Word editables entregados en paquete .ZIP de descarga instantánea)</strong>, al solicitar la generación y descarga inmediata del expediente, el usuario consiente que el servicio ha sido ejecutado y entregado en su totalidad en el acto, generándose un <strong>Acuse de Entrega Criptográfico (Art. 91 Código de Comercio)</strong> con sello de tiempo UTC y huella SHA-256 del archivo entregado. En caso de cualquier error técnico en la compilación o en los datos del expediente, el usuario cuenta con <strong>garantía de re-emisión y corrección sin costo durante 90 días naturales (Art. 77 LFPC)</strong> escribiendo a <strong>soporte@mallen.lat</strong>.</p>
            </div>

            <div>
                <h2 class="text-sm font-bold text-blue-400 uppercase mb-1">4. Atribución de Datos Abiertos Gubernamentales (INEGI / SAT / DOF)</h2>
                <p>En cumplimiento a los Términos de Libre Uso de la Información Estadística y Geográfica del INEGI y de Datos Abiertos del Gobierno de México: los datos de unidades económicas, claves SCIAN y listados de los Artículos 69 y 69-B del CFF provienen de fuentes públicas oficiales. Su procesamiento estadístico y cruce normativo es realizado de manera independiente por Firma Pericial Mallen y no compromete ni representa la opinión oficial del INEGI, SAT, STPS, IMSS, SABG ni PROFECO.</p>
            </div>
        </div>
    </div>
</body>
</html>"##.to_string())
}

/// 3. Ventanilla Automatizada de Derechos ARCO y Purga de Caché en 1 Clic (/derechos-arco)
pub fn render_mallen_arco_portal() -> Html<String> {
    Html(r##"<!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Ventanilla Digital de Derechos ARCO y Purga de Caché — Firma Pericial Mallen</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;800&family=JetBrains+Mono:wght@400;500;600&display=swap" rel="stylesheet">
    <style>body { font-family: 'Inter', sans-serif; } .font-mono-code { font-family: 'JetBrains Mono', monospace; }</style>
    <link rel="stylesheet" href="/css/quantum-glass.css?v=2026.27">
</head>
<body class="bg-slate-950 text-slate-100 min-h-screen p-6 sm:p-10 border-t-4 border-cyan-500">
    <div class="max-w-3xl mx-auto">
        <div class="flex items-center justify-between pb-5 mb-6 border-b border-white/15">
            <div>
                <span class="text-xs font-bold text-cyan-400 font-mono-code uppercase block">ARTÍCULOS 22 A 35 LFPDPPP • AUTORIDAD GARANTE SABG</span>
                <h1 class="text-2xl sm:text-3xl font-extrabold text-white mt-1">Ventanilla Automatizada de Derechos ARCO y Baja de Caché</h1>
            </div>
            <a href="/" class="px-4 py-2 rounded-xl bg-slate-900 border border-white/15 text-xs font-mono-code text-cyan-300 hover:bg-slate-800 transition">← Portal Principal</a>
        </div>

        <div class="quantum-laser-console rounded-2xl p-6 sm:p-8 font-mono-code text-xs">
            <p class="text-slate-300 leading-relaxed mb-6">
                En cumplimiento estricto a los Artículos 22 al 35 de la LFPDPPP, si usted es titular de un RFC (Persona Física con Actividad Empresarial) o representante de un dominio evaluado y desea <strong>cancelar/eliminar de inmediato cualquier diagnóstico temporal almacenado en el caché de este servidor</strong> o ejercer sus derechos de Acceso, Rectificación, Cancelación u Oposición, ingrese el identificador a continuación:
            </p>

            <form onsubmit="submitArcoOptout(event)" class="space-y-4">
                <div>
                    <label class="block text-[10px] text-cyan-300 uppercase font-bold mb-1">RFC, Razón Social o Dominio Web a Purgar de Caché *</label>
                    <input id="arcoTarget" type="text" required placeholder="Ej. GOMA850101XXX o midominio.com.mx"
                        class="w-full rounded-xl px-4 py-3 text-xs text-white">
                </div>
                <div>
                    <label class="block text-[10px] text-cyan-300 uppercase font-bold mb-1">Correo Electrónico para Recibir su Folio de Cancelación *</label>
                    <input id="arcoEmail" type="email" required placeholder="titular@empresa.com.mx"
                        class="w-full rounded-xl px-4 py-3 text-xs text-white">
                </div>
                <div>
                    <label class="block text-[10px] text-cyan-300 uppercase font-bold mb-1">Tipo de Solicitud ARCO</label>
                    <select id="arcoType" class="w-full bg-slate-900 border border-white/15 rounded-xl px-4 py-3 text-xs text-white">
                        <option value="CANCELACION_PURGA_CACHE">Cancelación y Purga Inmediata de Diagnóstico en Caché</option>
                        <option value="OPOSICION_TRATAMIENTO">Oposición a Futuros Escaneos de Telemetría Web (Opt-Out)</option>
                        <option value="ACCESO_RECTIFICACION">Acceso o Rectificación de Datos en Cédula</option>
                    </select>
                </div>
                <button type="submit" class="w-full bg-gradient-to-r from-cyan-600 to-emerald-600 hover:from-cyan-500 hover:to-emerald-500 text-white font-extrabold py-3.5 rounded-xl uppercase tracking-wider transition">
                    Ejecutar Purga Inmediata y Emitir Acuse Oficial ARCO →
                </button>
            </form>

            <div id="arcoResultBox" class="hidden mt-6 bg-emerald-950/40 border border-emerald-500/40 rounded-xl p-4 text-emerald-200"></div>
        </div>
    </div>

    <script>
        async function submitArcoOptout(e) {
            e.preventDefault();
            const target = document.getElementById("arcoTarget").value.trim();
            const email = document.getElementById("arcoEmail").value.trim();
            const motivo = document.getElementById("arcoType").value;
            const resp = await fetch("/api/arco_optout", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({ target, email, motivo })
            });
            const data = await resp.json();
            const box = document.getElementById("arcoResultBox");
            box.classList.remove("hidden");
            box.innerHTML = `
                <strong class="text-emerald-400 block uppercase mb-1"> Solicitud Procesada y Caché Purgado con Éxito</strong>
                <div>Folio Oficial ARCO: <span class="text-white font-bold">${data.folio}</span></div>
                <div>Identificador Purgado: <span class="text-white">${data.purged_target}</span></div>
                <div>Estampa UTC: <span class="text-slate-300">${data.timestamp_utc}</span></div>
                <div class="mt-1 text-[11px] text-emerald-300">Se ha eliminado cualquier copia temporal del diagnóstico en <code>audit_cache</code> y registrado su acuse conforme al Art. 32 LFPDPPP.</div>
            `;
        }
    </script>
</body>
</html>"##.to_string())
}
