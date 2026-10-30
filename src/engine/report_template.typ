#set page(
  paper: "us-letter",
  margin: (top: 1.1cm, bottom: 1.2cm, left: 1.2cm, right: 1.2cm),
  fill: rgb("#ffffff"),
  header: context {
    if counter(page).get().first() > 1 [
      #grid(
        columns: (1fr, auto),
        align(left)[#text(size: 6.4pt, weight: "bold", fill: rgb("#0f172a"))[FIRMA PERICIAL MALLEN • EXPEDIENTE FORENSE DE CUMPLIMIENTO 360° • {{EMPRESA_TAX_ID}}]],
        align(right)[#text(size: 6.4pt, fill: rgb("#475569"))[SELLO CRIPTOGRÁFICO SHA-256: {{AUDIT_HASH}}]]
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
      align(left)[#text(size: 5.9pt, fill: rgb("#64748b"))[Dictamen Preventivo de Cumplimiento Técnico-Documental (Arts. 89–93 Código de Comercio). Verificación: https://mallen.lat/verify/{{EMPRESA_TAX_ID}}]],
      align(right)[#text(size: 6.4pt, weight: "bold", fill: rgb("#0f172a"))[Página #counter(page).display("1 de 1", both: true)]]
    )
  ]
)

#set text(font: ("DejaVu Sans", "Liberation Sans", "Arial"), size: 7.5pt, fill: rgb("#1e293b"), lang: "es")
#set par(justify: true, leading: 0.44em)

// ==================== PÁGINA 1: DICTAMEN EJECUTIVO, COMPULSA Y GRÁFICAS ====================
#box(
  width: 100%,
  fill: rgb("#0f172a"),
  radius: 5pt,
  inset: (x: 12pt, y: 8.5pt),
  stroke: 1pt + rgb("#10b981")
)[
  #grid(
    columns: (1fr, auto),
    gutter: 10pt,
    [
      #text(size: 6.5pt, weight: "bold", fill: rgb("#34d399"))[{{PARTNER_HEADER}}]
      #linebreak()
      #text(size: 11pt, weight: "bold", fill: rgb("#ffffff"))[DICTAMEN PERICIAL DETERMINÍSTICO DE EXPOSICIÓN NORMATIVA 360°]
      #linebreak()
      #text(size: 6.6pt, fill: rgb("#cbd5e1"))[Auditoría Multivariable INEGI DENUE • SAT (Arts. 69 y 69-B) • STPS (39 NOMs + REPSE) • IMSS • LFPDPPP • 9 Sectoriales • PROFECO]
    ],
    align(right + horizon)[
      #box(fill: rgb("#1e293b"), stroke: 0.6pt + rgb("#34d399"), radius: 4pt, inset: 5.5pt)[
        #text(size: 6pt, fill: rgb("#94a3b8"))[FOLIO CRIPTOGRÁFICO SHA-256]
        #linebreak()
        #text(size: 7.2pt, weight: "bold", fill: rgb("#34d399"))[{{AUDIT_HASH}}]
        #linebreak()
        #text(size: 5.8pt, fill: rgb("#cbd5e1"))[CORTE UTC: {{FECHA_EMISION}}]
      ]
    ]
  )
]

#v(0.18em)

