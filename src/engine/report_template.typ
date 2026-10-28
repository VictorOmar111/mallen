#set page(paper: "a4", margin: (x: 1.5cm, y: 1.35cm))
#set text(size: 8.3pt)

#rect(width: 100%, fill: rgb("#0f172a"), inset: 10pt, radius: 4pt)[
  #align(center)[
    #text(fill: white, weight: "bold", size: 11.8pt)[DICTAMEN PERICIAL DE AUDITORÍA LEGAL, FISCAL Y REGULATORIA] \
    #v(0.2em)
    #text(fill: rgb("#cbd5e1"), size: 7.6pt)[{{PARTNER_HEADER}} — EVALUACIÓN INTEGRAL DE RIESGO NORMATIVO Y PROTECCIÓN PATRIMONIAL B2B]
  ]
]

#v(0.25em)

#grid(
  columns: (1.28fr, 1.32fr),
  gutter: 0.8em,
  [
    #v(0.1em)
    *EMPRESA AUDITADA:* {{EMPRESA_NOMBRE}} \
    *IDENTIFICADOR / RFC:* {{EMPRESA_TAX_ID}} \
    *REPRESENTANTE LEGAL:* {{REPRESENTANTE_LEGAL}} \
    *SECTOR / SCIAN:* {{EMPRESA_ACTIVIDAD}} \
    *DOMICILIO / PLANTILLA:* {{EMPRESA_DIRECCION}} \
    *FECHA DE CORTE OSINT:* {{FECHA_EMISION}} \
    *ESTATUS DICTAMEN:* #text(fill: rgb("{{STATUS_HEX}}"), weight: "bold")[{{ESTATUS_TEXTO}}]
  ],
  [
    #rect(width: 100%, fill: rgb("#f8fafc"), stroke: 1pt + rgb("#cbd5e1"), inset: 5pt, radius: 5pt)[
      #align(center)[
        #text(size: 7.4pt, weight: "bold", fill: rgb("#0f172a"))[ÍNDICE DE EXPOSICIÓN: {{RISK_SCORE}} / 100 PUNTOS] \
        #v(0.25em)
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
                  #text(size: 4.5pt, weight: "bold", fill: rgb("#64748b"))[PTS]
                ]
              ]
            ]
          ],
          [
            #align(left + horizon)[
              #text(size: 5.2pt)[
                #box(fill: rgb("#2563eb"), width: 4pt, height: 4pt, radius: 1pt) *STPS (39 NOMs):* {{SCORE_STPS}} pts (\$ {{MULTA_STPS}}) \
                #box(fill: rgb("#dc2626"), width: 4pt, height: 4pt, radius: 1pt) *SAT (CFF 69-B/CSD):* {{SCORE_SAT}} pts (\$ {{MULTA_SAT}}) \
                #box(fill: rgb("#d97706"), width: 4pt, height: 4pt, radius: 1pt) *SABG / INAI (LFPDPPP):* {{SCORE_INAI}} pts (\$ {{MULTA_INAI}}) \
                #box(fill: rgb("#16a34a"), width: 4pt, height: 4pt, radius: 1pt) *IMSS / INFONAVIT:* {{SCORE_IMSS}} pts (\$ {{MULTA_IMSS}}) \
                #box(fill: rgb("#ea580c"), width: 4pt, height: 4pt, radius: 1pt) *SECTORIAL / PIPC:* {{SCORE_PC}} pts (\$ {{MULTA_PC}}) \
                #box(fill: rgb("#7c3aed"), width: 4pt, height: 4pt, radius: 1pt) *PROFECO (LFPC):* {{SCORE_PROFECO}} pts (\$ {{MULTA_PROFECO}})
              ]
            ]
          ]
        )
      ]
    ]
  ]
)

#v(0.2em)

#rect(width: 100%, fill: rgb("{{BANNER_BG_HEX}}"), stroke: 1pt + rgb("{{BANNER_BORDER_HEX}}"), inset: 6pt, radius: 4pt)[
  #grid(
    columns: (1fr, auto),
    [
      #text(fill: rgb("#0f172a"), weight: "bold", size: 7.8pt)[CONTINGENCIA ECONÓMICA TOTAL SINCRONIZADA (SUMA EXACTA DE LOS 6 ÓRGANOS EN UMA 2026):] \
      #v(0.1em)
      #text(fill: rgb("{{STATUS_HEX}}"), size: 12.2pt, weight: "bold")[\$ {{ESTIMATED_FINE}} MXN]
    ],
    [
      #align(right + horizon)[
        #box(fill: rgb("{{STATUS_HEX}}"), inset: (x: 6pt, y: 3pt), radius: 3pt)[
          #text(fill: white, weight: "bold", size: 7.2pt)[{{BADGE_ACCION}}]
        ]
      ]
    ]
  )
]

