#set page(
  paper: "us-letter",
  margin: (top: 1.15cm, bottom: 1.25cm, left: 1.25cm, right: 1.25cm),
  fill: rgb("#ffffff"),
  header: context {
    if counter(page).get().first() > 1 [
      #grid(
        columns: (1fr, auto),
        align(left)[#text(size: 6.5pt, weight: "bold", fill: rgb("#475569"))[FIRMA PERICIAL MALLEN • EXPEDIENTE FORENSE DE CUMPLIMIENTO 360° • {{EMPRESA_TAX_ID}}]],
        align(right)[#text(size: 6.5pt, fill: rgb("#64748b"))[HUELLA CRIPTOGRÁFICA SHA-256: {{AUDIT_HASH}}]]
      )
      #v(-0.15cm)
      #line(length: 100%, stroke: 0.4pt + rgb("#cbd5e1"))
    ]
  },
  footer: context [
    #line(length: 100%, stroke: 0.4pt + rgb("#cbd5e1"))
    #v(0.1cm)
    #grid(
      columns: (1fr, auto),
      align(left)[#text(size: 6pt, fill: rgb("#64748b"))[Dictamen Preventivo de Cumplimiento Técnico-Documental (Arts. 89–93 Código de Comercio). Verificación pública: https://mallen.lat/verify/{{EMPRESA_TAX_ID}}]],
      align(right)[#text(size: 6.5pt, weight: "bold", fill: rgb("#0f172a"))[Página #counter(page).display("1 de 1", both: true)]]
    )
  ]
)

#set text(font: ("DejaVu Sans", "Liberation Sans", "Arial"), size: 7.7pt, fill: rgb("#1e293b"), lang: "es")
#set par(justify: true, leading: 0.46em)

// ==================== ENCABEZADO INSTITUCIONAL TIER-1 ====================
#box(
  width: 100%,
  fill: rgb("#0f172a"),
  radius: 5pt,
  inset: (x: 12pt, y: 9pt),
  stroke: 1pt + rgb("#10b981")
)[
  #grid(
    columns: (1fr, auto),
    gutter: 10pt,
    [
      #text(size: 6.6pt, weight: "bold", fill: rgb("#34d399"))[{{PARTNER_HEADER}}]
      #linebreak()
      #text(size: 11pt, weight: "bold", fill: rgb("#ffffff"))[DICTAMEN PERICIAL DETERMINÍSTICO DE EXPOSICIÓN NORMATIVA 360°]
      #linebreak()
      #text(size: 6.8pt, fill: rgb("#cbd5e1"))[Cruce Multivariable INEGI DENUE • SAT (Arts. 69 y 69-B) • STPS (39 NOMs + REPSE) • IMSS • LFPDPPP • 9 Sectoriales • PROFECO]
    ],
    align(right + horizon)[
      #box(fill: rgb("#1e293b"), stroke: 0.6pt + rgb("#34d399"), radius: 4pt, inset: 6pt)[
        #text(size: 6.2pt, fill: rgb("#94a3b8"))[FOLIO CRIPTOGRÁFICO SHA-256]
        #linebreak()
        #text(size: 7.3pt, weight: "bold", fill: rgb("#34d399"))[{{AUDIT_HASH}}]
        #linebreak()
        #text(size: 6pt, fill: rgb("#cbd5e1"))[CORTE UTC: {{FECHA_EMISION}}]
      ]
    ]
  )
]

#v(0.22em)

