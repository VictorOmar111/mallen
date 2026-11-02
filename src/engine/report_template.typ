#set page(
  paper: "us-letter",
  margin: (top: 1.05cm, bottom: 1.15cm, left: 1.2cm, right: 1.2cm),
  fill: rgb("#ffffff"),
  header: context {
    if counter(page).get().first() > 1 [
      #grid(
        columns: (1fr, auto),
        align(left)[#text(size: 6.3pt, weight: "bold", fill: rgb("#0f172a"))[FIRMA PERICIAL MALLEN • EXPEDIENTE CORPORATIVO DE CUMPLIMIENTO INTEGRAL • {{EMPRESA_TAX_ID}}]],
        align(right)[#text(size: 6.3pt, fill: rgb("#475569"))[SELLO CRIPTOGRÁFICO SHA-256: {{AUDIT_HASH}}]]
      )
      #v(-0.15cm)
      #line(length: 100%, stroke: 0.4pt + rgb("#cbd5e1"))
    ]
  },
  footer: context [
    #line(length: 100%, stroke: 0.4pt + rgb("#cbd5e1"))
    #v(0.08cm)
    #grid(
      columns: (1fr, auto),
      align(left)[#text(size: 5.9pt, fill: rgb("#64748b"))[Dictamen Preventivo de Cumplimiento Técnico-Documental (Arts. 89–93 Código de Comercio). Verificación pública: https://mallen.lat/verify/{{EMPRESA_TAX_ID}}]],
      align(right)[#text(size: 6.3pt, weight: "bold", fill: rgb("#0f172a"))[Página #counter(page).display("1 de 1", both: true)]]
    )
  ]
)

#set text(font: ("DejaVu Sans", "Liberation Sans", "Arial"), size: 7.4pt, fill: rgb("#1e293b"), lang: "es")
#set par(justify: true, leading: 0.43em)

// Macro gráfico de Efecto Difuminado (Frosted Glass Blur) para el Pre-Dictamen sin pago
#let frosted-blur(tag_label) = box(
  width: 100%,
  fill: rgb("#f8fafc"),
  stroke: 0.45pt + rgb("#cbd5e1"),
  radius: 2.5pt,
  inset: (x: 5pt, y: 3pt)
)[
  #grid(
    columns: (1fr, auto),
    gutter: 6pt,
    align: horizon,
    [
      #box(width: 92%, height: 3.2pt, fill: rgb("#cbd5e1"), radius: 1.5pt)
      #v(1.8pt)
      #box(width: 68%, height: 3.2pt, fill: rgb("#e2e8f0"), radius: 1.5pt)
    ],
    [
      #box(fill: rgb("#0f172a"), radius: 2pt, inset: (x: 4.5pt, y: 2pt))[
        #text(size: 4.8pt, weight: "bold", fill: rgb("#38bdf8"))[#tag_label]
      ]
    ]
  )
]

// ==================== PÁGINA 1: DICTAMEN EJECUTIVO, COMPULSA Y GRÁFICAS ====================
#box(
  width: 100%,
  fill: rgb("#0f172a"),
  radius: 5pt,
  inset: (x: 12pt, y: 8pt),
  stroke: 1pt + rgb("#10b981")
)[
  #grid(
    columns: (1fr, auto),
    gutter: 10pt,
    [
      #text(size: 6.4pt, weight: "bold", fill: rgb("#34d399"))[{{PARTNER_HEADER}}]
      #linebreak()
      #text(size: 10.8pt, weight: "bold", fill: rgb("#ffffff"))[DICTAMEN PERICIAL DETERMINÍSTICO DE EXPOSICIÓN NORMATIVA 360°]
      #linebreak()
      #text(size: 6.5pt, fill: rgb("#cbd5e1"))[Auditoría Multivariable INEGI DENUE • SAT (Arts. 69 y 69-B) • STPS (39 NOMs + REPSE) • IMSS • LFPDPPP • 9 Sectoriales • PROFECO]
    ],
    align(right + horizon)[
      #box(fill: rgb("#1e293b"), stroke: 0.6pt + rgb("#34d399"), radius: 4pt, inset: 5.5pt)[
        #text(size: 5.9pt, fill: rgb("#94a3b8"))[FOLIO CRIPTOGRÁFICO SHA-256]
        #linebreak()
        #text(size: 7.1pt, weight: "bold", fill: rgb("#34d399"))[{{AUDIT_HASH}}]
        #linebreak()
        #text(size: 5.8pt, fill: rgb("#cbd5e1"))[CORTE UTC: {{FECHA_EMISION}}]
      ]
    ]
  )
]