#v(0.2em)
#line(length: 100%, stroke: 0.5pt + rgb("#cbd5e1"))

#text(size: 9pt, weight: "bold", fill: rgb("#0f172a"))[1. RESUMEN EJECUTIVO Y METODOLOGÍA DE EVALUACIÓN PERICIAL]
#v(0.12em)
El presente dictamen cuantifica de forma determinística la exposición legal, fiscal y administrativa de *"{{EMPRESA_NOMBRE}}"* ({{EMPRESA_TAX_ID}}), representada por *{{REPRESENTANTE_LEGAL}}*. Los montos aquí expresados están sincronizados 1:1 con el motor pericial en línea (`mallen.lat/verify/{{REF_TAX}}`).

#v(0.25em)

#text(size: 9pt, weight: "bold", fill: rgb("#0f172a"))[2. MARCO NORMATIVO AUDITADO (360° COMPLIANCE MEXICANO)]
#v(0.12em)
- *STPS (39 NOMs):* LFT Arts. 132, 330-A a 330-K, 509, 540 y 994 (Seguridad, Salud, NOM-019, NOM-030, NOM-035, NOM-036-1 y NOM-037).
- *SAT (CFF / LISR / LIVA):* Art. 69-B (Materialidad B2B y Fecha Cierta SCJN), Art. 17-K (Buzón), Art. 17-H Bis (CSD), Art. 28 y Art. 29-A.
- *PROTECCIÓN DE DATOS (LFPDPPP / SABG ex-INAI):* Arts. 8, 9, 15, 16, 17, 19, 22-37 y 63-64 (Avisos, ARCO, Biométricos y SSL).
- *IMSS e INFONAVIT:* LSS Arts. 15, 15-A (REPSE / ICSOE / SISSUB), 39 (Opinión 32-D), 71-75 (Prima de Riesgo) y 304-A.
- *SECTORIAL Y PROTECCIÓN CIVIL:* LGPC (PIPC >100 m²), COFEPRIS, SEMARNAT, CNBV/LFPIORPI, SICT, SEP y SECTUR según código SCIAN.
- *PROFECO:* LFPC Arts. 32, 76 BIS (Comercio Digital), 85-87 (Contratos de Adhesión RRPAC) y 128 BIS.

#v(0.25em)

#text(size: 9pt, weight: "bold", fill: rgb("#0f172a"))[3. DESGLOSE MATEMÁTICO EXACTO DE CONTINGENCIA POR ÓRGANO REGULADOR]
#v(0.12em)

