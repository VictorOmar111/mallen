/**
 * FIRMA MALLEN® — Centro de Preguntas Frecuentes Unificado (Apartados 01 al 06),
 * Botón Único de Modo Día en Footer y Enlaces Directos de App por OS (v98)
 */
(function () {
  "use strict";

  const FAQ_APARTADOS = [
    {
      num: "APARTADO 01",
      titulo: "Alcance del Diagnóstico, Personas Físicas y Padrón INEGI DENUE",
      preguntas: [
        {
          q: "¿La plataforma también aplica para Personas Físicas o solo para empresas (personas morales)?",
          a: "También aplica para Personas Físicas con actividad empresarial, profesional o comercial cuando cuentan con personal, emiten o reciben comprobantes fiscales, operan un espacio de trabajo o recaban datos de clientes y colaboradores. Se evalúan las obligaciones laborales, fiscales y de privacidad pertinentes sin imponer requisitos exclusivos de personas morales."
        },
        {
          q: "¿Existen registros de Personas Físicas en el INEGI DENUE o el padrón es solo para empresas?",
          a: "El INEGI DENUE indexa unidades económicas (establecimientos físicos con actividad comercial, industrial o de servicios) e incluye tanto personas morales como negocios operados por personas físicas, pero no publica el RFC por reserva fiscal. Cuando un RFC de persona física o un registro reciente no aparece en el DENUE, el motor utiliza el giro SCIAN, la entidad y el número exacto de colaboradores que usted indique."
        },
        {
          q: "¿Cómo calcula la plataforma la exposición VEMP sin tener mi contabilidad interna?",
          a: "Cruza el giro SCIAN y el estrato operativo del INEGI DENUE (5.31 millones de unidades económicas) con el número real de colaboradores y las tablas de sanciones en UMAs vigentes de la LFT, LSS, CFF, LFPDPPP y LFPC."
        },
        {
          q: "¿Qué ocurre si los datos detectados automáticamente en padrones públicos no coinciden con la operación actual de mi empresa?",
          a: "Los padrones públicos pueden presentar desfase temporal respecto de cambios recientes de domicilio, plantilla, actividad o estructura operativa. Por ello, el sistema permite revisar y calibrar los parámetros de la empresa para que el diagnóstico y los documentos se ajusten a la situación real vigente."
        }
      ]
    },
    {
      num: "APARTADO 02",
      titulo: "Validez ante Autoridades, Responsabilidad Civil y Certificación",
      preguntas: [
        {
          q: "Si una autoridad federal (ej. un inspector de la STPS o un auditor del IMSS) se presenta físicamente en mi empresa y le presento mi Dictamen con folio SHA-256 y su código QR, ¿existe fundamento jurídico en la Ley Federal del Trabajo o en la Ley del Seguro Social que obligue a la autoridad a aceptar este documento digital privado como eximente de sanción? Si la respuesta es no, ¿por qué lo publicitan como un escudo ante revisiones?",
          a: "No existe en la Ley Federal del Trabajo (LFT) ni en la Ley del Seguro Social (LSS) un precepto que obligue a un inspector federal del trabajo o a un auditor del IMSS a tener por desvirtuada una infracción, a cerrar una visita o a eximir de sanción únicamente por la exhibición de un documento digital privado con folio SHA-256 o código QR. En el procedimiento administrativo de inspección y comprobación, la autoridad actúa conforme a sus facultades legales y evalúa el cumplimiento material y documental de cada obligación específica según las constancias y evidencias recabadas en la diligencia. Se comunica como herramienta de blindaje preventivo porque permite a la organización diagnosticar omisiones antes de que ocurra una visita, ordenar y estructurar los instrumentos internos que la normativa contempla (políticas, actas, comisiones, avisos, controles y bitácoras) y contar con un expediente trazable e íntegro —apoyado en los principios de integridad y conservación de mensajes de datos de los Artículos 89 a 93 del Código de Comercio— que respalde la diligencia probatoria de la empresa y apoye la defensa técnica o la corrección oportuna."
        },
        {
          q: "Si implemento sus 7 contratos y políticas corporativas descargadas por $3,000 MXN y, a raíz de un conflicto con un trabajador o una revisión de la autoridad, dichos documentos resultan ser deficientes o inaplicables, ¿Firma Mallen® asume una fianza o responsabilidad civil solidaria para cubrir el monto de la multa o la indemnización laboral derivada de su machote automatizado?",
          a: "No. La contratación o descarga del expediente digital no constituye una póliza de fianza, seguro de caución, garantía financiera ni asunción de responsabilidad civil solidaria, subsidiaria o indemnizatoria por multas administrativas, créditos fiscales, capitales constitutivos, recargos, actualizaciones o condenas e indemnizaciones laborales derivadas de conflictos individuales o colectivos con trabajadores. Los documentos, políticas, actas y formatos entregados constituyen ingeniería documental preventiva y marcos de referencia que deben ser revisados, calibrados a la operación real del centro de trabajo, completados con los datos específicos de la organización, suscritos por quienes cuenten con facultades legales y ejecutados de manera efectiva en la operación diaria."
        },
        {
          q: "Mencionan estándares internacionales como ISO 19011 o cumplimiento SEC / FINRA. ¿Está su software certificado formalmente bajo una auditoría de tercera parte (como TÜV, BSI o un organismo acreditado en México por la EMA) para emitir dictámenes de cumplimiento en materia de seguridad social y normatividad laboral, o es una validación estrictamente interna de su plataforma web?",
          a: "La plataforma no cuenta con una certificación de tercera parte emitida por organismos de evaluación de la conformidad o acreditación (como EMA, TÜV o BSI) que certifique al software como entidad dictaminadora oficial en materia laboral, fiscal o de seguridad social, ni opera como entidad regulada o supervisada bajo marcos de mercados de valores extranjeros como la SEC o FINRA. Las referencias a ISO 19011 (directrices para auditorías), IIA (estándares de auditoría interna) o principios de integridad de mensajes de datos y trazabilidad criptográfica representan criterios metodológicos y de diseño técnico incorporados en la estructura de evaluación y en la huella de integridad del expediente."
        },
        {
          q: "¿El sistema sustituye la asesoría personalizada de un abogado litigante o de un contador público autorizado ante una controversia o auditoría abierta?",
          a: "No. El diagnóstico y el expediente documental preventivo son herramientas de análisis de riesgo y preparación documental, pero no sustituyen la representación jurídica especializada, la defensa en procedimientos administrativos o litigiosos ni el dictamen o criterio contable-fiscal profesional ante una auditoría ya iniciada o una controversia concreta."
        }
      ]
    },
    {
      num: "APARTADO 03",
      titulo: "Materialidad Fiscal (SAT Arts. 5-A y 69-B) y Fecha Cierta",
      preguntas: [
        {
          q: "El SAT exige materialidad y razón de negocios real (Art. 5-A CFF). Si su motor calcula mi exposición y genera contratos basándose en una Mediana Estadística Censal del INEGI cuando todavía no calibro mi plantilla exacta, ¿no me está vendiendo un documento genérico basado en estimaciones estadísticas que justamente carece de la materialidad específica que el SAT castiga?",
          a: "El Artículo 5-A del Código Fiscal de la Federación (CFF) exige que los actos jurídicos con efectos fiscales tengan una razón de negocios real y que la materialidad y sustancia económica de las operaciones se acrediten con elementos objetivos y comprobables (contratos, entregables, trazabilidad, ejecución efectiva y correspondencia contable-financiera), no con simples estimaciones estadísticas. Cuando el diagnóstico preliminar utiliza un estrato censal del INEGI DENUE o una mediana estadística antes de que el usuario capture su plantilla exacta, ese cálculo cumple una función estimativa y orientativa de exposición para dimensionar el riesgo inicial. Para que los instrumentos sirvan como base documental, es indispensable calibrar el número exacto de colaboradores y los datos operativos reales de la empresa y resguardar la evidencia de ejecución efectiva."
        },
        {
          q: "¿Cómo se articula la razón de negocios del Artículo 5-A del CFF con la fecha cierta exigida en la jurisprudencia sobre el Artículo 69-B del CFF?",
          a: "La razón de negocios se enfoca en el beneficio económico razonablemente esperado y en la sustancia real de la operación más allá del beneficio fiscal, mientras que la fecha cierta busca acreditar de manera fehaciente cuándo se celebró o perfeccionó un documento privado frente a terceros y autoridades fiscales. En la práctica contable y jurídica, ambos elementos son complementarios: un contrato debe responder a una necesidad operativa real del giro y contar con soporte documental, trazabilidad y constancia de celebración y ejecución."
        },
        {
          q: "¿Qué elementos probatorios convienen conservar para defender la materialidad de servicios intangibles, consultoría, software o marketing?",
          a: "En operaciones de servicios intangibles conviene integrar un expediente probatorio coherente que incluya contrato detallado con alcance y entregables definidos, constancias de fecha cierta o trazabilidad de celebración, cotizaciones o propuestas, entregables técnicos o informes verificables, bitácoras o comunicaciones de avance, comprobantes de ejecución y correspondencia clara entre el comprobante fiscal, el pago y el registro contable."
        }
      ]
    },
    {
      num: "APARTADO 04",
      titulo: "STPS (39 NOMs), Seguridad Social (IMSS / REPSE) y Protección Civil",
      preguntas: [
        {
          q: "¿En qué casos la subcontratación de servicios especializados genera riesgos en materia laboral, de seguridad social o de deducibilidad fiscal?",
          a: "Las contingencias surgen cuando se simulan esquemas de subcontratación de personal, se contratan servicios que no son especializados respecto de la actividad de la beneficiaria, se omiten registros, avisos o reportes informativos exigidos por la ley o no se verifica y conserva la documentación de cumplimiento de la empresa contratista. En materia fiscal y de seguridad social, estas deficiencias pueden derivar en el rechazo de deducciones o acreditamientos y determinación de diferencias."
        },
        {
          q: "¿Cómo impacta la clasificación de actividad y la siniestralidad en la prima del Seguro de Riesgos de Trabajo ante el IMSS?",
          a: "La prima del Seguro de Riesgos de Trabajo se relaciona con la clase asignada a la actividad declarada del centro de trabajo y con la siniestralidad de los casos de riesgos de trabajo registrados en el periodo correspondiente. Una clasificación inadecuada frente a la actividad real o la falta de prevención de accidentes y enfermedades de trabajo puede generar ajustes, diferencias o incrementos en las cuotas patronales."
        },
        {
          q: "¿Qué alcances y límites tiene la aplicación de cuestionarios o políticas de prevención de factores de riesgo psicosocial en el centro de trabajo?",
          a: "La elaboración y aplicación de políticas, medidas de prevención y cuestionarios es un componente esencial del cumplimiento en materia de factores de riesgo psicosocial y entorno organizacional, pero su valor preventivo y probatorio depende de que se adapten al tamaño y características reales del centro de trabajo, se difundan adecuadamente, se canalicen los hallazgos o quejas y se ejecuten acciones de seguimiento y control en la práctica."
        },
        {
          q: "¿Cuándo resulta exigible un Programa Interno de Protección Civil y qué ocurre si una empresa opera en más de una entidad federativa?",
          a: "La exigibilidad, modalidad y requisitos del Programa Interno de Protección Civil dependen de la normativa de protección civil de cada entidad federativa y municipio, así como de factores como la superficie, aforo, giro y nivel de riesgo del inmueble. Cuando una organización cuenta con sucursales o centros de trabajo en distintos estados o municipios, debe revisar y cumplir las disposiciones locales específicas de cada ubicación además de las normas federales aplicables."
        }
      ]
    },
    {
      num: "APARTADO 05",
      titulo: "Privacidad (LFPDPPP), Datos Biométricos y Comercio Electrónico",
      preguntas: [
        {
          q: "¿Qué diferencia existe entre contar con un Aviso de Privacidad publicado en el sitio web y cumplir con el deber de seguridad y tratamiento de datos sensibles o biométricos?",
          a: "Publicar un aviso de privacidad cumple con el deber de informar a los titulares sobre las finalidades y condiciones del tratamiento, pero el cumplimiento integral exige además contar con base de legitimación o consentimiento válido —especialmente consentimiento expreso y por escrito cuando se tratan datos sensibles, de salud o biométricos—, implementar medidas de seguridad administrativas, técnicas y físicas acordes al riesgo, regular a los encargados y atender los derechos de acceso, rectificación, cancelación y oposición."
        },
        {
          q: "¿Qué implicaciones legales y fiscales tiene el uso de herramientas digitales, comercio electrónico, facturación y canales de contacto con clientes?",
          a: "Las empresas que comercializan productos o servicios por medios digitales deben asegurar claridad en la información previa a la contratación, precios, condiciones de entrega, cambios o devoluciones y garantías, contar con políticas de privacidad y seguridad acordes a los datos recabados, mantener canales de atención accesibles y cuidar la correcta emisión, validación y conservación de comprobantes fiscales y registros electrónicos."
        },
        {
          q: "¿Cómo debe documentarse la relación con encargados de tratamiento de datos (despachos contables, nómina, nube, soporte TI)?",
          a: "Cuando un proveedor externo o despacho trata datos personales por cuenta del responsable, es necesario formalizar por escrito los términos del encargo estableciendo que el tratamiento se realizará únicamente conforme a las instrucciones del responsable, prohibiendo finalidades distintas, exigiendo medidas de seguridad adecuadas, regulando sub-encargos y definiendo el tratamiento o devolución de la información al concluir el servicio."
        }
      ]
    },
    {
      num: "APARTADO 06",
      titulo: "Operación del Expediente, Seguridad, Reformas y Revisión Profesional",
      preguntas: [
        {
          q: "¿Qué criterios prácticos debe seguir un contador o asesor jurídico al revisar un dictamen preventivo automatizado antes de adoptarlo en una empresa?",
          a: "El profesional debe verificar la exactitud de los datos de identidad, régimen, giro, plantilla y centros de trabajo, contrastar los supuestos normativos con la operación real del cliente, completar los campos específicos de representantes, comisiones, contrapartes o fechas, y supervisar que los documentos seleccionados se firmen, difundan y apliquen de forma congruente con la estrategia legal y fiscal de la empresa."
        },
        {
          q: "¿Cómo se manejan los cambios normativos, reformas legales o actualizaciones de la UMA y de los padrones públicos después de emitir un expediente?",
          a: "La normatividad laboral, fiscal, de seguridad social y de protección de datos, así como el valor de la UMA y los padrones públicos, pueden actualizarse periódicamente. Cada expediente refleja el análisis y los parámetros vigentes al momento de su emisión y calibración, por lo que ante cambios legislativos o de plantilla conviene revisar y actualizar los instrumentos correspondientes."
        },
        {
          q: "¿Cómo se protege la confidencialidad de la información ingresada durante la calibración y generación del expediente?",
          a: "La plataforma aplica controles de seguridad de la información, cifrado en tránsito y resguardo de registros orientados a proteger la integridad y confidencialidad de los datos procesados, limitando su uso a la generación, verificación y trazabilidad del expediente solicitado conforme a las políticas de privacidad aplicables."
        },
        {
          q: "¿Por qué se entregan formatos editables en el expediente y qué precaución debe tomarse al modificarlos?",
          a: "Los formatos editables permiten a la empresa y a su área legal o contable adaptar cláusulas, anexos, firmantes o detalles operativos específicos de cada contrato o política interna. Al editar cualquier documento, debe cuidarse de no alterar referencias normativas esenciales, alcances de confidencialidad o congruencia con los demás instrumentos del expediente, verificando siempre su adecuación legal final antes de firmarlo."
        }
      ]
    }
  ];

  // 1. Renderizar los 6 Apartados completos dentro de #faq sin dejar preguntas flotando
  function mountStructuredFaqSection() {
    const faqRoot =
      document.getElementById("faq") ||
      Array.from(document.querySelectorAll("section, div")).find(function (el) {
        return (el.textContent || "").indexOf("Preguntas Frecuentes") !== -1;
      });
    if (!faqRoot) return;

    if (document.getElementById("mallen-faq-6-apartados-v98")) return;

    // Retirar cualquier <details> suelto que haya quedado flotando dentro de #faq
    Array.from(faqRoot.querySelectorAll("details")).forEach(function (d) {
      d.remove();
    });

    const wrapper = document.createElement("div");
    wrapper.id = "mallen-faq-6-apartados-v98";
    wrapper.className = "space-y-4 mt-6";

    FAQ_APARTADOS.forEach(function (ap) {
      const catDetails = document.createElement("details");
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
      wrapper.appendChild(catDetails);
    });

    faqRoot.appendChild(wrapper);
  }

  // 2. Botón Único de Modo Día / Oscuro en Footer (sin duplicados)
  window.toggleMallenTheme = function () {
    const root = document.documentElement;
    const body = document.body;
    const isLight = !root.classList.contains("light-mode");
    root.classList.toggle("light-mode", isLight);
    root.classList.toggle("w16-light-mode", isLight);
    if (body) {
      body.classList.toggle("light-mode", isLight);
      body.classList.toggle("w16-light-mode", isLight);
    }
    try {
      localStorage.setItem("mallen_theme_mode", isLight ? "light" : "dark");
      localStorage.setItem("w16_theme_mode", isLight ? "light" : "dark");
    } catch (_) {}
    updateBottomThemeButtonLabel();
  };
  window.toggleW16PureLightMode = window.toggleMallenTheme;

  function updateBottomThemeButtonLabel() {
    const isLight = document.documentElement.classList.contains("light-mode");
    const label = isLight ? "Cambiar a Modo Oscuro" : "Cambiar a Modo Día";
    const btn = document.getElementById("mallen-bottom-theme-btn");
    if (btn) btn.textContent = label;
  }

  // 3. Enlaces directos de descarga de App por Sistema Operativo
  window.openMallenOsInstall = function (osName) {
    if ((osName === "android" || osName === "desktop") && typeof window.openMallenAppInstaller === "function") {
      window.openMallenAppInstaller();
    }
    const modal = document.getElementById("mallen-pwa-install-modal");
    const backdrop = document.getElementById("mallen-pwa-modal-backdrop");
    if (!modal || !backdrop) return;

    const titleMap = {
      android: "Descarga Directa — Android (Chrome / Edge / Samsung)",
      ios: "Descarga Directa — iPhone / iPad (iOS Safari)",
      desktop: "Descarga Directa — Windows / macOS (Escritorio)"
    };
    const stepsMap = {
      android: "1. Si aparece el aviso nativo del navegador, pulse <strong>Instalar</strong>.<br>2. Si no aparece automáticamente, toque el menú <strong>⋮</strong> arriba a la derecha y seleccione <strong>Instalar aplicación</strong> o <strong>Agregar a la pantalla principal</strong>.",
      ios: "1. Abra <strong>mallen.lat</strong> en <strong>Safari</strong>.<br>2. Toque el botón <strong>Compartir</strong> (cuadro con flecha hacia arriba) y seleccione <strong>Agregar a pantalla de inicio</strong>.<br>3. Pulse <strong>Agregar</strong> para instalar el ícono nativo de FIRMA MALLEN®.",
      desktop: "1. En Chrome o Edge, haga clic en el ícono de <strong>Instalar</strong> situado a la derecha de la barra de direcciones.<br>2. Confirme <strong>Instalar FIRMA MALLEN®</strong> para abrirla como aplicación independiente de escritorio."
    };

    modal.innerHTML =
      '<div class="flex items-center justify-between pb-3 mb-4 border-b border-slate-800">' +
        '<div>' +
          '<span class="text-xs font-mono-data uppercase text-emerald-400 block">APP OFICIAL FIRMA MALLEN®</span>' +
          '<h3 class="text-lg font-extrabold text-white">' + (titleMap[osName] || titleMap.android) + '</h3>' +
        '</div>' +
        '<button type="button" onclick="window.closeMallenAppInstaller()" class="mallen-btn-secondary px-3 py-1 text-xs">Cerrar ✕</button>' +
      '</div>' +
      '<div class="p-4 rounded-xl border border-slate-800 bg-slate-950/70 text-sm text-slate-200 leading-relaxed">' +
        (stepsMap[osName] || stepsMap.android) +
      '</div>';

    modal.classList.add("open");
    backdrop.classList.add("open");
  };

  function setupFooterControlsOnly() {
    // Ocultar botones de tema en cabecera superior
    document.querySelectorAll("header [data-mallen-theme-btn], #eu-theme-toggle-btn").forEach(function (el) {
      el.style.display = "none";
    });

    // Sincronizar estado guardado
    try {
      const saved = localStorage.getItem("mallen_theme_mode") || localStorage.getItem("w16_theme_mode");
      if (saved === "light") {
        document.documentElement.classList.add("light-mode", "w16-light-mode");
        if (document.body) document.body.classList.add("light-mode", "w16-light-mode");
      }
    } catch (_) {}

    const footer = document.querySelector("footer");
    if (!footer) return;

    // Ocultar cualquier botón duplicado anterior de tema dentro del footer (dejando solo #mallen-bottom-theme-btn)
    footer.querySelectorAll("button").forEach(function (btn) {
      if (btn.id === "mallen-bottom-theme-btn") return;
      const txt = (btn.textContent || "").toLowerCase();
      if (txt.indexOf("modo") !== -1 || txt.indexOf("claro") !== -1 || txt.indexOf("oscuro") !== -1 || txt.indexOf("día") !== -1 || txt.indexOf("dia") !== -1) {
        btn.style.display = "none";
      }
    });

    if (!document.getElementById("mallen-footer-utility-bar")) {
      const bar = document.createElement("div");
      bar.id = "mallen-footer-utility-bar";
      bar.className = "max-w-6xl mx-auto mt-6 pt-5 border-t border-slate-800/80 flex flex-col sm:flex-row items-center justify-between gap-4";
      bar.innerHTML =
        '<div class="flex flex-wrap items-center gap-2">' +
          '<span class="text-xs font-semibold uppercase tracking-wider text-slate-400 mr-1">App FIRMA MALLEN®:</span>' +
          '<button type="button" onclick="window.openMallenOsInstall(\'android\')" class="mallen-btn-secondary px-3 py-1.5 text-xs font-semibold border border-slate-700 hover:border-emerald-400">Android</button>' +
          '<button type="button" onclick="window.openMallenOsInstall(\'ios\')" class="mallen-btn-secondary px-3 py-1.5 text-xs font-semibold border border-slate-700 hover:border-emerald-400">iOS (iPhone / iPad)</button>' +
          '<button type="button" onclick="window.openMallenOsInstall(\'desktop\')" class="mallen-btn-secondary px-3 py-1.5 text-xs font-semibold border border-slate-700 hover:border-emerald-400">Windows / macOS</button>' +
        '</div>' +
        '<div>' +
          '<button type="button" id="mallen-bottom-theme-btn" data-mallen-theme-btn="1" onclick="window.toggleMallenTheme()" class="mallen-btn-secondary px-4 py-1.5 text-xs font-bold border border-slate-600 hover:border-white">Cambiar a Modo Día</button>' +
        '</div>';
      footer.appendChild(bar);
    }

    updateBottomThemeButtonLabel();
  }

  function initV98() {
    mountStructuredFaqSection();
    setupFooterControlsOnly();
  }

  document.addEventListener("DOMContentLoaded", function () {
    initV98();
    setTimeout(initV98, 350);
    setTimeout(setupFooterControlsOnly, 900);
  });
})();