// FICHA FORENSE DEL ENTE AUDITADO
#table(
  columns: (17%, 33%, 17%, 33%),
  inset: (x: 5.5pt, y: 3.8pt),
  stroke: 0.5pt + rgb("#cbd5e1"),
  fill: (x, y) => if calc.odd(x) { rgb("#ffffff") } else { rgb("#f8fafc") },
  [#text(weight: "bold", size: 6.6pt, fill: rgb("#334155"))[ENTE AUDITADO:]],
  [#text(weight: "bold", size: 7.1pt, fill: rgb("#0f172a"))[{{EMPRESA_NOMBRE}}]],
  [#text(weight: "bold", size: 6.6pt, fill: rgb("#334155"))[RFC / ID BÚSQUEDA:]],
  [#text(weight: "bold", size: 7.1pt, fill: rgb("#0f172a"))[{{EMPRESA_TAX_ID}} (Índice Riesgo: {{RISK_SCORE}}/100)]],

  [#text(weight: "bold", size: 6.6pt, fill: rgb("#334155"))[GIRO SCIAN OFICIAL:]],
  [#text(size: 6.8pt, fill: rgb("#0f172a"))[{{EMPRESA_ACTIVIDAD}}]],
  [#text(weight: "bold", size: 6.6pt, fill: rgb("#334155"))[REPRESENTACIÓN:]],
  [#text(size: 6.8pt, fill: rgb("#0f172a"))[{{REPRESENTANTE_LEGAL}}]],

  [#text(weight: "bold", size: 6.6pt, fill: rgb("#334155"))[DOMICILIO / DENUE:]],
  [#text(size: 6.7pt, fill: rgb("#0f172a"))[{{EMPRESA_DIRECCION}}]],
  [#text(weight: "bold", size: 6.6pt, fill: rgb("#334155"))[ESTATUS DICTAMEN:]],
  [#text(weight: "bold", size: 6.8pt, fill: rgb("#b91c1c"))[{{ESTATUS_TEXTO}}]]
)

#v(0.14em)

#text(size: 8pt, weight: "bold", fill: rgb("#0f172a"))[1. METODOLOGÍA DE PARAMETRIZACIÓN POR GIRO SCIAN, CAPACIDAD ECONÓMICA Y PRIMA IMSS]
#v(0.06em)
Dictamen pericial compilado específicamente para *"{{EMPRESA_NOMBRE}}"* (`{{EMPRESA_TAX_ID}}`), evaluando su actividad económica (*{{EMPRESA_ACTIVIDAD}}*) en la jurisdicción de *{{EMPRESA_JURISDICCION}}*. Los cálculos sancionatorios en Unidades de Medida y Actualización (UMA 2026: \$113.14 MXN) ponderan el estrato de personal conforme al Art. 992 de la LFT y Art. 75 del CFF, manteniendo coincidencia matemática exacta entre esta Matriz Consolidada (Sección 3) y la suma de los 37 Vectores del Anexo Pericial (Página 2).

#v(0.14em)

#text(size: 8pt, weight: "bold", fill: rgb("#0f172a"))[2. COMPULSA FORENSE EN BASES DE DATOS OFICIALES, LISTAS NEGRAS Y TELEMETRÍA WEB]
#v(0.06em)
#table(
  columns: (21%, 29%, 21%, 29%),
  inset: (x: 5.5pt, y: 3.5pt),
  stroke: 0.5pt + rgb("#cbd5e1"),
  fill: (x, y) => if calc.even(x) { rgb("#f1f5f9") } else { rgb("#ffffff") },
  [#text(weight: "bold", size: 6.4pt)[SAT ART. 69-B (EFOS/EDOS):]],
  [#text(weight: "bold", size: 6.4pt, fill: rgb("#0f172a"))[{{SAT_69B_STATUS}}]],
  [#text(weight: "bold", size: 6.4pt)[SAT ART. 69 (CRÉDITOS/CSD):]],
  [#text(weight: "bold", size: 6.4pt, fill: rgb("#0f172a"))[{{SAT_69_STATUS}}]],

  [#text(weight: "bold", size: 6.4pt)[PADRÓN STPS REPSE:]],
  [#text(weight: "bold", size: 6.4pt, fill: rgb("#0f172a"))[{{REPSE_STATUS}}]],
  [#text(weight: "bold", size: 6.4pt)[SFP INHABILITADOS:]],
  [#text(weight: "bold", size: 6.4pt, fill: rgb("#0f172a"))[{{SFP_STATUS}}]],

  [#text(weight: "bold", size: 6.4pt)[CIFRADO TLS/SSL (ART. 19):]],
  [#text(weight: "bold", size: 6.4pt, fill: rgb("#0f172a"))[{{SSL_STATUS}}]],
  [#text(weight: "bold", size: 6.4pt)[AVISO PRIVACIDAD LFPDPPP:]],
  [#text(weight: "bold", size: 6.4pt, fill: rgb("#0f172a"))[{{PRIVACY_STATUS}}]]
)

#v(0.14em)

#text(size: 8pt, weight: "bold", fill: rgb("#0f172a"))[3. MATRIZ CONSOLIDADA DE CONTINGENCIA PATRIMONIAL POR ÓRGANO REGULADOR (SINCRONIZADA 1:1)]
#v(0.06em)
#table(
  columns: (25%, 19%, 56%),
  inset: (x: 5.5pt, y: 3.8pt),
  stroke: 0.5pt + rgb("#cbd5e1"),
  fill: (x, y) => if y == 0 { rgb("#0f172a") } else if y == 7 { rgb("#fef2f2") } else if calc.odd(y) { rgb("#f8fafc") } else { rgb("#ffffff") },
  [#text(fill: white, weight: "bold", size: 6.6pt)[ÓRGANO REGULADOR / VECTORES]],
  [#text(fill: white, weight: "bold", size: 6.6pt)[SUBTOTAL EXPOSICIÓN]],
  [#text(fill: white, weight: "bold", size: 6.6pt)[DICTAMINACIÓN FORENSE ESPECÍFICA AL GIRO AUDITADO]],

  [#text(weight: "bold", size: 6.6pt)[1. STPS (Vectores 01 al 08)]],
  [#text(weight: "bold", size: 6.9pt, fill: rgb("#b91c1c"))[\$ {{MULTA_STPS}} MXN]],
  [#text(size: 6.3pt)[{{HALLAZGO_TABLA_STPS}}]],

  [#text(weight: "bold", size: 6.6pt)[2. SAT / CFF (Vectores 09 al 14)]],
  [#text(weight: "bold", size: 6.9pt, fill: rgb("#b91c1c"))[\$ {{MULTA_SAT}} MXN]],
  [#text(size: 6.3pt)[{{HALLAZGO_TABLA_SAT}}]],

  [#text(weight: "bold", size: 6.6pt)[3. IMSS / INFONAVIT (Vectores 15 al 20)]],
  [#text(weight: "bold", size: 6.9pt, fill: rgb("#b91c1c"))[\$ {{MULTA_IMSS}} MXN]],
  [#text(size: 6.3pt)[{{HALLAZGO_TABLA_IMSS}}]],

  [#text(weight: "bold", size: 6.6pt)[4. SABG / LFPDPPP (Vectores 21 al 26)]],
  [#text(weight: "bold", size: 6.9pt, fill: rgb("#b91c1c"))[\$ {{MULTA_INAI}} MXN]],
  [#text(size: 6.3pt)[{{INAI_OBS}}]],

  [#text(weight: "bold", size: 6.6pt)[5. 9 SECTORIALES / PIPC (Vectores 27 al 32)]],
  [#text(weight: "bold", size: 6.9pt, fill: rgb("#b91c1c"))[\$ {{MULTA_PC}} MXN]],
  [#text(size: 6.3pt)[{{HALLAZGO_TABLA_PC}}]],

  [#text(weight: "bold", size: 6.6pt)[6. PROFECO / ECONOMÍA (Vectores 33 al 37)]],
  [#text(weight: "bold", size: 6.9pt, fill: rgb("#b91c1c"))[\$ {{MULTA_PROFECO}} MXN]],
  [#text(size: 6.3pt)[{{HALLAZGO_TABLA_PROFECO}}]],

  [#text(weight: "bold", size: 7pt, fill: rgb("#991b1b"))[CONTINGENCIA TOTAL AUDITADA:]],
  [#text(weight: "bold", size: 7.6pt, fill: rgb("#991b1b"))[\$ {{ESTIMATED_FINE}} MXN]],
  [#text(weight: "bold", size: 6.4pt, fill: rgb("#7f1d1d"))[Suma exacta de los 37 vectores evaluados en la Página 2 conforme a UMA 2026 (\$113.14 MXN).]]
)

#v(0.14em)

// ==================== SECCIÓN 4: GRÁFICA VISUAL DE CONCENTRACIÓN PATRIMONIAL Y SEVERIDAD ====================
#text(size: 8pt, weight: "bold", fill: rgb("#0f172a"))[4. GRÁFICA PERICIAL DE DISTRIBUCIÓN DE RIESGO PATRIMONIAL Y MEDIDAS DE APREMIO]
#v(0.06em)
{{VISUAL_RISK_CHART_TYPST}}

#v(0.14em)

// PIE EJECUTIVO DE PÁGINA 1
#box(width: 100%, fill: rgb("#f8fafc"), stroke: 0.6pt + rgb("#94a3b8"), radius: 4pt, inset: 5.5pt)[
  #grid(
    columns: (1fr, auto),
    [
      #text(size: 6.5pt, weight: "bold", fill: rgb("#0f172a"))[PERITO DICTAMINADOR: {{AUDITOR_NOMBRE}} • Cédula: {{AUDITOR_CEDULA}} • Reg. STPS: {{AUDITOR_DC5}}]
      #linebreak()
      #text(size: 6pt, fill: rgb("#475569"))[Jurisdicción: {{EMPRESA_JURISDICCION}} • Oficial Privacidad: {{OFICIAL_PRIVACIDAD}} • Coordinador NOM-019: {{COORDINADOR_CSH}} • HTTP: {{HTTP_STATUS}}]
    ],
    align(right + horizon)[
      #text(size: 6.5pt, weight: "bold", fill: rgb("#047857"))[PÁGINA 2: LOS 37 VECTORES • PÁGINA 3: HOJA DE RUTA Y ROI →]
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
      #text(size: 8.8pt, weight: "bold", fill: rgb("#ffffff"))[ANEXO FORENSE I: CÉDULA DE EVALUACIÓN DE LOS 37 VECTORES NORMATIVOS]
      #linebreak()
      #text(size: 6.3pt, fill: rgb("#93c5fd"))[Ente Auditado: {{EMPRESA_NOMBRE}} ({{EMPRESA_TAX_ID}}) • Giro SCIAN: {{EMPRESA_ACTIVIDAD}}]
    ],
    align(right + horizon)[
      #text(size: 7pt, weight: "bold", fill: rgb("#fbbf24"))[TOTAL: \$ {{ESTIMATED_FINE}} MXN]
    ]
  )
]

#v(0.1em)

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
  #text(size: 9pt, weight: "bold", fill: rgb("#ffffff"))[ANEXO FORENSE II: HOJA DE RUTA DE REMEDIACIÓN 30-60-90 DÍAS, INGENIERÍA ROI E INVENTARIO DEL KIT 360°]
  #linebreak()
  #text(size: 6.4pt, fill: rgb("#6ee7b7"))[Plan de Ejecución Preventiva para {{EMPRESA_NOMBRE}} ({{EMPRESA_TAX_ID}}) • Sello Criptográfico: {{AUDIT_HASH}}]
]

#v(0.18em)

#text(size: 8pt, weight: "bold", fill: rgb("#0f172a"))[5. CRONOGRAMA EJECUTIVO DE REMEDIACIÓN E IMPLEMENTACIÓN EN SITIO (30 / 60 / 90 DÍAS)]
#v(0.08em)
#table(
  columns: (18%, 27%, 37%, 18%),
  inset: (x: 6pt, y: 4.5pt),
  stroke: 0.5pt + rgb("#cbd5e1"),
  fill: (x, y) => if y == 0 { rgb("#0f172a") } else if calc.odd(y) { rgb("#f8fafc") } else { rgb("#ffffff") },
  [#text(fill: white, weight: "bold", size: 6.6pt)[FASE / HORIZONTE]],
  [#text(fill: white, weight: "bold", size: 6.6pt)[VECTORES CRÍTICOS A BLINDAR]],
  [#text(fill: white, weight: "bold", size: 6.6pt)[ACCIÓN TÉCNICA CON LOS INSTRUMENTOS DEL KIT MALLEN]],
  [#text(fill: white, weight: "bold", size: 6.6pt)[RIESGO MITIGADO]],

  [#text(weight: "bold", size: 6.6pt, fill: rgb("#b91c1c"))[FASE 1: INMEDIATA\n(Días 1 a 15)]],
  [#text(size: 6.4pt)[*LFPDPPP, PROFECO y SAT Art. 69-B*\n(Vectores 09, 10, 21, 22, 23, 33 y 35)]],
  [#text(size: 6.4pt)[1. Publicar en el pie de página web el *Aviso de Privacidad Integral* y los *Términos E-Commerce Art. 76 BIS LFPC* incluidos en el Kit.\n2. Firmar el *Contrato Marco de Materialidad B2B con Fecha Cierta* y recabar el *Consentimiento Biométrico/Nómina* del personal.]],
  [#text(weight: "bold", size: 6.5pt, fill: rgb("#047857"))[Evita multas directas de oscuridad web y rechazo fiscal de deducciones]],

  [#text(weight: "bold", size: 6.6pt, fill: rgb("#b45309"))[FASE 2: LABORAL\n(Días 16 a 45)]],
  [#text(size: 6.4pt)[*STPS e IMSS Patronal*\n(Vectores 01, 02, 04, 15, 16 y 17)]],
  [#text(size: 6.4pt)[1. Suscribir el *Acta Constitutiva de la Comisión Mixta (NOM-019)* y el *Diagnóstico Preventivo NOM-030*.\n2. Aplicar los cuestionarios de la *Política NOM-035*, el *Adendum de Teletrabajo NOM-037* y ejecutar la confronta SUA vs. CFDI de Nómina.]],
  [#text(weight: "bold", size: 6.5pt, fill: rgb("#047857"))[Desactiva multas por empleado en inspecciones extraordinarias STPS]],

  [#text(weight: "bold", size: 6.6pt, fill: rgb("#1d4ed8"))[FASE 3: FÍSICA Y SECTORIAL\n(Días 46 a 90)]],
  [#text(size: 6.4pt)[*Seguridad Física, PIPC y Giro SCIAN*\n(Vectores 03, 05, 06, 07, 13 y 29 al 31)]],
  [#text(size: 6.4pt)[1. Integrar la carpeta del *Programa Interno de Protección Civil (PIPC)* con el Acta de 4 Brigadas.\n2. Archivar el *Expediente de Beneficiario Controlador (Art. 32-B Ter CFF)* y las bitácoras sectoriales propias del giro `{{EMPRESA_ACTIVIDAD}}`.]],
  [#text(weight: "bold", size: 6.5pt, fill: rgb("#047857"))[Previene clausura total o parcial del establecimiento mercantil]]
)

#v(0.18em)

#text(size: 8pt, weight: "bold", fill: rgb("#0f172a"))[6. CALENDARIO ANUAL DE OBLIGACIONES PERIÓDICAS DE CUMPLIMIENTO (AGENDA DEL CONTADOR Y APODERADO)]
#v(0.08em)
#table(
  columns: (20%, 26%, 54%),
  inset: (x: 6pt, y: 4pt),
  stroke: 0.5pt + rgb("#cbd5e1"),
  fill: (x, y) => if y == 0 { rgb("#1e293b") } else if calc.odd(y) { rgb("#f8fafc") } else { rgb("#ffffff") },
  [#text(fill: white, weight: "bold", size: 6.5pt)[PERIODICIDAD / MES]],
  [#text(fill: white, weight: "bold", size: 6.5pt)[OBLIGACIÓN Y FUNDAMENTO]],
  [#text(fill: white, weight: "bold", size: 6.5pt)[CONTROL DOCUMENTAL EXIGIBLE EN AUDITORÍA]],

  [#text(weight: "bold", size: 6.4pt)[Mensual (Días 5 y 17)]],
  [#text(size: 6.4pt)[REP CFDI 4.0, SUA IMSS y Opinión 32-D]],
  [#text(size: 6.4pt)[Emisión de Complementos de Pago antes del día 5, confronta EMA/EBA vs. SUA el día 17 y descarga de Opinión 32-D positiva (SAT, IMSS e INFONAVIT).]],

  [#text(weight: "bold", size: 6.4pt)[Febrero (Anual)]],
  [#text(size: 6.4pt)[Prima de Riesgo de Trabajo IMSS (Art. 74 LSS)]],
  [#text(size: 6.4pt)[Presentación de la Determinación Anual de la Prima en el Seguro de Riesgos de Trabajo conforme a la siniestralidad del ejercicio previo.]],

  [#text(weight: "bold", size: 6.4pt)[Enero / Mayo / Septiembre]],
  [#text(size: 6.4pt)[Informativas ICSOE (IMSS) y SISSUB (INFONAVIT)]],
  [#text(size: 6.4pt)[Presentación cuatrimestral a más tardar el día 17 de los contratos de servicios especializados u obras ejecutadas (Art. 15-A LSS).]],

  [#text(weight: "bold", size: 6.4pt)[Trimestral / Anual]],
  [#text(size: 6.4pt)[Recorridos NOM-019, NOM-035 y Revalidación PIPC]],
  [#text(size: 6.4pt)[Actas trimestrales de verificación de la Comisión Mixta SST, simulacros semestrales de evacuación PIPC y actualización del Documento de Seguridad LFPDPPP.]]
)

#v(0.18em)

#grid(
  columns: (1fr, 1fr),
  gutter: 8pt,
  [
    #text(size: 7.8pt, weight: "bold", fill: rgb("#0f172a"))[7. ANÁLISIS FINANCIERO Y RETORNO DE INVERSIÓN (ROI)]
    #v(0.06em)
    #table(
      columns: (58%, 20%, 22%),
      inset: 3.8pt,
      stroke: 0.4pt + rgb("#cbd5e1"),
      fill: (x, y) => if y == 0 { rgb("#0f172a") } else if y == 5 { rgb("#ecfdf5") } else { rgb("#ffffff") },
      [#text(fill: white, weight: "bold", size: 6.2pt)[CONCEPTO EN DESPACHOS TRADICIONALES]],
      [#text(fill: white, weight: "bold", size: 6.2pt)[MERCADO]],
      [#text(fill: white, weight: "bold", size: 6.2pt)[KIT MALLEN]],
      [#text(size: 6.2pt)[Pólizas STPS (NOM-035, 037, 019, 030)]], [#text(size: 6.2pt)[\$25,000]], [#text(weight: "bold", size: 6.1pt, fill: rgb("#047857"))[INCLUIDO]],
      [#text(size: 6.2pt)[Contrato Marco 69-B Fecha Cierta + Bitácora]], [#text(size: 6.2pt)[\$35,000]], [#text(weight: "bold", size: 6.1pt, fill: rgb("#047857"))[INCLUIDO]],
      [#text(size: 6.2pt)[Aviso Integral LFPDPPP + Biométricos + ARCO]], [#text(size: 6.2pt)[\$22,000]], [#text(weight: "bold", size: 6.1pt, fill: rgb("#047857"))[INCLUIDO]],
      [#text(size: 6.2pt)[Guía PIPC + Protocolo IMSS REPSE + PROFECO]], [#text(size: 6.2pt)[\$45,000]], [#text(weight: "bold", size: 6.1pt, fill: rgb("#047857"))[INCLUIDO]],
      [#text(weight: "bold", size: 6.4pt, fill: rgb("#065f46"))[Ahorro Directo en Consultoría (>95%):]],
      [#text(weight: "bold", size: 6.3pt, fill: rgb("#64748b"))[\$127,000]],
      [#text(weight: "bold", size: 6.6pt, fill: rgb("#047857"))[\${{ACTIVATION_PRICE}}]]
    )
  ],
  [
    #text(size: 7.8pt, weight: "bold", fill: rgb("#0f172a"))[8. INVENTARIO DE LOS 17 INSTRUMENTOS DEL KIT (.ZIP)]
    #v(0.06em)
    #box(width: 100%, fill: rgb("#f8fafc"), stroke: 0.5pt + rgb("#cbd5e1"), radius: 4pt, inset: 5.5pt)[
      #text(size: 6.2pt, fill: rgb("#1e293b"))[
        *A. 10 Certificados de Acreditación Normativa (PDF Sellados SHA-256):*\
        NOM-035 Psicosocial • NOM-037 Teletrabajo • NOM-019/030 Comisión SST • SAT Art. 69-B Materialidad • LFPDPPP Privacidad • IMSS 32-D/REPSE • Protección Civil PIPC • PROFECO Art. 76 BIS • LFPIORPI PLD • Blindaje Maestro 360°.\
        #v(0.2em)
        *B. 7 Instrumentos Jurídicos Editables (Word-HTML y PDF Listos para Firma):*\
        1. Aviso de Privacidad Integral, Simplificado y Formato Biométrico LFPDPPP.\
        2. Política de Prevención Psicosocial y Cuestionarios NOM-035-STPS.\
        3. Acta Constitutiva de Comisión Mixta NOM-019 y Diagnóstico NOM-030.\
        4. Adendum de Teletrabajo y Lista Ergonómica NOM-037-STPS-2023.\
        5. Contrato Marco B2B con Cláusula de Fecha Cierta SCJN y Bitácora Art. 69-B.\
        6. Protocolo de Blindaje Patronal IMSS SUA y Checklist Mensual REPSE.\
        7. Términos y Condiciones E-Commerce Art. 76 BIS LFPC y Póliza de Garantía.
      ]
    ]
  ]
)

#v(0.18em)

// BLOQUE DE FIRMA PERICIAL Y SELLO DE INTEGRIDAD EN PÁGINA 3
#box(width: 100%, fill: rgb("#0f172a"), stroke: 0.8pt + rgb("#10b981"), radius: 4pt, inset: 7pt)[
  #grid(
    columns: (1fr, auto),
    gutter: 10pt,
    [
      #text(size: 6.8pt, weight: "bold", fill: rgb("#34d399"))[CERTIFICACIÓN CRIPTOGRÁFICA DE INTEGRIDAD Y ALCANCE METODOLÓGICO (ARTS. 89–93 CÓDIGO DE COMERCIO)]
      #linebreak()
      #text(size: 6pt, fill: rgb("#e2e8f0"))[
        El presente Dictamen Pericial y sus Anexos I y II han sido compilados por *{{AUDITOR_NOMBRE}}* (Cédula Profesional: `{{AUDITOR_CEDULA}}` | Registro STPS: `{{AUDITOR_DC5}}`). La huella criptográfica `{{AUDIT_HASH}}` garantiza la inalterabilidad del expediente. Los vectores clasificados como *"EXENTO SCIAN"* o *"BLINDADO"* reflejan una contingencia de \$0.00 MXN para el giro evaluado.
      ]
    ],
    align(right + horizon)[
      #box(fill: rgb("#065f46"), stroke: 0.6pt + rgb("#34d399"), radius: 3pt, inset: 5pt)[
        #text(size: 6pt, fill: rgb("#a7f3d0"))[EXPEDIENTE COMPLETO + CFDI 4.0]
        #linebreak()
        #text(size: 7.5pt, weight: "bold", fill: rgb("#ffffff"))[\$ {{ACTIVATION_PRICE}} MXN]
      ]
    ]
  )
]