#v(0.16em)

// FICHA TÉCNICO DEL ENTE AUDITADO
#table(
  columns: (17%, 33%, 17%, 33%),
  inset: (x: 5.5pt, y: 3.6pt),
  stroke: 0.5pt + rgb("#cbd5e1"),
  fill: (x, y) => if calc.odd(x) { rgb("#ffffff") } else { rgb("#f8fafc") },
  [#text(weight: "bold", size: 6.5pt, fill: rgb("#334155"))[ENTE AUDITADO:]],
  [#text(weight: "bold", size: 7pt, fill: rgb("#0f172a"))[{{EMPRESA_NOMBRE}}]],
  [#text(weight: "bold", size: 6.5pt, fill: rgb("#334155"))[RFC / ID BÚSQUEDA:]],
  [#text(weight: "bold", size: 7pt, fill: rgb("#0f172a"))[{{EMPRESA_TAX_ID}} (Índice Riesgo: {{RISK_SCORE}}/100)]],

  [#text(weight: "bold", size: 6.5pt, fill: rgb("#334155"))[GIRO SCIAN OFICIAL:]],
  [#text(size: 6.7pt, fill: rgb("#0f172a"))[{{EMPRESA_ACTIVIDAD}}]],
  [#text(weight: "bold", size: 6.5pt, fill: rgb("#334155"))[REPRESENTACIÓN:]],
  [#text(size: 6.7pt, fill: rgb("#0f172a"))[{{REPRESENTANTE_LEGAL}}]],

  [#text(weight: "bold", size: 6.5pt, fill: rgb("#334155"))[DOMICILIO / DENUE:]],
  [#text(size: 6.6pt, fill: rgb("#0f172a"))[{{EMPRESA_DIRECCION}}]],
  [#text(weight: "bold", size: 6.5pt, fill: rgb("#334155"))[ESTATUS DICTAMEN:]],
  [#text(weight: "bold", size: 6.7pt, fill: rgb("{{STATUS_HEX}}"))[{{ESTATUS_TEXTO}}]]
)

#v(0.14em)

#text(size: 7.8pt, weight: "bold", fill: rgb("#0f172a"))[1. METODOLOGÍA DE PARAMETRIZACIÓN POR GIRO SCIAN, CAPACIDAD ECONÓMICA Y PRIMA IMSS]
#v(0.05em)
Dictamen pericial compilado específicamente para *"{{EMPRESA_NOMBRE}}"* (`{{EMPRESA_TAX_ID}}`), evaluando su actividad económica registrada (*{{EMPRESA_ACTIVIDAD}}*) en la jurisdicción de *{{EMPRESA_JURISDICCION}}*. Los cálculos sancionatorios en Unidades de Medida y Actualización (UMA 2026: \$113.14 MXN) ponderan la capacidad económica y plantilla conforme al Art. 992 de la LFT y Art. 75 del CFF, manteniendo sincronización matemática 1:1 entre esta Matriz Consolidada, el portal de verificación web y el Anexo Técnico de los 37 Vectores (Página 2).

#v(0.14em)

#text(size: 7.8pt, weight: "bold", fill: rgb("#0f172a"))[2. COMPULSA PERICIAL EN BASES DE DATOS OFICIALES, LISTAS NEGRAS Y TELEMETRÍA WEB]
#v(0.05em)
#table(
  columns: (21%, 29%, 21%, 29%),
  inset: (x: 5.5pt, y: 3.4pt),
  stroke: 0.5pt + rgb("#cbd5e1"),
  fill: (x, y) => if calc.even(x) { rgb("#f1f5f9") } else { rgb("#ffffff") },
  [#text(weight: "bold", size: 6.3pt)[SAT ART. 69-B (EFOS/EDOS):]],
  [#text(weight: "bold", size: 6.3pt, fill: rgb("#0f172a"))[{{SAT_69B_STATUS}}]],
  [#text(weight: "bold", size: 6.3pt)[SAT ART. 69 (CRÉDITOS/CSD):]],
  [#text(weight: "bold", size: 6.3pt, fill: rgb("#0f172a"))[{{SAT_69_STATUS}}]],

  [#text(weight: "bold", size: 6.3pt)[PADRÓN STPS REPSE:]],
  [#text(weight: "bold", size: 6.3pt, fill: rgb("#0f172a"))[{{REPSE_STATUS}}]],
  [#text(weight: "bold", size: 6.3pt)[SFP INHABILITADOS:]],
  [#text(weight: "bold", size: 6.3pt, fill: rgb("#0f172a"))[{{SFP_STATUS}}]],

  [#text(weight: "bold", size: 6.3pt)[CIFRADO TLS/SSL (ART. 19):]],
  [#text(weight: "bold", size: 6.3pt, fill: rgb("#0f172a"))[{{SSL_STATUS}}]],
  [#text(weight: "bold", size: 6.3pt)[AVISO PRIVACIDAD LFPDPPP:]],
  [#text(weight: "bold", size: 6.3pt, fill: rgb("#0f172a"))[{{PRIVACY_STATUS}}]]
)

#v(0.14em)

#text(size: 7.8pt, weight: "bold", fill: rgb("#0f172a"))[3. MATRIZ CONSOLIDADA DE CONTINGENCIA PATRIMONIAL POR ÓRGANO REGULADOR (SINCRONIZADA 1:1)]
#v(0.05em)
#table(
  columns: (25%, 19%, 56%),
  inset: (x: 5.5pt, y: 3.6pt),
  stroke: 0.5pt + rgb("#cbd5e1"),
  fill: (x, y) => if y == 0 { rgb("#0f172a") } else if y == 7 { rgb("#fef2f2") } else if calc.odd(y) { rgb("#f8fafc") } else { rgb("#ffffff") },
  align: (left + horizon, right + horizon, left + horizon),
  [#text(fill: white, weight: "bold", size: 6.5pt)[ÓRGANO REGULADOR / VECTORES]],
  [#align(left)[#text(fill: white, weight: "bold", size: 6.5pt)[SUBTOTAL EXPOSICIÓN]]],
  [#text(fill: white, weight: "bold", size: 6.5pt)[DICTAMINACIÓN TÉCNICA ESPECÍFICA AL GIRO AUDITADO]],

  [#text(weight: "bold", size: 6.5pt)[1. STPS (Vectores 01 al 08)]],
  [#text(weight: "bold", size: 6.8pt, fill: rgb("#b91c1c"))[\$ {{MULTA_STPS}} MXN]],
  [{{HALLAZGO_TABLA_STPS_TYPST}}],

  [#text(weight: "bold", size: 6.5pt)[2. SAT / CFF (Vectores 09 al 14)]],
  [#text(weight: "bold", size: 6.8pt, fill: rgb("#b91c1c"))[\$ {{MULTA_SAT}} MXN]],
  [{{HALLAZGO_TABLA_SAT_TYPST}}],

  [#text(weight: "bold", size: 6.5pt)[3. IMSS / INFONAVIT (Vectores 15 al 20)]],
  [#text(weight: "bold", size: 6.8pt, fill: rgb("#b91c1c"))[\$ {{MULTA_IMSS}} MXN]],
  [{{HALLAZGO_TABLA_IMSS_TYPST}}],

  [#text(weight: "bold", size: 6.5pt)[4. SABG / LFPDPPP (Vectores 21 al 26)]],
  [#text(weight: "bold", size: 6.8pt, fill: rgb("#b91c1c"))[\$ {{MULTA_INAI}} MXN]],
  [{{HALLAZGO_TABLA_INAI_TYPST}}],

  [#text(weight: "bold", size: 6.5pt)[5. 9 SECTORIALES / PIPC (Vectores 27 al 32)]],
  [#text(weight: "bold", size: 6.8pt, fill: rgb("#b91c1c"))[\$ {{MULTA_PC}} MXN]],
  [{{HALLAZGO_TABLA_PC_TYPST}}],

  [#text(weight: "bold", size: 6.5pt)[6. PROFECO / ECONOMÍA (Vectores 33 al 37)]],
  [#text(weight: "bold", size: 6.8pt, fill: rgb("#b91c1c"))[\$ {{MULTA_PROFECO}} MXN]],
  [{{HALLAZGO_TABLA_PROFECO_TYPST}}],

  [#text(weight: "bold", size: 6.9pt, fill: rgb("#991b1b"))[CONTINGENCIA TOTAL AUDITADA:]],
  [#text(weight: "bold", size: 7.5pt, fill: rgb("#991b1b"))[\$ {{ESTIMATED_FINE}} MXN]],
  [#text(weight: "bold", size: 6.3pt, fill: rgb("#7f1d1d"))[Suma determinística exacta de los 37 vectores evaluados en la Página 2 (UMA 2026: \$113.14 MXN).]]
)

#v(0.14em)

// ==================== SECCIÓN 4: GRÁFICA VISUAL DE CONCENTRACIÓN PATRIMONIAL ====================
#text(size: 7.8pt, weight: "bold", fill: rgb("#0f172a"))[4. GRÁFICA PERICIAL DE DISTRIBUCIÓN DE RIESGO PATRIMONIAL POR DEPENDENCIA]
#v(0.05em)
{{VISUAL_RISK_CHART_TYPST}}

#v(0.14em)

// PIE EJECUTIVO DE PÁGINA 1
#box(width: 100%, fill: rgb("#f8fafc"), stroke: 0.6pt + rgb("#94a3b8"), radius: 4pt, inset: 5.5pt)[
  #grid(
    columns: (1fr, auto),
    [
      #text(size: 6.4pt, weight: "bold", fill: rgb("#0f172a"))[PERITO DICTAMINADOR: {{AUDITOR_NOMBRE}} • Cédula: {{AUDITOR_CEDULA}} • Reg. STPS: {{AUDITOR_DC5}}]
      #linebreak()
      #text(size: 5.9pt, fill: rgb("#475569"))[Jurisdicción: {{EMPRESA_JURISDICCION}} • Oficial Privacidad: {{OFICIAL_PRIVACIDAD}} • Coordinador NOM-019: {{COORDINADOR_CSH}} • HTTP: {{HTTP_STATUS}}]
    ],
    align(right + horizon)[
      #text(size: 6.4pt, weight: "bold", fill: rgb("#047857"))[PÁGINA 2: CÉDULA 37 VECTORES • PÁGINA 3: CRONOGRAMA Y ROI →]
    ]
  )
]

// ==================== PÁGINA 2: MATRIZ ÍNTEGRA DE LOS 37 VECTORES EN 1 SOLA PÁGINA ====================
#pagebreak()

#box(
  width: 100%,
  fill: rgb("#0f172a"),
  radius: 4pt,
  inset: (x: 10pt, y: 5.5pt),
  stroke: 0.8pt + rgb("#3b82f6")
)[
  #grid(
    columns: (1fr, auto),
    [
      #text(size: 8.6pt, weight: "bold", fill: rgb("#ffffff"))[ANEXO TÉCNICO I: CÉDULA DE EVALUACIÓN DE LOS 37 VECTORES NORMATIVOS]
      #linebreak()
      #text(size: 6.2pt, fill: rgb("#93c5fd"))[Ente Auditado: {{EMPRESA_NOMBRE}} ({{EMPRESA_TAX_ID}}) • Giro SCIAN: {{EMPRESA_ACTIVIDAD}}]
    ],
    align(right + horizon)[
      #text(size: 6.9pt, weight: "bold", fill: rgb("#fbbf24"))[CONTINGENCIA ACUMULADA: \$ {{ESTIMATED_FINE}} MXN]
    ]
  )
]

#v(0.08em)

{{VECTORS_37_TABLE_TYPST}}

// ==================== PÁGINA 3: PLAN DE ACCIÓN 30-60-90 DÍAS, GRÁFICA ROI E INVENTARIO DE ENTREGABLES ====================
#pagebreak()

#box(
  width: 100%,
  fill: rgb("#0f172a"),
  radius: 4pt,
  inset: (x: 10pt, y: 6.5pt),
  stroke: 0.8pt + rgb("#10b981")
)[
  #text(size: 8.8pt, weight: "bold", fill: rgb("#ffffff"))[ANEXO TÉCNICO II: HOJA DE RUTA DE REMEDIACIÓN 30-60-90 DÍAS, INGENIERÍA ROI E INVENTARIO DEL KIT 360°]
  #linebreak()
  #text(size: 6.3pt, fill: rgb("#6ee7b7"))[Plan de Ejecución Preventiva para {{EMPRESA_NOMBRE}} ({{EMPRESA_TAX_ID}}) • Sello Criptográfico: {{AUDIT_HASH}}]
]

#v(0.16em)

#text(size: 7.8pt, weight: "bold", fill: rgb("#0f172a"))[5. CRONOGRAMA EJECUTIVO DE REMEDIACIÓN E IMPLEMENTACIÓN EN SITIO (30 / 60 / 90 DÍAS)]
#v(0.06em)
#table(
  columns: (18%, 26%, 38%, 18%),
  inset: (x: 4pt, y: 2.8pt),
  stroke: 0.5pt + rgb("#cbd5e1"),
  fill: (x, y) => if y == 0 { rgb("#0f172a") } else if calc.odd(y) { rgb("#f8fafc") } else { rgb("#ffffff") },
  [#text(fill: white, weight: "bold", size: 6.5pt)[FASE / HORIZONTE]],
  [#text(fill: white, weight: "bold", size: 6.5pt)[VECTORES CRÍTICOS A BLINDAR]],
  [#text(fill: white, weight: "bold", size: 6.5pt)[ACCIÓN TÉCNICA CON LOS INSTRUMENTOS DEL KIT MALLEN]],
  [#text(fill: white, weight: "bold", size: 6.5pt)[RIESGO MITIGADO]],

  [
    #text(weight: "bold", size: 6.5pt, fill: rgb("#b91c1c"))[FASE 1: INMEDIATA]
    #linebreak()
    #text(size: 6pt, fill: rgb("#475569"))[(Días 1 a 15)]
  ],
  [
    #text(weight: "bold", size: 6.3pt)[LFPDPPP, PROFECO y SAT Art. 69-B]
    #linebreak()
    #text(size: 6pt, fill: rgb("#475569"))[(Vectores 09, 10, 21, 22, 23, 33 y 35)]
  ],
  [
    {{ROADMAP_PHASE_1}}
  ],
  [#text(weight: "bold", size: 6.3pt, fill: rgb("#047857"))[Evita multas directas de oscuridad web y rechazo fiscal de deducciones]],

  [
    #text(weight: "bold", size: 6.5pt, fill: rgb("#b45309"))[FASE 2: LABORAL]
    #linebreak()
    #text(size: 6pt, fill: rgb("#475569"))[(Días 16 a 45)]
  ],
  [
    #text(weight: "bold", size: 6.3pt)[STPS e IMSS Patronal]
    #linebreak()
    #text(size: 6pt, fill: rgb("#475569"))[(Vectores 01, 02, 04, 15, 16 y 17)]
  ],
  [
    {{ROADMAP_PHASE_2}}
  ],
  [#text(weight: "bold", size: 6.3pt, fill: rgb("#047857"))[Desactiva multas por empleado en inspecciones extraordinarias STPS]],

  [
    #text(weight: "bold", size: 6.5pt, fill: rgb("#1d4ed8"))[FASE 3: FÍSICA Y SECTORIAL]
    #linebreak()
    #text(size: 6pt, fill: rgb("#475569"))[(Días 46 a 90)]
  ],
  [
    #text(weight: "bold", size: 6.3pt)[Seguridad Física, PIPC y Giro SCIAN]
    #linebreak()
    #text(size: 6pt, fill: rgb("#475569"))[(Vectores 03, 05, 06, 07, 13 y 29 al 31)]
  ],
  [
    {{ROADMAP_PHASE_3}}
  ],
  [#text(weight: "bold", size: 6.3pt, fill: rgb("#047857"))[Previene clausura total o parcial del establecimiento mercantil]]
)

#v(0.16em)

#text(size: 7.8pt, weight: "bold", fill: rgb("#0f172a"))[6. CALENDARIO ANUAL DE OBLIGACIONES PERIÓDICAS DE CUMPLIMIENTO (AGENDA DEL CONTADOR Y APODERADO)]
#v(0.06em)
{{CALENDAR_SECTION_TYPST}}

#v(0.18em)

#grid(
  columns: (48%, 52%),
  gutter: 8pt,
  [
    #text(size: 7.6pt, weight: "bold", fill: rgb("#0f172a"))[7. ANÁLISIS FINANCIERO Y RETORNO DE INVERSIÓN (ROI)]
    #v(0.05em)
    #table(
      columns: (58%, 20%, 22%),
      inset: 3.8pt,
      stroke: 0.4pt + rgb("#cbd5e1"),
      fill: (x, y) => if y == 0 { rgb("#0f172a") } else if y == 5 { rgb("#ecfdf5") } else { rgb("#ffffff") },
      [#text(fill: white, weight: "bold", size: 6.1pt)[CONCEPTO EN DESPACHOS TRADICIONALES]],
      [#text(fill: white, weight: "bold", size: 6.1pt)[MERCADO]],
      [#text(fill: white, weight: "bold", size: 6.1pt)[KIT MALLEN]],
      [#text(size: 6.1pt)[Pólizas STPS (NOM-035, 037, 019, 030)]], [#text(size: 6.1pt)[\$25,000]], [#text(weight: "bold", size: 6pt, fill: rgb("#047857"))[INCLUIDO]],
      [#text(size: 6.1pt)[Contrato Marco 69-B Fecha Cierta + Bitácora]], [#text(size: 6.1pt)[\$35,000]], [#text(weight: "bold", size: 6pt, fill: rgb("#047857"))[INCLUIDO]],
      [#text(size: 6.1pt)[Aviso Integral LFPDPPP + Biométricos + ARCO]], [#text(size: 6.1pt)[\$22,000]], [#text(weight: "bold", size: 6pt, fill: rgb("#047857"))[INCLUIDO]],
      [#text(size: 6.1pt)[Guía PIPC + Protocolo IMSS REPSE + PROFECO]], [#text(size: 6.1pt)[\$45,000]], [#text(weight: "bold", size: 6pt, fill: rgb("#047857"))[INCLUIDO]],
      [#text(weight: "bold", size: 6.3pt, fill: rgb("#065f46"))[Ahorro Directo en Consultoría (mayor al 95%):]],
      [#text(weight: "bold", size: 6.2pt, fill: rgb("#64748b"))[\$127,000]],
      [#text(weight: "bold", size: 6.5pt, fill: rgb("#047857"))[\${{ACTIVATION_PRICE}}]]
    )
  ],
  [
    #text(size: 7.6pt, weight: "bold", fill: rgb("#0f172a"))[8. INVENTARIO DE LOS 17 INSTRUMENTOS DEL KIT (.ZIP)]
    #v(0.05em)
    #box(width: 100%, fill: rgb("#f8fafc"), stroke: 0.5pt + rgb("#cbd5e1"), radius: 4pt, inset: 5.5pt)[
      #text(size: 6.1pt, fill: rgb("#1e293b"))[
        *A. 10 Certificados de Acreditación Normativa (PDF Sellados SHA-256):*
        #linebreak()
        NOM-035 Psicosocial • NOM-037 Teletrabajo • NOM-019/030 Comisión SST • SAT Art. 69-B Materialidad • LFPDPPP Privacidad • IMSS 32-D/REPSE • Protección Civil PIPC • PROFECO Art. 76 BIS • LFPIORPI PLD • Blindaje Maestro 360°.
        #v(0.18em)
        *B. 7 Instrumentos Jurídicos Editables (Word-HTML y PDF Listos para Firma):*
        #linebreak()
        1. Aviso de Privacidad Integral, Simplificado y Formato Biométrico LFPDPPP.
        #linebreak()
        2. Política de Prevención Psicosocial y Cuestionarios NOM-035-STPS.
        #linebreak()
        3. Acta Constitutiva de Comisión Mixta NOM-019 y Diagnóstico NOM-030.
        #linebreak()
        4. Adendum de Teletrabajo y Lista Ergonómica NOM-037-STPS-2023.
        #linebreak()
        5. Contrato Marco B2B con Cláusula de Fecha Cierta SCJN y Bitácora Art. 69-B.
        #linebreak()
        6. Protocolo de Blindaje Patronal IMSS SUA y Checklist Mensual REPSE.
        #linebreak()
        7. Términos y Condiciones E-Commerce Art. 76 BIS LFPC y Póliza de Garantía.
      ]
    ]
  ]
)

#v(0.16em)

// BLOQUE DE FIRMA PERICIAL Y SELLO DE INTEGRIDAD EN PÁGINA 3
#box(width: 100%, fill: rgb("#0f172a"), stroke: 0.8pt + rgb("#10b981"), radius: 4pt, inset: 7pt)[
  #grid(
    columns: (1fr, auto),
    gutter: 10pt,
    [
      #text(size: 6.7pt, weight: "bold", fill: rgb("#34d399"))[CERTIFICACIÓN CRIPTOGRÁFICA DE INTEGRIDAD Y ALCANCE METODOLÓGICO (ARTS. 89–93 CÓDIGO DE COMERCIO)]
      #linebreak()
      #text(size: 5.9pt, fill: rgb("#e2e8f0"))[
        El presente Dictamen Pericial y sus Anexos I y II han sido compilados por *{{AUDITOR_NOMBRE}}* (Cédula Profesional: `{{AUDITOR_CEDULA}}` | Registro STPS: `{{AUDITOR_DC5}}`). La huella criptográfica `{{AUDIT_HASH}}` garantiza la inalterabilidad del expediente. Los vectores clasificados como *"EXENTO SCIAN"* o *"BLINDADO"* reflejan una contingencia de \$0.00 MXN para el giro evaluado.
      ]
    ],
    align(right + horizon)[
      #box(fill: rgb("#065f46"), stroke: 0.6pt + rgb("#34d399"), radius: 3pt, inset: 5pt)[
        #text(size: 5.8pt, fill: rgb("#a7f3d0"))[EXPEDIENTE COMPLETO + CFDI 4.0]
        #linebreak()
        #text(size: 7.4pt, weight: "bold", fill: rgb("#ffffff"))[\$ {{ACTIVATION_PRICE}} MXN]
      ]
    ]
  )
]
