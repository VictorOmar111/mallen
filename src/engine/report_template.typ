#set page(paper: "a4", margin: (x: 1.5cm, y: 1.4cm))
#set text(size: 8.4pt)

// ENCABEZADO CORPORATIVO PERICIAL (SOPORTE WHITE-LABEL B2B2B)
#rect(width: 100%, fill: rgb("#0f172a"), inset: 10pt, radius: 4pt)[
  #align(center)[
    #text(fill: white, weight: "bold", size: 12pt)[DICTAMEN PERICIAL DE AUDITORÍA LEGAL, FISCAL Y REGULATORIA] \
    #v(0.2em)
    #text(fill: rgb("#cbd5e1"), size: 7.8pt)[{{PARTNER_HEADER}} — EVALUACIÓN INTEGRAL DE RIESGO NORMATIVO Y PROTECCIÓN PATRIMONIAL B2B]
  ]
]

#v(0.3em)

// DASHBOARD PERICIAL: FICHA TÉCNICA + DONA MULTICOLOR 6 SECTORES
#grid(
  columns: (1.25fr, 1.35fr),
  gutter: 0.8em,
  [
    #v(0.1em)
    *EMPRESA AUDITADA:* {{EMPRESA_NOMBRE}} \
    *IDENTIFICADOR / RFC:* {{EMPRESA_TAX_ID}} \
    *SECTOR / SCIAN:* {{EMPRESA_ACTIVIDAD}} \
    *UBICACIÓN / PLANTILLA:* {{EMPRESA_DIRECCION}} \
    *FECHA DE CORTE OSINT:* {{FECHA_EMISION}} \
    *ESTATUS AUDITORÍA:* #text(fill: rgb("#dc2626"), weight: "bold")[EXPOSICIÓN PATRIMONIAL CRÍTICA ACTIVA]
  ],
  [
    #rect(width: 100%, fill: rgb("#f8fafc"), stroke: 1pt + rgb("#cbd5e1"), inset: 5pt, radius: 5pt)[
      #align(center)[
        #text(size: 7.5pt, weight: "bold", fill: rgb("#0f172a"))[ÍNDICE DE VULNERABILIDAD: {{RISK_SCORE}} / 100 PUNTOS] \
        #v(0.3em)
        #grid(
          columns: (auto, 1fr),
          gutter: 0.5em,
          [
            #box(width: 52pt, height: 52pt)[
              #place(top + left, circle(radius: 26pt, fill: rgb("#e2e8f0")))
              #place(top + left, circle(radius: 26pt, fill: rgb("#2563eb")))
              #place(top + left, dx: 2pt, dy: 1pt, circle(radius: 24.5pt, fill: rgb("#dc2626")))
              #place(top + left, dx: 4pt, dy: 2pt, circle(radius: 23pt, fill: rgb("#d97706")))
              #place(top + left, dx: 6pt, dy: 3pt, circle(radius: 21pt, fill: rgb("#16a34a")))
              #place(top + left, dx: 8pt, dy: 4pt, circle(radius: 19pt, fill: rgb("#ea580c")))
              #place(top + left, dx: 10pt, dy: 5pt, circle(radius: 17pt, fill: rgb("#7c3aed")))
              #place(top + left, dx: 12pt, dy: 12pt, circle(radius: 14pt, fill: rgb("#f8fafc")))
              #place(center + horizon)[
                #align(center)[
                  #text(size: 10pt, weight: "bold", fill: rgb("#0f172a"))[{{RISK_SCORE}}] \
                  #text(size: 4.5pt, weight: "bold", fill: rgb("#64748b"))[RIESGO]
                ]
              ]
            ]
          ],
          [
            #align(left + horizon)[
              #text(size: 5.2pt)[
                #box(fill: rgb("#2563eb"), width: 4pt, height: 4pt, radius: 1pt) *STPS (39 NOMs):* {{SCORE_STPS}} pts (Inspección Laboral) \
                #box(fill: rgb("#dc2626"), width: 4pt, height: 4pt, radius: 1pt) *SAT (CFF 69-B/CSD):* {{SCORE_SAT}} pts (Materialidad Fiscal) \
                #box(fill: rgb("#d97706"), width: 4pt, height: 4pt, radius: 1pt) *SABG / INAI (LFPDPPP):* {{SCORE_INAI}} pts (Datos y Biométricos) \
                #box(fill: rgb("#16a34a"), width: 4pt, height: 4pt, radius: 1pt) *IMSS / INFONAVIT:* {{SCORE_IMSS}} pts (32-D / REPSE / SBC) \
                #box(fill: rgb("#ea580c"), width: 4pt, height: 4pt, radius: 1pt) *SECTORIAL / PIPC:* {{SCORE_PC}} pts (Clausura Operativa) \
                #box(fill: rgb("#7c3aed"), width: 4pt, height: 4pt, radius: 1pt) *PROFECO (LFPC):* {{SCORE_PROFECO}} pts (Art. 76 BIS / RRPAC)
              ]
            ]
          ]
        )
      ]
    ]
  ]
)