// ==================== FICHA FORENSE DEL ENTE AUDITADO Y PERFIL SCIAN ====================
#table(
  columns: (18%, 32%, 18%, 32%),
  inset: (x: 6pt, y: 4.2pt),
  stroke: 0.5pt + rgb("#cbd5e1"),
  fill: (x, y) => if calc.odd(x) { rgb("#ffffff") } else { rgb("#f8fafc") },
  [#text(weight: "bold", size: 6.8pt, fill: rgb("#334155"))[ENTE AUDITADO:]],
  [#text(weight: "bold", size: 7.3pt, fill: rgb("#0f172a"))[{{EMPRESA_NOMBRE}}]],
  [#text(weight: "bold", size: 6.8pt, fill: rgb("#334155"))[RFC / ID BÚSQUEDA:]],
  [#text(weight: "bold", size: 7.3pt, fill: rgb("#0f172a"))[{{EMPRESA_TAX_ID}} (Score: {{RISK_SCORE}}/100)]],

  [#text(weight: "bold", size: 6.8pt, fill: rgb("#334155"))[GIRO SCIAN OFICIAL:]],
  [#text(size: 7pt, fill: rgb("#0f172a"))[{{EMPRESA_ACTIVIDAD}}]],
  [#text(weight: "bold", size: 6.8pt, fill: rgb("#334155"))[REPRESENTACIÓN:]],
  [#text(size: 7pt, fill: rgb("#0f172a"))[{{REPRESENTANTE_LEGAL}}]],

  [#text(weight: "bold", size: 6.8pt, fill: rgb("#334155"))[DOMICILIO / DENUE:]],
  [#text(size: 6.9pt, fill: rgb("#0f172a"))[{{EMPRESA_DIRECCION}}]],
  [#text(weight: "bold", size: 6.8pt, fill: rgb("#334155"))[ESTATUS DICTAMEN:]],
  [#text(weight: "bold", size: 7pt, fill: rgb("#b91c1c"))[{{ESTATUS_TEXTO}}]]
)

#v(0.18em)

// ==================== SECCIÓN 1: DIAGNÓSTICO POLIMÓRFICO POR GIRO SCIAN Y PRIMA IMSS ====================
#text(size: 8.2pt, weight: "bold", fill: rgb("#0f172a"))[1. PERFIL DE RIESGO INDUSTRIAL POR GIRO SCIAN, PLANTILLA Y CLASIFICACIÓN IMSS (ART. 196 RACERF)]
#v(0.08em)
El presente dictamen pericial ha sido compilado de forma específica para la unidad económica *"{{EMPRESA_NOMBRE}}"* (`{{EMPRESA_TAX_ID}}`), aplicando el motor de reglas paramétricas sobre su actividad económica registrada (*{{EMPRESA_ACTIVIDAD}}*) en la jurisdicción de *{{EMPRESA_JURISDICCION}}*. Las cargas normativas y multas en Unidades de Medida y Actualización (UMA 2026: \$113.14 MXN) han sido ponderadas conforme a la capacidad económica y plantilla del centro de trabajo (Art. 992 LFT y Art. 75 CFF), sincronizadas 1:1 con el expediente verificable en `https://mallen.lat/verify/{{EMPRESA_TAX_ID}}`.

#v(0.16em)

// ==================== SECCIÓN 2: CRUCE CONTRA BASES DE DATOS OFICIALES Y LISTAS NEGRAS ====================
#text(size: 8.2pt, weight: "bold", fill: rgb("#0f172a"))[2. COMPULSA FORENSE CONTRA PADRONES FEDERALES, LISTAS NEGRAS Y TELEMETRÍA OSINT]
#v(0.08em)
#table(
  columns: (22%, 28%, 22%, 28%),
  inset: (x: 6pt, y: 3.8pt),
  stroke: 0.5pt + rgb("#cbd5e1"),
  fill: (x, y) => if calc.even(x) { rgb("#f1f5f9") } else { rgb("#ffffff") },
  [#text(weight: "bold", size: 6.6pt)[SAT ART. 69-B (EFOS/EDOS):]],
  [#text(weight: "bold", size: 6.6pt, fill: rgb("#0f172a"))[{{SAT_69B_STATUS}}]],
  [#text(weight: "bold", size: 6.6pt)[SAT ART. 69 (CRÉDITOS/CSD):]],
  [#text(weight: "bold", size: 6.6pt, fill: rgb("#0f172a"))[{{SAT_69_STATUS}}]],

  [#text(weight: "bold", size: 6.6pt)[PADRÓN STPS REPSE:]],
  [#text(weight: "bold", size: 6.6pt, fill: rgb("#0f172a"))[{{REPSE_STATUS}}]],
  [#text(weight: "bold", size: 6.6pt)[SFP INHABILITADOS:]],
  [#text(weight: "bold", size: 6.6pt, fill: rgb("#0f172a"))[{{SFP_STATUS}}]],

  [#text(weight: "bold", size: 6.6pt)[CIFRADO TLS/SSL (ART. 19):]],
  [#text(weight: "bold", size: 6.6pt, fill: rgb("#0f172a"))[{{SSL_STATUS}}]],
  [#text(weight: "bold", size: 6.6pt)[AVISO PRIVACIDAD LFPDPPP:]],
  [#text(weight: "bold", size: 6.6pt, fill: rgb("#0f172a"))[{{PRIVACY_STATUS}}]]
)

#v(0.18em)

// ==================== SECCIÓN 3: CUANTIFICACIÓN MATEMÁTICA POR ÓRGANO REGULADOR ====================
#text(size: 8.2pt, weight: "bold", fill: rgb("#0f172a"))[3. MATRIZ CONSOLIDADA DE CONTINGENCIA PATRIMONIAL POR ÓRGANO REGULADOR (UMA 2026)]
#v(0.08em)
#table(
  columns: (26%, 20%, 54%),
  inset: (x: 6pt, y: 4.2pt),
  stroke: 0.5pt + rgb("#cbd5e1"),
  fill: (x, y) => if y == 0 { rgb("#0f172a") } else if y == 7 { rgb("#fef2f2") } else if calc.odd(y) { rgb("#f8fafc") } else { rgb("#ffffff") },
  [#text(fill: white, weight: "bold", size: 6.8pt)[ÓRGANO REGULADOR / MARCO]],
  [#text(fill: white, weight: "bold", size: 6.8pt)[EXPOSICIÓN ESTIMADA]],
  [#text(fill: white, weight: "bold", size: 6.8pt)[DICTAMINACIÓN TÉCNICA ESPECÍFICA AL GIRO AUDITADO]],

  [#text(weight: "bold", size: 6.8pt)[1. STPS (39 NOMs + LFT)]],
  [#text(weight: "bold", size: 7pt, fill: rgb("#b91c1c"))[\$ {{MULTA_STPS}} MXN]],
  [#text(size: 6.6pt)[{{HALLAZGO_TABLA_STPS}}]],

  [#text(weight: "bold", size: 6.8pt)[2. SAT (CFF / LISR / CFDI 4.0)]],
  [#text(weight: "bold", size: 7pt, fill: rgb("#b91c1c"))[\$ {{MULTA_SAT}} MXN]],
  [#text(size: 6.6pt)[{{HALLAZGO_TABLA_SAT}}]],

  [#text(weight: "bold", size: 6.8pt)[3. SABG / LFPDPPP (Privacidad)]],
  [#text(weight: "bold", size: 7pt, fill: rgb("#b91c1c"))[\$ {{MULTA_INAI}} MXN]],
  [#text(size: 6.6pt)[{{INAI_OBS}}]],

  [#text(weight: "bold", size: 6.8pt)[4. IMSS / INFONAVIT (LSS / REPSE)]],
  [#text(weight: "bold", size: 7pt, fill: rgb("#b91c1c"))[\$ {{MULTA_IMSS}} MXN]],
  [#text(size: 6.6pt)[{{HALLAZGO_TABLA_IMSS}}]],

  [#text(weight: "bold", size: 6.8pt)[5. 9 SECTORIALES Y PIPC (SCIAN)]],
  [#text(weight: "bold", size: 7pt, fill: rgb("#b91c1c"))[\$ {{MULTA_PC}} MXN]],
  [#text(size: 6.6pt)[{{HALLAZGO_TABLA_PC}}]],

  [#text(weight: "bold", size: 6.8pt)[6. PROFECO Y ECONOMÍA (LFPC)]],
  [#text(weight: "bold", size: 7pt, fill: rgb("#b91c1c"))[\$ {{MULTA_PROFECO}} MXN]],
  [#text(size: 6.6pt)[{{HALLAZGO_TABLA_PROFECO}}]],

  [#text(weight: "bold", size: 7.2pt, fill: rgb("#991b1b"))[CONTINGENCIA TOTAL AUDITADA:]],
  [#text(weight: "bold", size: 7.8pt, fill: rgb("#991b1b"))[\$ {{ESTIMATED_FINE}} MXN]],
  [#text(weight: "bold", size: 6.6pt, fill: rgb("#7f1d1d"))[Suma determinística de los 37 vectores evaluados conforme a giro SCIAN y plantilla.]]
)

#v(0.18em)

// ==================== SECCIÓN 4: IMPACTO OPERATIVO Y RETORNO DE INVERSIÓN ====================
#grid(
  columns: (1fr, 1fr),
  gutter: 8pt,
  [
    #text(size: 7.8pt, weight: "bold", fill: rgb("#0f172a"))[4. RIESGOS CAUTELARES DE CONTINUIDAD OPERATIVA]
    #v(0.06em)
    #table(
      columns: (36%, 64%),
      inset: 3.8pt,
      stroke: 0.4pt + rgb("#cbd5e1"),
      [#text(weight: "bold", size: 6.4pt, fill: rgb("#991b1b"))[Restricción Sello CSD (Art. 17-H Bis CFF)]],
      [#text(size: 6.4pt)[Suspensión de facturación CFDI 4.0 y bloqueo de cobranza comercial.]],
      [#text(weight: "bold", size: 6.4pt, fill: rgb("#991b1b"))[Clausura STPS / PIPC / COFEPRIS]],
      [#text(size: 6.4pt)[Imposición de sellos por falta de PIPC, estudios NOM-002 o avisos sectoriales.]],
      [#text(weight: "bold", size: 6.4pt, fill: rgb("#b45309"))[Opinión 32-D Negativa y Rechazo B2B]],
      [#text(size: 6.4pt)[Retención de pagos de clientes corporativos y no deducibilidad fiscal.]]
    )
  ],
  [
    #text(size: 7.8pt, weight: "bold", fill: rgb("#0f172a"))[5. COSTO EN DESPACHOS VS. EXPEDIENTE MALLEN 360°]
    #v(0.06em)
    #table(
      columns: (58%, 20%, 22%),
      inset: 3.8pt,
      stroke: 0.4pt + rgb("#cbd5e1"),
      [#text(size: 6.4pt)[Carpetas STPS (NOM-035, 037, 019, 030)]], [#text(size: 6.4pt)[\$25,000]], [#text(weight: "bold", size: 6.2pt, fill: rgb("#047857"))[INCLUIDO]],
      [#text(size: 6.4pt)[Contrato 69-B Fecha Cierta + REPSE IMSS]], [#text(size: 6.4pt)[\$50,000]], [#text(weight: "bold", size: 6.2pt, fill: rgb("#047857"))[INCLUIDO]],
      [#text(size: 6.4pt)[LFPDPPP Integral + PIPC + PROFECO]], [#text(size: 6.4pt)[\$52,000]], [#text(weight: "bold", size: 6.2pt, fill: rgb("#047857"))[INCLUIDO]],
      [#text(weight: "bold", size: 6.4pt)[Consultoría Tradicional: \$127,000 MXN]], [#text(weight: "bold", size: 6.4pt, fill: rgb("#047857"))[Kit 360°:]], [#text(weight: "bold", size: 6.6pt, fill: rgb("#047857"))[\${{ACTIVATION_PRICE}}]]
    )
  ]
)

#v(0.18em)

// ==================== SECCIÓN 6: FIRMA PERICIAL Y FECHA CIERTA ====================
#box(width: 100%, fill: rgb("#f8fafc"), stroke: 0.6pt + rgb("#94a3b8"), radius: 4pt, inset: 6pt)[
  #grid(
    columns: (1fr, auto),
    [
      #text(size: 6.7pt, weight: "bold", fill: rgb("#0f172a"))[FIRMA PERICIAL EMISORA: Firma Pericial Mallen — Sistemas de Auditoría Forense y Compliance 360° (mallen.lat)]
      #linebreak()
      #text(size: 6.4pt, fill: rgb("#334155"))[PERITO DICTAMINADOR: {{AUDITOR_NOMBRE}} • Cédula Profesional: {{AUDITOR_CEDULA}} • Registro STPS: {{AUDITOR_DC5}}]
      #linebreak()
      #text(size: 6.1pt, fill: rgb("#475569"))[Jurisdicción: {{EMPRESA_JURISDICCION}} • Oficial Privacidad: {{OFICIAL_PRIVACIDAD}} • Coordinador NOM-019: {{COORDINADOR_CSH}} • HTTP: {{HTTP_STATUS}}]
    ],
    align(right + horizon)[
      #text(size: 6.8pt, weight: "bold", fill: rgb("#047857"))[EXPEDIENTE + CFDI 4.0: \$ {{ACTIVATION_PRICE}} MXN]
      #linebreak()
      #text(size: 6pt, fill: rgb("#64748b"))[Ver pase a Página 2: Desglose de los 37 Vectores →]
    ]
  )
]

// ==================== PÁGINA 2: ANEXO TÉCNICO FORENSE DE LOS 37 VECTORES EVALUADOS ====================
#pagebreak()

#box(
  width: 100%,
  fill: rgb("#0f172a"),
  radius: 4pt,
  inset: (x: 10pt, y: 7pt),
  stroke: 0.8pt + rgb("#3b82f6")
)[
  #text(size: 9.2pt, weight: "bold", fill: rgb("#ffffff"))[ANEXO PERICIAL: DESGLOSE INDIVIDUAL DE LOS 37 VECTORES NORMATIVOS AUDITADOS]
  #linebreak()
  #text(size: 6.7pt, fill: rgb("#93c5fd"))[Ente Auditado: {{EMPRESA_NOMBRE}} ({{EMPRESA_TAX_ID}}) • Giro Aplicado: {{EMPRESA_ACTIVIDAD}} • Sincronización Criptográfica: {{AUDIT_HASH}}]
]

#v(0.18em)

#text(size: 6.5pt, fill: rgb("#334155"))[
  A continuación se presenta la evaluación individualizada de los *37 Vectores Oficiales de Cumplimiento Normativo Mexicano* (STPS, SAT, IMSS/INFONAVIT, SABG/LFPDPPP, 9 Módulos Sectoriales/PLD y PROFECO/Economía), determinando para cada vector su aplicabilidad al giro SCIAN de la empresa, su rango sancionatorio en UMAs vigentes (\$113.14 MXN) y el requerimiento técnico-documental específico:
]

#v(0.14em)

{{VECTORS_37_TABLE_TYPST}}

#v(0.18em)
#box(width: 100%, fill: rgb("#f1f5f9"), stroke: 0.5pt + rgb("#cbd5e1"), radius: 4pt, inset: 5pt)[
  #text(size: 6pt, fill: rgb("#475569"))[
    *NOTA METODOLÓGICA Y ALCANCE LEGAL (ART. 89 CÓDIGO DE COMERCIO):* El presente Anexo Pericial forma parte indivisible del Dictamen con huella SHA-256 `{{AUDIT_HASH}}`. Los vectores marcados como *"EXENTO SCIAN"* o *"BLINDADO"* presentan contingencia de \$0.00 MXN al no actualizarse el supuesto normativo en la actividad económica evaluada o al haberse acreditado el control técnico correspondiente. El Kit de Blindaje Legal 360° entrega los 10 Certificados de Acreditación y los 7 Instrumentos Jurídicos listos para firma autógrafa o electrónica para subsanar de inmediato los vectores con exposición activa.
  ]
]