#table(
  columns: (1.15fr, 1.15fr, 2.4fr),
  fill: (x, y) => if y == 0 { rgb("#0f172a") } else if calc.even(y) { rgb("#f8fafc") } else { none },
  stroke: (x, y) => if y == 0 { (bottom: 1.5pt + rgb("#0f172a")) } else { (bottom: 0.5pt + rgb("#e2e8f0")) },
  table.header(
    [*#text(fill: white)[ÓRGANO / NORMATIVA]*],
    [*#text(fill: white)[CONTINGENCIA ESTIMADA]*],
    [*#text(fill: white)[HALLAZGO TÉCNICO Y FUNDAMENTO]*]
  ),
  [STPS (39 NOMs Aplicables)], [\$ {{MULTA_STPS}} MXN], [{{HALLAZGO_TABLA_STPS}}],
  [SAT (CFF Art. 69-B / CSD)], [\$ {{MULTA_SAT}} MXN], [{{HALLAZGO_TABLA_SAT}}],
  [SABG / ex-INAI (LFPDPPP)], [\$ {{MULTA_INAI}} MXN], [{{INAI_OBS}}],
  [IMSS / INFONAVIT (LSS / REPSE)], [\$ {{MULTA_IMSS}} MXN], [{{HALLAZGO_TABLA_IMSS}}],
  [SECTORIAL / PROT. CIVIL], [\$ {{MULTA_PC}} MXN], [{{HALLAZGO_TABLA_PC}}],
  [PROFECO (LFPC Art. 76 BIS)], [\$ {{MULTA_PROFECO}} MXN], [{{HALLAZGO_TABLA_PROFECO}}],
  [*TOTAL ACUMULADO AUDITADO:*], [*\$ {{ESTIMATED_FINE}} MXN*], [*Suma exacta de los 6 vectores regulatorios evaluados en UMA 2026.*]
)

#pagebreak()

#text(size: 9pt, weight: "bold", fill: rgb("#0f172a"))[4. MATRIZ DE IMPACTO OPERATIVO Y MEDIDAS DE APREMIO INMEDIATAS]
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

#v(0.3em)

#text(size: 9pt, weight: "bold", fill: rgb("#0f172a"))[5. ANÁLISIS COSTO-BENEFICIO: DESPACHOS TRADICIONALES VS. KIT MALLEN 360°]
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

#v(0.3em)

#text(size: 9pt, weight: "bold", fill: rgb("#0f172a"))[6. TELEMETRÍA TÉCNICA DE HUELLA DIGITAL Y ESCANEO OSINT]
#v(0.15em)
#rect(width: 100%, fill: rgb("#f8fafc"), stroke: 1pt + rgb("#cbd5e1"), inset: 5pt, radius: 4pt)[
  #grid(
    columns: (1fr, 1fr, 1fr),
    [ *Cifrado TLS / SSL (Art. 19):* \ {{SSL_STATUS}} ],
    [ *Aviso Privacidad LFPDPPP:* \ {{PRIVACY_STATUS}} ],
    [ *Respuesta HTTP Dominio:* \ Código {{HTTP_STATUS}} ]
  )
  #v(0.2em)
  *Jurisdicción Aplicada:* {{EMPRESA_JURISDICCION}} | *Oficial de Privacidad:* {{OFICIAL_PRIVACIDAD}} | *Coordinador NOM-019:* {{COORDINADOR_CSH}}
]

#v(0.3em)

#text(size: 9pt, weight: "bold", fill: rgb("#0f172a"))[7. EXPEDIENTE DE INSTRUMENTOS PERSONALIZADOS ENTREGADOS EN SU ZIP]
#v(0.15em)
- [X] *10 Certificados de Acreditación Normativa (PDF):* Personalizados con Razón Social, Giro SCIAN, Jurisdicción y Representante Legal.
- [X] *7 Instrumentos Jurídicos en PDF y Word-HTML (Formatos Ejecutivos PDF y Word):* Aviso Integral LFPDPPP, Política NOM-035, Acta Comisión Mixta NOM-019/030, Política Teletrabajo NOM-037, Contrato Materialidad Art. 69-B con Fecha Cierta, Protocolo IMSS REPSE y Términos E-Commerce Art. 76 BIS LFPC.

#v(0.3em)

#text(size: 9pt, weight: "bold", fill: rgb("#0f172a"))[8. FIRMA PERICIAL Y SELLO CRIPTOGRÁFICO DE FECHA CIERTA]
#v(0.15em)
*FIRMA PERICIAL EMISORA:* Firma Pericial Mallen — GRC & Compliance Automatizado (`mallen.lat`) \
*AUDITOR DICTAMINADOR:* {{AUDITOR_NOMBRE}} (Cédula: {{AUDITOR_CEDULA}} | Reg. STPS DC-5: {{AUDITOR_DC5}}) \
*HUELLA CRIPTOGRÁFICA SHA-256:* #raw("{{AUDIT_HASH}}")

#v(0.3em)

#rect(width: 100%, fill: rgb("{{BANNER_BG_HEX}}"), stroke: 1.5pt + rgb("{{BANNER_BORDER_HEX}}"), inset: 6.5pt, radius: 5pt)[
  #align(center)[
    #text(fill: rgb("{{STATUS_HEX}}"), weight: "bold", size: 9.6pt)[ACTIVACIÓN DE BLINDAJE INTEGRAL 360° + CFDI 4.0: \$ {{ACTIVATION_PRICE}} MXN] \
    #v(0.1em)
    #text(fill: rgb("#0f172a"), weight: "bold", size: 7.8pt)[Verificación y Descarga: #raw("https://mallen.lat/verify/{{REF_TAX}}")]
  ]
]