#v(0.25em)

// BANNER DE EXPOSICIÓN FINANCIERA (ANCLAJE PSICOLÓGICO DE CONTINGENCIA)
#rect(width: 100%, fill: rgb("#fef2f2"), stroke: 1pt + rgb("#fca5a5"), inset: 6pt, radius: 4pt)[
  #grid(
    columns: (1fr, auto),
    [
      #text(fill: rgb("#0f172a"), weight: "bold", size: 8pt)[CONTINGENCIA ECONÓMICA ESTIMADA POR MULTAS ACUMULABLES (UMA 2026):] \
      #v(0.1em)
      #text(fill: rgb("#dc2626"), size: 12.5pt, weight: "bold")[\$ {{ESTIMATED_FINE}} MXN]
    ],
    [
      #align(right + horizon)[
        #box(fill: rgb("#dc2626"), inset: (x: 6pt, y: 3pt), radius: 3pt)[
          #text(fill: white, weight: "bold", size: 7.5pt)[ACCIÓN CORRECTIVA REQUERIDA]
        ]
      ]
    ]
  )
]

#v(0.25em)
#line(length: 100%, stroke: 0.5pt + rgb("#cbd5e1"))

#text(size: 9.2pt, weight: "bold", fill: rgb("#0f172a"))[1. RESUMEN EJECUTIVO Y METODOLOGÍA DE EVALUACIÓN PERICIAL]
#v(0.15em)
El presente dictamen cuantifica la exposición legal, fiscal y administrativa de *"{{EMPRESA_NOMBRE}}"* cruzando registros públicos (INEGI DENUE, padrones SAT Art. 69/69-B, STPS REPSE, SFP) y telemetría OSINT no intrusiva. El puntaje de *{{RISK_SCORE}}/100* pondera: *Impacto Financiero en UMAs*, *Visibilidad de Brechas ante Bots Gubernamentales* y *Riesgo de Suspensión de Sellos o Clausura*.

#v(0.3em)

#text(size: 9.2pt, weight: "bold", fill: rgb("#0f172a"))[2. MARCO NORMATIVO AUDITADO (360° COMPLIANCE MEXICANO)]
#v(0.15em)
- *STPS (39 NOMs):* LFT Arts. 132, 330-A a 330-K, 509, 540 y 994 (Seguridad, Salud, NOM-019, NOM-030, NOM-035, NOM-036-1 y NOM-037).
- *SAT (CFF / LISR / LIVA):* Art. 69-B (Materialidad B2B y Fecha Cierta SCJN), Art. 17-K (Buzón), Art. 17-H Bis (CSD), Art. 28 y Art. 29-A.
- *PROTECCIÓN DE DATOS (LFPDPPP / SABG ex-INAI):* Arts. 8, 9, 15, 16, 17, 19, 22-37 y 63-64 (Avisos, ARCO, Biométricos y SSL).
- *IMSS e INFONAVIT:* LSS Arts. 15, 15-A (REPSE / ICSOE / SISSUB), 39 (Opinión 32-D), 71-75 (Prima de Riesgo) y 304-A.
- *SECTORIAL Y PROTECCIÓN CIVIL:* LGPC (PIPC >100 m²), COFEPRIS, SEMARNAT, CNBV/LFPIORPI, SICT, SEP y SECTUR según código SCIAN.
- *PROFECO:* LFPC Arts. 32, 76 BIS (Comercio Digital), 85-87 (Contratos de Adhesión RRPAC) y 128 BIS.

#v(0.3em)

#text(size: 9.2pt, weight: "bold", fill: rgb("#0f172a"))[3. DESGLOSE DETALLADO DE MULTAS POTENCIALES POR ÓRGANO REGULADOR]
#v(0.15em)

