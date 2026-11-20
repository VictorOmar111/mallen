/**
 * FIRMA MALLEN® — Centro de Preguntas Frecuentes (Apartados 01 al 06),
 * Botón Único de Modo Día en Footer y Enlaces Directos de App por OS (v100)
 */
(function () {
  "use strict";

  const FAQ_APARTADOS = [
    {
      num: "APARTADO 01",
      titulo: "Alcance del Diagnóstico, Personas Físicas y Fuentes Oficiales",
      preguntas: [
        {
          q: "¿El diagnóstico y el expediente preventivo aplican también para Personas Físicas o únicamente para personas morales?",
          a: "También aplica para Personas Físicas con actividad empresarial, profesional o comercial cuando cuentan con personal, emiten o reciben comprobantes fiscales, operan un espacio de trabajo o recaban datos de clientes y colaboradores. Se evalúan las obligaciones laborales, fiscales y de privacidad pertinentes sin imponer requisitos exclusivos de personas morales."
        },
        {
          q: "¿Con qué metodología determina la plataforma la exposición económica sin acceder a la contabilidad interna?",
          a: "Cruza el giro SCIAN y el estrato operativo del padrón oficial INEGI DENUE con el número real de colaboradores y las tablas de sanciones en UMAs vigentes de la LFT, LSS, CFF, LFPDPPP y LFPC."
        },
        {
          q: "¿Cómo procede el sistema si la información registrada en padrones públicos difiere de la operación actual de la empresa?",
          a: "Los padrones públicos pueden presentar desfase temporal respecto de cambios recientes de domicilio, plantilla, actividad o estructura operativa. Por ello, el sistema permite revisar y calibrar los parámetros de la empresa para que el diagnóstico y los documentos se ajusten a la situación real vigente."
        }
      ]
    },
    {
      num: "APARTADO 02",
      titulo: "Validez ante Autoridades, Alcance de Responsabilidad y Metodología",
      preguntas: [
        {
          q: "¿Tiene el dictamen con sello criptográfico SHA-256 y código QR carácter vinculante para eximir sanciones ante una inspección de la STPS o del IMSS?",
          a: "No existe en la Ley Federal del Trabajo (LFT) ni en la Ley del Seguro Social (LSS) un precepto que obligue a un inspector federal del trabajo o a un auditor del IMSS a tener por desvirtuada una infracción, a cerrar una visita o a eximir de sanción únicamente por la exhibición de un documento digital privado con folio SHA-256 o código QR. En el procedimiento administrativo de inspección y comprobación, la autoridad actúa conforme a sus facultades legales y evalúa el cumplimiento material y documental de cada obligación específica según las constancias y evidencias recabadas en la diligencia. Se comunica como herramienta de blindaje preventivo porque permite a la organización diagnosticar omisiones antes de que ocurra una visita, ordenar y estructurar los instrumentos internos que la normativa contempla (políticas, actas, comisiones, avisos, controles y bitácoras) y contar con un expediente trazable e íntegro —apoyado en los principios de integridad y conservación de mensajes de datos de los Artículos 89 a 93 del Código de Comercio— que respalde la diligencia probatoria de la empresa y apoye la defensa técnica o la corrección oportuna."
        },
        {
          q: "¿Asume FIRMA MALLEN® responsabilidad solidaria, garantía financiera o fianza ante multas administrativas o litigios laborales derivados de la implementación de los documentos?",
          a: "No. La contratación o descarga del expediente digital no constituye una póliza de fianza, seguro de caución, garantía financiera ni asunción de responsabilidad civil solidaria, subsidiaria o indemnizatoria por multas administrativas, créditos fiscales, capitales constitutivos, recargos, actualizaciones o condenas e indemnizaciones laborales derivadas de conflictos individuales o colectivos con trabajadores. Los documentos, políticas, actas y formatos entregados constituyen ingeniería documental preventiva y marcos de referencia que deben ser revisados, calibrados a la operación real del centro de trabajo, completados con los datos específicos de la organización, suscritos por quienes cuenten con facultades legales y ejecutados de manera efectiva en la operación diaria."
        },
        {
          q: "¿Cuenta la plataforma con certificación de tercera parte (EMA, TÜV, BSI) o supervisión regulatoria extranjera (SEC / FINRA) para emitir dictámenes oficiales?",
          a: "La plataforma no cuenta con una certificación de tercera parte emitida por organismos de evaluación de la conformidad o acreditación (como EMA, TÜV o BSI) que certifique al software como entidad dictaminadora oficial en materia laboral, fiscal o de seguridad social, ni opera como entidad regulada o supervisada bajo marcos de mercados de valores extranjeros como la SEC o FINRA. Las referencias a ISO 19011 (directrices para auditorías), IIA (estándares de auditoría interna) o principios de integridad de mensajes de datos y trazabilidad criptográfica representan criterios metodológicos y de diseño técnico incorporados en la estructura de evaluación y en la huella de integridad del expediente."
        },
        {
          q: "¿Sustituye el expediente automatizado la representación jurídica o la asesoría contable personalizada ante una auditoría en curso?",
          a: "No. El diagnóstico y el expediente documental preventivo son herramientas de análisis de riesgo y preparación documental, pero no sustituyen la representación jurídica especializada, la defensa en procedimientos administrativos o litigiosos ni el dictamen o criterio contable-fiscal profesional ante una auditoría ya iniciada o una controversia concreta."
        }
      ]
    },
    {
      num: "APARTADO 03",
      titulo: "Materialidad Fiscal (SAT Arts. 5-A y 69-B) y Fecha Cierta",
      preguntas: [
        {
          q: "¿Qué alcance tiene el cálculo preliminar basado en estratos estadísticos del INEGI frente a los requisitos de materialidad y razón de negocios del Artículo 5-A del CFF?",
          a: "El Artículo 5-A del Código Fiscal de la Federación (CFF) exige que los actos jurídicos con efectos fiscales tengan una razón de negocios real y que la materialidad y sustancia económica de las operaciones se acrediten con elementos objetivos y comprobables (contratos, entregables, trazabilidad, ejecución efectiva y correspondencia contable-financiera), no con simples estimaciones estadísticas. Cuando el diagnóstico preliminar utiliza un estrato censal del INEGI DENUE o una mediana estadística antes de que el usuario capture su plantilla exacta, ese cálculo cumple una función estimativa y orientativa de exposición para dimensionar el riesgo inicial. Para que los instrumentos sirvan como base documental, es indispensable calibrar el número exacto de colaboradores y los datos operativos reales de la empresa y resguardar la evidencia de ejecución efectiva."
        },
        {
          q: "¿Cómo se complementan la razón de negocios (Artículo 5-A del CFF) y la fecha cierta en la defensa de operaciones privadas (Artículo 69-B del CFF)?",
          a: "La razón de negocios se enfoca en el beneficio económico razonablemente esperado y en la sustancia real de la operación más allá del beneficio fiscal, mientras que la fecha cierta busca acreditar de manera fehaciente cuándo se celebró o perfeccionó un documento privado frente a terceros y autoridades fiscales. En la práctica contable y jurídica, ambos elementos son complementarios: un contrato debe responder a una necesidad operativa real del giro y contar con soporte documental, trazabilidad y constancia de celebración y ejecución."
        },
        {
          q: "¿Qué elementos documentales fortalecen la materialidad de servicios intangibles, consultoría, desarrollo tecnológico o mercadotecnia?",
          a: "En operaciones de servicios intangibles conviene integrar un expediente probatorio coherente que incluya contrato detallado con alcance y entregables definidos, constancias de fecha cierta o trazabilidad de celebración, cotizaciones o propuestas, entregables técnicos o informes verificables, bitácoras o comunicaciones de avance, comprobantes de ejecución y correspondencia clara entre el comprobante fiscal, el pago y el registro contable."
        }
      ]
    },
    {
      num: "APARTADO 04",
      titulo: "Normatividad Laboral (STPS), Seguridad Social (IMSS / REPSE) y Protección Civil",
      preguntas: [
        {
          q: "¿Qué contingencias laborales, fiscales y de seguridad social pueden derivarse de una contratación deficiente de servicios especializados?",
          a: "Las contingencias surgen cuando se simulan esquemas de subcontratación de personal, se contratan servicios que no son especializados respecto de la actividad de la beneficiaria, se omiten registros, avisos o reportes informativos exigidos por la ley o no se verifica y conserva la documentación de cumplimiento de la empresa contratista. En materia fiscal y de seguridad social, estas deficiencias pueden derivar en el rechazo de deducciones o acreditamientos y determinación de diferencias."
        },
        {
          q: "¿De qué manera influyen la clasificación de actividad y la siniestralidad en la prima del Seguro de Riesgos de Trabajo ante el IMSS?",
          a: "La prima del Seguro de Riesgos de Trabajo se relaciona con la clase asignada a la actividad declarada del centro de trabajo y con la siniestralidad de los casos de riesgos de trabajo registrados en el periodo correspondiente. Una clasificación inadecuada frente a la actividad real o la falta de prevención de accidentes y enfermedades de trabajo puede generar ajustes, diferencias o incrementos en las cuotas patronales."
        },
        {
          q: "¿Qué condiciones determinan la eficacia preventiva y probatoria de las políticas y evaluaciones de riesgo psicosocial?",
          a: "La elaboración y aplicación de políticas, medidas de prevención y cuestionarios es un componente esencial del cumplimiento en materia de factores de riesgo psicosocial y entorno organizacional, pero su valor preventivo y probatorio depende de que se adapten al tamaño y características reales del centro de trabajo, se difundan adecuadamente, se canalicen los hallazgos o quejas y se ejecuten acciones de seguimiento y control en la práctica."
        },
        {
          q: "¿Cómo se determina la exigibilidad del Programa Interno de Protección Civil cuando una organización opera en diversas entidades federativas?",
          a: "La exigibilidad, modalidad y requisitos del Programa Interno de Protección Civil dependen de la normativa de protección civil de cada entidad federativa y municipio, así como de factores como la superficie, aforo, giro y nivel de riesgo del inmueble. Cuando una organización cuenta con sucursales o centros de trabajo en distintos estados o municipios, debe revisar y cumplir las disposiciones locales específicas de cada ubicación además de las normas federales aplicables."
        }
      ]
    },
    {
      num: "APARTADO 05",
      titulo: "Protección de Datos (LFPDPPP), Información Biométrica y Entorno Digital",
      preguntas: [
        {
          q: "¿Es suficiente publicar un Aviso de Privacidad en el sitio web para cumplir integralmente con el tratamiento de datos personales, sensibles o biométricos?",
          a: "Publicar un aviso de privacidad cumple con el deber de informar a los titulares sobre las finalidades y condiciones del tratamiento, pero el cumplimiento integral exige además contar con base de legitimación o consentimiento válido —especialmente consentimiento expreso y por escrito cuando se tratan datos sensibles, de salud o biométricos—, implementar medidas de seguridad administrativas, técnicas y físicas acordes al riesgo, regular a los encargados y atender los derechos de acceso, rectificación, cancelación y oposición."
        },
        {
          q: "¿Qué obligaciones jurídicas y fiscales deben observarse en el comercio electrónico, portales digitales y canales de atención a clientes?",
          a: "Las empresas que comercializan productos o servicios por medios digitales deben asegurar claridad en la información previa a la contratación, precios, condiciones de entrega, cambios o devoluciones y garantías, contar con políticas de privacidad y seguridad acordes a los datos recabados, mantener canales de atención accesibles y cuidar la correcta emisión, validación y conservación de comprobantes fiscales y registros electrónicos."
        },
        {
          q: "¿Cómo debe formalizarse jurídicamente la relación con despachos externos, proveedores de nómina o plataformas en la nube que tratan datos personales?",
          a: "Cuando un proveedor externo o despacho trata datos personales por cuenta del responsable, es necesario formalizar por escrito los términos del encargo estableciendo que el tratamiento se realizará únicamente conforme a las instrucciones del responsable, prohibiendo finalidades distintas, exigiendo medidas de seguridad adecuadas, regulando sub-encargos y definiendo el tratamiento o devolución de la información al concluir el servicio."
        }
      ]
    },
    {
      num: "APARTADO 06",
      titulo: "Gestión del Expediente, Confidencialidad, Reformas y Revisión Profesional",
      preguntas: [
        {
          q: "¿Qué aspectos esenciales conviene que revise un abogado o contador público antes de implementar un expediente preventivo en la empresa?",
          a: "El profesional debe verificar la exactitud de los datos de identidad, régimen, giro, plantilla y centros de trabajo, contrastar los supuestos normativos con la operación real del cliente, completar los campos específicos de representantes, comisiones, contrapartes o fechas, y supervisar que los documentos seleccionados se firmen, difundan y apliquen de forma congruente con la estrategia legal y fiscal de la empresa."
        },
        {
          q: "¿Cómo impactan las reformas normativas o las actualizaciones anuales de la UMA a un expediente emitido con anterioridad?",
          a: "La normatividad laboral, fiscal, de seguridad social y de protección de datos, así como el valor de la UMA y los padrones públicos, pueden actualizarse periódicamente. Cada expediente refleja el análisis y los parámetros vigentes al momento de su emisión y calibración, por lo que ante cambios legislativos o de plantilla conviene revisar y actualizar los instrumentos correspondientes."
        },
        {
          q: "¿Qué medidas de seguridad protegen la confidencialidad de los datos proporcionados durante la calibración del expediente?",
          a: "La plataforma aplica controles de seguridad de la información, cifrado en tránsito y resguardo de registros orientados a proteger la integridad y confidencialidad de los datos procesados, limitando su uso a la generación, verificación y trazabilidad del expediente solicitado conforme a las políticas de privacidad aplicables."
        },
        {
          q: "¿Con qué finalidad se entregan instrumentos en formato editable y qué cuidados deben observarse al adaptarlos?",
          a: "Los formatos editables permiten a la empresa y a su área legal o contable adaptar cláusulas, anexos, firmantes o detalles operativos específicos de cada contrato o política interna. Al editar cualquier documento, debe cuidarse de no alterar referencias normativas esenciales, alcances de confidencialidad o congruencia con los demás instrumentos del expediente, verificando siempre su adecuación legal final antes de firmarlo."
        }
      ]
    }
  ];

  function mountStructuredFaqSection() {
    const faqRoot =
      document.getElementById("faq") ||
      Array.from(document.querySelectorAll("section, div")).find(function (el) {
        return (el.textContent || "").indexOf("Preguntas Frecuentes") !== -1;
      });
    if (!faqRoot) return;

    if (document.getElementById("mallen-faq-6-apartados-v100")) return;

    const badgeSpan = faqRoot.querySelector("div.text-center span");
    if (badgeSpan) {
      badgeSpan.textContent = "CENTRO DE CERTEZA METODOLÓGICA, ALCANCE JURÍDICO & GUÍA OPERATIVA (6 APARTADOS)";
    }
    const subP = faqRoot.querySelector("div.text-center p");
    if (subP) {
      subP.textContent = "Seleccione cualquiera de los 6 apartados temáticos para desplegar las respuestas detalladas sobre alcance, validez jurídica, materialidad fiscal, seguridad social y operación del expediente.";
    }

    let hostContainer = faqRoot.querySelector(".max-w-5xl");
    if (!hostContainer) {
      hostContainer = document.createElement("div");
      hostContainer.className = "space-y-4 max-w-5xl mx-auto text-left";
      faqRoot.appendChild(hostContainer);
    }
    hostContainer.innerHTML = "";
    hostContainer.id = "mallen-faq-6-apartados-v100";

    FAQ_APARTADOS.forEach(function (ap, idx) {
      const catDetails = document.createElement("details");
      if (idx === 0) catDetails.open = true;
      catDetails.className = "faq-cat rounded-2xl border border-slate-700/80 bg-slate-900/70 p-5 sm:p-6 shadow-lg";

      const catSummary = document.createElement("summary");
      catSummary.className = "flex cursor-pointer list-none items-center justify-between gap-4 select-none";
      catSummary.innerHTML =
        '<div class="flex items-center gap-3 text-left">' +
          '<span class="px-2.5 py-1 rounded-md text-xs font-mono font-bold bg-emerald-500/15 text-emerald-400 border border-emerald-500/30 shrink-0">' + ap.num + '</span>' +
          '<span class="font-extrabold text-base sm:text-lg tracking-tight text-white">' + ap.titulo + '</span>' +
        '</div>' +
        '<span class="flex h-7 w-7 shrink-0 items-center justify-center rounded-full border border-slate-600 text-xs font-mono">▼</span>';

      const innerList = document.createElement("div");
      innerList.className = "mt-5 space-y-3 border-t border-slate-800 pt-4";

      ap.preguntas.forEach(function (item) {
        const sub = document.createElement("details");
        sub.className = "faq-sub rounded-xl border border-slate-800 bg-slate-950/60 p-4";

        const subSum = document.createElement("summary");
        subSum.className = "flex cursor-pointer list-none items-start justify-between gap-4 font-semibold text-xs sm:text-sm text-left text-white";
        subSum.innerHTML = '<span class="flex-1">' + item.q + '</span><span class="text-slate-400 font-mono text-xs shrink-0">+</span>';

        const subP = document.createElement("p");
        subP.className = "mt-3 text-slate-300 leading-relaxed text-xs sm:text-sm border-t border-slate-800/70 pt-3";
        subP.textContent = item.a;

        sub.appendChild(subSum);
        sub.appendChild(subP);
        innerList.appendChild(sub);
      });

      catDetails.appendChild(catSummary);
      catDetails.appendChild(innerList);
      hostContainer.appendChild(catDetails);
    });
  }

  function getActiveRegionLocale() {
    const euCode = (document.body && document.body.getAttribute("data-eu-country") || "").toLowerCase();
    if (euCode === "de" || euCode === "at" || euCode === "ch") return "de";
    if (euCode === "nl" || euCode === "be") return "nl";
    if (euCode === "lu") return "fr";
    return "es";
  }

  const I18N_FOOTER = {
    es: {
      dayBtn: "Cambiar a Modo Día",
      darkBtn: "Cambiar a Modo Oscuro",
      appLbl: "Descargar App:",
      regLbl: "Región:",
      contactTitle: "¿Prefieres que un especialista te contacte?",
      contactSub: "Deja únicamente tu correo corporativo o personal y te escribimos en breve.",
      contactPh: "tu@correo.com",
      contactBtn: "Contactarme",
      contactSending: "Enviando...",
      contactOk: "Correo registrado. Te escribiremos en breve.",
      contactErr: "Error de conexión. Intente de nuevo.",
      close: "Cerrar ✕"
    },
    de: {
      dayBtn: "Heller Modus",
      darkBtn: "Dunkler Modus",
      appLbl: "App herunterladen:",
      regLbl: "Region:",
      contactTitle: "Möchten Sie von einem Spezialisten kontaktiert werden?",
      contactSub: "Hinterlassen Sie einfach Ihre E-Mail-Adresse – wir melden uns kurzfristig.",
      contactPh: "name@unternehmen.de",
      contactBtn: "Kontakt anfordern",
      contactSending: "Wird gesendet...",
      contactOk: "E-Mail gespeichert. Wir melden uns in Kürze bei Ihnen.",
      contactErr: "Verbindungsfehler. Bitte erneut versuchen.",
      close: "Schließen ✕"
    },
    nl: {
      dayBtn: "Lichte modus",
      darkBtn: "Donkere modus",
      appLbl: "App downloaden:",
      regLbl: "Regio:",
      contactTitle: "Liever persoonlijk contact met een specialist?",
      contactSub: "Laat alleen uw e-mailadres achter en wij nemen spoedig contact op.",
      contactPh: "naam@bedrijf.nl",
      contactBtn: "Neem contact op",
      contactSending: "Verzenden...",
      contactOk: "E-mailadres opgeslagen. Wij nemen snel contact op.",
      contactErr: "Verbindingsfout. Probeer het opnieuw.",
      close: "Sluiten ✕"
    },
    fr: {
      dayBtn: "Mode clair",
      darkBtn: "Mode sombre",
      appLbl: "Télécharger l’App :",
      regLbl: "Région :",
      contactTitle: "Souhaitez-vous être contacté par un spécialiste ?",
      contactSub: "Laissez simplement votre adresse e-mail et nous vous répondrons rapidement.",
      contactPh: "nom@entreprise.lu",
      contactBtn: "Être contacté",
      contactSending: "Envoi...",
      contactOk: "Adresse enregistrée. Nous vous contacterons sous peu.",
      contactErr: "Erreur de connexion. Veuillez réessayer.",
      close: "Fermer ✕"
    }
  };

  function getActiveRegionLocale() {
    const euCode = (document.body && document.body.getAttribute("data-eu-country") || "").toLowerCase();
    if (euCode === "de" || euCode === "at" || euCode === "ch") return "de";
    if (euCode === "nl" || euCode === "be") return "nl";
    if (euCode === "lu") return "fr";
    return "es";
  }

  const I18N_FOOTER = {
    es: {
      dayBtn: "Cambiar a Modo Día",
      darkBtn: "Cambiar a Modo Oscuro",
      appLbl: "Descargar App:",
      regLbl: "Región:",
      contactTitle: "¿Prefieres que un especialista te contacte?",
      contactSub: "Deja únicamente tu correo corporativo o personal y te escribimos en breve.",
      contactPh: "tu@correo.com",
      contactBtn: "Contactarme",
      contactSending: "Enviando...",
      contactOk: "Correo registrado. Te escribiremos en breve.",
      contactErr: "Error de conexión. Intente de nuevo.",
      close: "Cerrar ✕"
    },
    de: {
      dayBtn: "Heller Modus",
      darkBtn: "Dunkler Modus",
      appLbl: "App herunterladen:",
      regLbl: "Region:",
      contactTitle: "Möchten Sie von einem Spezialisten kontaktiert werden?",
      contactSub: "Hinterlassen Sie einfach Ihre E-Mail-Adresse – wir melden uns kurzfristig.",
      contactPh: "name@unternehmen.de",
      contactBtn: "Kontakt anfordern",
      contactSending: "Wird gesendet...",
      contactOk: "E-Mail gespeichert. Wir melden uns in Kürze bei Ihnen.",
      contactErr: "Verbindungsfehler. Bitte erneut versuchen.",
      close: "Schließen ✕"
    },
    nl: {
      dayBtn: "Lichte modus",
      darkBtn: "Donkere modus",
      appLbl: "App downloaden:",
      regLbl: "Regio:",
      contactTitle: "Liever persoonlijk contact met een specialist?",
      contactSub: "Laat alleen uw e-mailadres achter en wij nemen spoedig contact op.",
      contactPh: "naam@bedrijf.nl",
      contactBtn: "Neem contact op",
      contactSending: "Verzenden...",
      contactOk: "E-mailadres opgeslagen. Wij nemen snel contact op.",
      contactErr: "Verbindingsfout. Probeer het opnieuw.",
      close: "Sluiten ✕"
    },
    fr: {
      dayBtn: "Mode clair",
      darkBtn: "Mode sombre",
      appLbl: "Télécharger l’App :",
      regLbl: "Région :",
      contactTitle: "Souhaitez-vous être contacté par un spécialiste ?",
      contactSub: "Laissez simplement votre adresse e-mail et nous vous répondrons rapidement.",
      contactPh: "nom@entreprise.lu",
      contactBtn: "Être contacté",
      contactSending: "Envoi...",
      contactOk: "Adresse enregistrée. Nous vous contacterons sous peu.",
      contactErr: "Erreur de connexion. Veuillez réessayer.",
      close: "Fermer ✕"
    }
  };

  window.toggleMallenTheme = function () {
    const root = document.documentElement;
    const body = document.body;
    const isLight = !root.classList.contains("light-mode");
    root.classList.toggle("light-mode", isLight);
    if (body) body.classList.toggle("light-mode", isLight);
    try { localStorage.setItem("mallen_theme_mode", isLight ? "light" : "dark"); } catch (_) {}
    updateBottomThemeButtonLabel();
  };
  window.toggleW16PureLightMode = window.toggleMallenTheme;

  function updateBottomThemeButtonLabel() {
    const loc = getActiveRegionLocale();
    const t = I18N_FOOTER[loc] || I18N_FOOTER.es;
    const isLight = document.documentElement.classList.contains("light-mode");
    const btn = document.getElementById("mallen-bottom-theme-btn");
    if (btn) btn.textContent = isLight ? t.darkBtn : t.dayBtn;
  }

  window.openMallenOsInstall = function (osName) {
    if ((osName === "android" || osName === "desktop") && typeof window.openMallenAppInstaller === "function") {
      window.openMallenAppInstaller();
    }
    const modal = document.getElementById("mallen-pwa-install-modal");
    const backdrop = document.getElementById("mallen-pwa-modal-backdrop");
    if (!modal || !backdrop) return;
    const loc = getActiveRegionLocale();
    const t = I18N_FOOTER[loc] || I18N_FOOTER.es;

    const guides = {
      es: {
        android: "1. Si aparece el aviso nativo, pulse <strong>Instalar</strong>.<br>2. O abra el menú <strong>⋮</strong> y elija <strong>Instalar aplicación</strong>.",
        ios: "1. Abra en <strong>Safari</strong>, toque <strong>Compartir</strong> y elija <strong>Agregar a pantalla de inicio</strong>.",
        desktop: "1. En Chrome o Edge, haga clic en el ícono <strong>Instalar</strong> en la barra de direcciones."
      },
      de: {
        android: "1. Tippen Sie auf <strong>Installieren</strong> oder öffnen Sie das Browser-Menü <strong>⋮</strong> → <strong>App installieren</strong>.",
        ios: "1. In <strong>Safari</strong> auf <strong>Teilen</strong> tippen und <strong>Zum Home-Bildschirm</strong> wählen.",
        desktop: "1. In Chrome oder Edge rechts in der Adressleiste auf <strong>App installieren</strong> klicken."
      },
      nl: {
        android: "1. Tik op <strong>Installeren</strong> of open het menu <strong>⋮</strong> → <strong>App installeren</strong>.",
        ios: "1. Tik in <strong>Safari</strong> op <strong>Deel</strong> en kies <strong>Zet op beginscherm</strong>.",
        desktop: "1. Klik in Chrome of Edge rechts in de adresbalk op <strong>App installeren</strong>."
      },
      fr: {
        android: "1. Appuyez sur <strong>Installer</strong> ou ouvrez le menu <strong>⋮</strong> → <strong>Installer l’application</strong>.",
        ios: "1. Dans <strong>Safari</strong>, appuyez sur <strong>Partager</strong> puis <strong>Sur l’écran d’accueil</strong>.",
        desktop: "1. Dans Chrome ou Edge, cliquez sur l’icône <strong>Installer</strong> dans la barre d’adresse."
      }
    };
    const g = guides[loc] || guides.es;

    modal.innerHTML =
      '<div class="flex items-center justify-between pb-3 mb-4 border-b border-slate-800">' +
        '<div>' +
          '<span class="text-xs font-mono-data uppercase text-emerald-400 block">FIRMA MALLEN® APP</span>' +
          '<h3 class="text-lg font-extrabold text-white">' + osName.toUpperCase() + '</h3>' +
        '</div>' +
        '<button type="button" onclick="window.closeMallenAppInstaller()" class="mallen-btn-secondary px-3 py-1 text-xs">' + t.close + '</button>' +
      '</div>' +
      '<div class="p-4 rounded-xl border border-slate-800 bg-slate-950/70 text-sm text-slate-200 leading-relaxed">' +
        (g[osName] || g.android) +
      '</div>';

    modal.classList.add("open");
    backdrop.classList.add("open");
  };

  function setupFooterControlsOnly() {
    document.querySelectorAll("header [data-mallen-theme-btn], #eu-theme-toggle-btn, #mallen-install-app-global-btn").forEach(function (el) {
      el.style.display = "none";
    });

    try {
      if (localStorage.getItem("mallen_theme_mode") === "light") {
        document.documentElement.classList.add("light-mode");
        if (document.body) document.body.classList.add("light-mode");
      }
    } catch (_) {}

    const footer = document.querySelector("footer");
    if (!footer) return;
    const loc = getActiveRegionLocale();
    const t = I18N_FOOTER[loc] || I18N_FOOTER.es;

    footer.querySelectorAll("button").forEach(function (btn) {
      if (btn.id === "mallen-bottom-theme-btn" || btn.id === "mallen-quick-contact-btn") return;
      const txt = (btn.textContent || "").toLowerCase();
      if (txt.indexOf("modo") !== -1 || txt.indexOf("claro") !== -1 || txt.indexOf("oscuro") !== -1 || txt.indexOf("día") !== -1) {
        btn.style.display = "none";
      }
    });

    if (!document.getElementById("mallen-quick-contact-bar")) {
      const cbar = document.createElement("div");
      cbar.id = "mallen-quick-contact-bar";
      cbar.className = "max-w-4xl mx-auto mb-6 p-4 sm:p-5 rounded-2xl border border-slate-800 bg-slate-950/70 flex flex-col sm:flex-row items-center justify-between gap-3";
      cbar.innerHTML =
        '<div class="text-left">' +
          '<div class="text-sm font-bold text-white">' + t.contactTitle + '</div>' +
          '<div class="text-xs text-slate-400">' + t.contactSub + '</div>' +
        '</div>' +
        '<form id="mallen-quick-contact-form" class="flex w-full sm:w-auto items-center gap-2">' +
          '<input id="mallen-quick-contact-email" type="email" required placeholder="' + t.contactPh + '" class="w-full sm:w-64 px-3.5 py-2 rounded-lg border border-slate-700 bg-slate-900 text-sm text-white focus:border-emerald-400 focus:outline-none" />' +
          '<button type="submit" id="mallen-quick-contact-btn" class="mallen-btn-primary px-4 py-2 text-xs font-bold shrink-0">' + t.contactBtn + '</button>' +
        '</form>' +
        '<div id="mallen-quick-contact-status" class="hidden text-xs font-semibold text-emerald-400"></div>';
      footer.insertBefore(cbar, footer.firstChild);

      const form = document.getElementById("mallen-quick-contact-form");
      if (form) {
        form.addEventListener("submit", async function (ev) {
          ev.preventDefault();
          const emailInput = document.getElementById("mallen-quick-contact-email");
          const btn = document.getElementById("mallen-quick-contact-btn");
          const statusEl = document.getElementById("mallen-quick-contact-status");
          const emailVal = emailInput ? emailInput.value.trim() : "";
          if (!emailVal) return;
          if (btn) { btn.disabled = true; btn.textContent = t.contactSending; }
          try {
            const r = await fetch("/api/quick-contact", {
              method: "POST",
              headers: { "Content-Type": "application/json" },
              body: JSON.stringify({ email: emailVal })
            });
            if (statusEl) {
              statusEl.textContent = r.ok ? t.contactOk : t.contactErr;
              statusEl.className = "text-xs font-semibold " + (r.ok ? "text-emerald-400" : "text-red-400");
            }
            if (r.ok && emailInput) emailInput.value = "";
          } catch (_) {
            if (statusEl) {
              statusEl.textContent = t.contactErr;
              statusEl.className = "text-xs font-semibold text-red-400";
            }
          } finally {
            if (btn) { btn.disabled = false; btn.textContent = t.contactBtn; }
          }
        });
      }
    }

    if (!document.getElementById("mallen-footer-utility-bar")) {
      const bar = document.createElement("div");
      bar.id = "mallen-footer-utility-bar";
      bar.className = "max-w-6xl mx-auto mt-6 pt-5 border-t border-slate-800/80 flex flex-col lg:flex-row items-center justify-between gap-4";
      bar.innerHTML =
        '<div class="flex flex-wrap items-center justify-center gap-2">' +
          '<span class="text-xs font-semibold uppercase tracking-wider text-slate-400 mr-1">' + t.regLbl + '</span>' +
          '<a href="/?country=MX" class="mallen-btn-secondary px-2.5 py-1 text-xs font-semibold border border-slate-700 hover:border-emerald-400">MX</a>' +
          '<a href="/region/de" class="mallen-btn-secondary px-2.5 py-1 text-xs font-semibold border border-slate-700 hover:border-emerald-400">DACH</a>' +
          '<a href="/region/nl" class="mallen-btn-secondary px-2.5 py-1 text-xs font-semibold border border-slate-700 hover:border-emerald-400">Benelux</a>' +
        '</div>' +
        '<div class="flex flex-wrap items-center justify-center gap-2">' +
          '<span class="text-xs font-semibold uppercase tracking-wider text-slate-400 mr-1">' + t.appLbl + '</span>' +
          '<button type="button" onclick="window.openMallenOsInstall(\'android\')" class="mallen-btn-secondary px-3 py-1 text-xs font-semibold border border-slate-700 hover:border-emerald-400">Android</button>' +
          '<button type="button" onclick="window.openMallenOsInstall(\'ios\')" class="mallen-btn-secondary px-3 py-1 text-xs font-semibold border border-slate-700 hover:border-emerald-400">iOS</button>' +
          '<button type="button" onclick="window.openMallenOsInstall(\'desktop\')" class="mallen-btn-secondary px-3 py-1 text-xs font-semibold border border-slate-700 hover:border-emerald-400">Windows / macOS</button>' +
        '</div>' +
        '<div>' +
          '<button type="button" id="mallen-bottom-theme-btn" data-mallen-theme-btn="1" onclick="window.toggleMallenTheme()" class="mallen-btn-secondary px-4 py-1.5 text-xs font-bold border border-slate-600 hover:border-white">' + t.dayBtn + '</button>' +
        '</div>';
      footer.appendChild(bar);
    }

    updateBottomThemeButtonLabel();
  }

  function initV107() {
    if (getActiveRegionLocale() === "es") {
      mountStructuredFaqSection();
    }
    setupFooterControlsOnly();
  }

  document.addEventListener("DOMContentLoaded", function () {
    initV107();
    setTimeout(initV107, 350);
  });
})();