#table(
  columns: (1.15fr, 1.25fr, 2.3fr),
  fill: (x, y) => if y == 0 { rgb("#0f172a") } else if calc.even(y) { rgb("#f8fafc") } else { none },
  stroke: (x, y) => if y == 0 { (bottom: 1.5pt + rgb("#0f172a")) } else { (bottom: 0.5pt + rgb("#e2e8f0")) },
  table.header(
    [*#text(fill: white)[ÓRGANO / NORMATIVA]*],
    [*#text(fill: white)[RANGO LEGAL (UMA/MXN)]*],
    [*#text(fill: white)[HALLAZGO TÉCNICO Y FUNDAMENTO]*]
  ),
  [STPS (NOM-035/037/019/030)], [250 a 5,000 UMAs por NOM], [{{HALLAZGO_TABLA_STPS}}],
  [SAT (CFF Art. 69-B / 17-H Bis)], [\$190,000 - \$650,000 MXN], [{{HALLAZGO_TABLA_SAT}}],
  [SABG / ex-INAI (LFPDPPP)], [100 a 320,000 UMAs], [{{INAI_OBS}}],
  [IMSS / INFONAVIT (LSS / REPSE)], [500 a 2,000 UMAs], [{{HALLAZGO_TABLA_IMSS}}],
  [SECTORIAL / PROT. CIVIL], [500 a 20,000 UMAs], [{{HALLAZGO_TABLA_PC}}],
  [PROFECO (LFPC Art. 76 BIS)], [250 a 45,000 UMAs], [{{HALLAZGO_TABLA_PROFECO}}],
)

#pagebreak()

#text(size: 9.2pt, weight: "bold", fill: rgb("#0f172a"))[4. MATRIZ DE IMPACTO OPERATIVO Y MEDIDAS DE APREMIO INMEDIATAS]
#v(0.15em)
#table(
  columns: (1fr, 1.3fr, 2fr),
  fill: (x, y) => if y == 0 { rgb("#1e293b") } else if calc.even(y) { rgb("#f8fafc") } else { none },
  stroke: (x, y) => if y == 0 { (bottom: 1.5pt + rgb("#1e293b")) } else { (bottom: 0.5pt + rgb("#e2e8f0")) },
  table.header(
    [*#text(fill: white)[SEVERIDAD]*],
    [*#text(fill: white)[MEDIDA CAUTELAR]*],
    [*#text(fill: white)[CONSECUENCIA EN LA CONTINUIDAD DE NEGOCIO]*]
  ),
  [Crítica Patrimonial], [Restricción CSD (Art. 17-H Bis CFF)], [Parálisis inmediata de facturación CFDI 4.0 y congelamiento de cobros.],
  [Crítica Operativa], [Clausura Total o Parcial (PIPC/STPS)], [Colocación de sellos de suspensión por riesgo inminente o falta de PIPC.],
  [Alta Comercial], [Opinión 32-D Negativa (SAT/IMSS)], [Retención de pagos por clientes corporativos e inhabilitación en licitaciones.],
)

#v(0.35em)

#text(size: 9.2pt, weight: "bold", fill: rgb("#0f172a"))[5. ANÁLISIS COSTO-BENEFICIO: DESPACHOS TRADICIONALES VS. KIT MALLEN 360°]
#v(0.15em)
#table(
  columns: (2.1fr, 1.1fr, 1fr),
  fill: (x, y) => if y == 0 { rgb("#0f172a") } else if calc.even(y) { rgb("#f8fafc") } else { none },
  stroke: (x, y) => if y == 0 { (bottom: 1.5pt + rgb("#0f172a")) } else { (bottom: 0.5pt + rgb("#e2e8f0")) },
  table.header(
    [*#text(fill: white)[CONTRATACIÓN EN DESPACHOS POR SEPARADO]*],
    [*#text(fill: white)[HONORARIOS MERCADO]*],
    [*#text(fill: white)[EN KIT MALLEN]*]
  ),
  [Pólizas y Carpetas STPS (NOM-035, NOM-037, NOM-019 y NOM-030)], [\$25,000.00 MXN], [*INCLUIDO*],
  [Aviso de Privacidad Integral, ARCO y Consentimiento Biométrico (LFPDPPP)], [\$12,000.00 MXN], [*INCLUIDO*],
  [Contrato Marco de Materialidad B2B con Fecha Cierta y Bitácora 69-B], [\$35,000.00 MXN], [*INCLUIDO*],
  [Guía de Programa Interno de Protección Civil (PIPC) y Actas de Brigada], [\$30,000.00 MXN], [*INCLUIDO*],
  [Protocolo de Blindaje Patronal IMSS 32-D y Expediente REPSE/ICSOE], [\$15,000.00 MXN], [*INCLUIDO*],
  [Términos y Condiciones E-Commerce Art. 76 BIS LFPC y Garantías PROFECO], [\$10,000.00 MXN], [*INCLUIDO*],
  [*INVERSIÓN TOTAL EN CONSULTORÍA TRADICIONAL:*], [\$127,000.00 MXN], [*AHORRO > 95%*]
)

#v(0.35em)

#text(size: 9.2pt, weight: "bold", fill: rgb("#0f172a"))[6. TELEMETRÍA TÉCNICA DE HUELLA DIGITAL Y ESCANEO OSINT]
#v(0.15em)
#rect(width: 100%, fill: rgb("#f8fafc"), stroke: 1pt + rgb("#cbd5e1"), inset: 5pt, radius: 4pt)[
  #grid(
    columns: (1fr, 1fr, 1fr),
    [ *Cifrado TLS / SSL (Art. 19):* \ {{SSL_STATUS}} ],
    [ *Aviso Privacidad LFPDPPP:* \ {{PRIVACY_STATUS}} ],
    [ *Respuesta HTTP Dominio:* \ Código {{HTTP_STATUS}} ]
  )
  #v(0.2em)
  *Vectores Evaluados:* `STPS_39_NOMS_SCIAN`, `LFPDPPP_SABG_PRIVACY`, `SAT_CFF_69B_EFOS`, `IMSS_32D_REPSE`, `PROT_CIVIL_PIPC`, `PROFECO_76BIS_RRPAC`
]

#v(0.35em)

#text(size: 9.2pt, weight: "bold", fill: rgb("#0f172a"))[7. EXPEDIENTE DE INSTRUMENTOS JURÍDICOS Y CERTIFICADOS ENTREGADOS EN SU ZIP]
#v(0.15em)
- [X] *10 Certificados de Acreditación Normativa (PDF):* Sellados con Hash SHA-256 y raíz de Árbol de Merkle en L2.
- [X] *Kit Legal 01 (LFPDPPP / SABG):* Aviso de Privacidad Integral, Simplificado, Cláusulas ARCO y Consentimiento Biométrico.
- [X] *Kit Legal 02 (STPS):* Política NOM-035, Política Teletrabajo NOM-037, Acta Comisión Mixta NOM-019 y Diagnóstico NOM-030.
- [X] *Kit Legal 03 (SAT Art. 69-B):* Contrato Marco de Prestación de Servicios con Cláusula de Fecha Cierta y Bitácora de Entregables.
- [X] *Kit Legal 04 y 05 (IMSS / PROFECO):* Checklist de Blindaje Solidario REPSE/ICSOE y Términos Comerciales Art. 76 BIS LFPC.

#v(0.35em)

#text(size: 9.2pt, weight: "bold", fill: rgb("#0f172a"))[8. FIRMA PERICIAL Y SELLO CRIPTOGRÁFICO DE FECHA CIERTA]
#v(0.15em)
*FIRMA PERICIAL EMISORA:* Firma Pericial Mallen — GRC & Compliance Automatizado (`mallen.lat`) \
*AUDITOR DICTAMINADOR:* C.P.C. Víctor Omar Malpica Guillén (Certificación Pericial ISO 27001 / NOM-151) \
*HUELLA CRIPTOGRÁFICA SHA-256:* #raw("{{AUDIT_HASH}}")

#v(0.35em)

// OFERTA DE ACTIVACIÓN CON ANCLAJE PSICOLÓGICO DEL 0.5%
#rect(width: 100%, fill: rgb("#fef2f2"), stroke: 1.5pt + rgb("#fca5a5"), inset: 6.5pt, radius: 5pt)[
  #align(center)[
    #text(fill: rgb("#dc2626"), weight: "bold", size: 9.8pt)[ACTIVACIÓN INMEDIATA DE BLINDAJE INTEGRAL 360°: \$ {{ACTIVATION_PRICE}} MXN] \
    #v(0.1em)
    #text(fill: rgb("#991b1b"), size: 7.2pt)[*(Inversión equivalente a menos del 0.5% de la contingencia promedio en multas de \$ {{AVG_RISK}} MXN)*] \
    #v(0.1em)
    #text(fill: rgb("#0f172a"), weight: "bold", size: 7.8pt)[Bóveda de Descarga y Verificación: #raw("https://mallen.lat/checkout?ref={{REF_TAX}}")]
  ]
]

#v(0.2em)
#rect(width: 100%, fill: rgb("#f1f5f9"), inset: 4pt, radius: 2pt)[
  #text(size: 5.4pt, fill: rgb("#475569"))[*MANIFIESTO DE ALCANCE PERICIAL OSINT Y ACEPTACIÓN ELECTRÓNICA (ART. 89 CÓDIGO DE COMERCIO):* El presente diagnóstico técnico se emite a partir de registros públicos abiertos (DENUE, SAT, STPS, SFP) y telemetría web externa al instante del corte ({{FECHA_EMISION}}), sin constituir acta de visita gubernamental ni fe pública notarial. La personalización final, impresión, recabo de firmas autógrafas y ejecución material en el centro de trabajo de los Kits Legales adjuntos es responsabilidad exclusiva del Representante Legal de la empresa auditada.]
]
