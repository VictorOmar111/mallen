#set page(paper: "a4", margin: (x: 1.5cm, y: 1.5cm))
#set text(size: 8.5pt)

// ENCABEZADO CORPORATIVO PERICIAL
#rect(width: 100%, fill: rgb("#0f172a"), inset: 10pt, radius: 4pt)[
  #align(center)[
    #text(fill: white, weight: "bold", size: 12.5pt)[DICTAMEN PERICIAL DE AUDITORÍA LEGAL, FISCAL Y REGULATORIA] \
    #v(0.2em)
    #text(fill: rgb("#cbd5e1"), size: 8pt)[EVALUACIÓN INTEGRAL DE RIESGO NORMATIVO Y PROTECCIÓN PATRIMONIAL B2B]
  ]
]

#v(0.3em)

// DASHBOARD PERICIAL: FICHA TÉCNICA + DONA MULTICOLOR 6 SECTORES
#grid(
  columns: (1.2fr, 1.4fr),
  gutter: 0.8em,
  [
    #v(0.1em)
    *EMPRESA AUDITADA:* {{EMPRESA_NOMBRE}} \
    *IDENTIFICADOR / TAX ID:* {{EMPRESA_TAX_ID}} \
    *SECTOR / ACTIVIDAD:* {{EMPRESA_ACTIVIDAD}} \
    *UBICACIÓN / CONTACTO:* {{EMPRESA_DIRECCION}} \
    *FECHA DE DICTAMEN:* 05 de Octubre, 2026 \
    *ESTATUS AUDITORÍA:* #text(fill: rgb("#dc2626"), weight: "bold")[RIESGO CRÍTICO DETECTADO]
  ],
  [
    #rect(width: 100%, fill: rgb("#f8fafc"), stroke: 1pt + rgb("#cbd5e1"), inset: 5pt, radius: 5pt)[
      #align(center)[
        #text(size: 7.5pt, weight: "bold", fill: rgb("#0f172a"))[SCORE DE RIESGO: {{RISK_SCORE}} PUNTOS (ACUMULADO)] \
        #v(0.3em)
        #grid(
          columns: (auto, 1fr),
          gutter: 0.5em,
          [
            #box(width: 52pt, height: 52pt)[
              #place(top + left, circle(radius: 26pt, fill: rgb("#e2e8f0")))
              #place(top + left, circle(radius: 26pt, fill: rgb("#2563eb"))) // STPS (25%)
              #place(top + left, dx: 2pt, dy: 1pt, circle(radius: 24.5pt, fill: rgb("#dc2626"))) // SAT (20%)
              #place(top + left, dx: 4pt, dy: 2pt, circle(radius: 23pt, fill: rgb("#d97706"))) // INAI (20%)
              #place(top + left, dx: 6pt, dy: 3pt, circle(radius: 21pt, fill: rgb("#16a34a"))) // IMSS (15%)
              #place(top + left, dx: 8pt, dy: 4pt, circle(radius: 19pt, fill: rgb("#ea580c"))) // PC (10%)
              #place(top + left, dx: 10pt, dy: 5pt, circle(radius: 17pt, fill: rgb("#7c3aed"))) // PROFECO (10%)
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
                #box(fill: rgb("#2563eb"), width: 4pt, height: 4pt, radius: 1pt) *STPS:* {{SCORE_STPS}} pts (Normativa Laboral) \
                #box(fill: rgb("#dc2626"), width: 4pt, height: 4pt, radius: 1pt) *SAT:* {{SCORE_SAT}} pts (Fiscal/Materialidad) \
                #box(fill: rgb("#d97706"), width: 4pt, height: 4pt, radius: 1pt) *INAI:* {{SCORE_INAI}} pts (Privacidad Datos) \
                #box(fill: rgb("#16a34a"), width: 4pt, height: 4pt, radius: 1pt) *IMSS:* {{SCORE_IMSS}} pts (Opinión 32-D) \
                #box(fill: rgb("#ea580c"), width: 4pt, height: 4pt, radius: 1pt) *PROT. CIVIL:* {{SCORE_PC}} pts (Riesgo Operativo) \
                #box(fill: rgb("#7c3aed"), width: 4pt, height: 4pt, radius: 1pt) *PROFECO:* {{SCORE_PROFECO}} pts (Comercial)
              ]
            ]
          ]
        )
      ]
    ]
  ]
)

#v(0.3em)

// BANNER DE EXPOSICIÓN FINANCIERA
#rect(width: 100%, fill: rgb("#fef2f2"), stroke: 1pt + rgb("#fca5a5"), inset: 6pt, radius: 4pt)[
  #grid(
    columns: (1fr, auto),
    [
      #text(fill: rgb("#0f172a"), weight: "bold", size: 8pt)[EXPOSICIÓN TOTAL ESTIMADA A SANCIONES ACUMULADAS:] \
      #v(0.1em)
      #text(fill: rgb("#dc2626"), size: 12.5pt, weight: "bold")[\$ {{ESTIMATED_FINE}} MXN]
    ],
    [
      #align(right + horizon)[
        #box(fill: rgb("#dc2626"), inset: (x: 6pt, y: 3pt), radius: 3pt)[
          #text(fill: white, weight: "bold", size: 7.5pt)[CRÍTICO: SANCIONABLE]
        ]
      ]
    ]
  )
]

#v(0.3em)
#line(length: 100%, stroke: 0.5pt + rgb("#cbd5e1"))

#text(size: 9.5pt, weight: "bold", fill: rgb("#0f172a"))[1. RESUMEN EJECUTIVO Y METODOLOGÍA DE EVALUACIÓN]
#v(0.2em)
El presente dictamen analiza el nivel de exposición legal, fiscal y administrativa de la entidad *"{{EMPRESA_NOMBRE}}"*. La puntuación acumulada de *{{RISK_SCORE}} Puntos* se determina ponderando 3 vectores: *Sanción Económica (\$MXN)*, *Probabilidad de Inspección por Rastreo OSINT* y *Riesgo de Medidas Cautelares Operativas*.

#v(0.4em)

#text(size: 9.5pt, weight: "bold", fill: rgb("#0f172a"))[2. MARCO NORMATIVO Y FUNDAMENTOS DE AUDITORÍA DIRECTA]
#v(0.2em)
La auditoría evalúa el grado de cumplimiento de las facultades de inspección vigentes:
- *STPS:* LFT Arts. 540 y 1002 (NOM-035, NOM-037, Comisiones Mixtas CSH).
- *SAT:* CFF Art. 69-B (Soporte documental de materialidad de operaciones B2B).
- *INAI:* Ley LFPDPPP Arts. 60-64 (Auditoría de Avisos de Privacidad y transferencia de datos).
- *IMSS e INFONAVIT:* LSS Arts. 15 y 304 (Opinión de Cumplimiento 32-D, Primas de Riesgo y Subcontratación REPSE).
- *PROTECCIÓN CIVIL:* Ley General de Protección Civil (Programa Interno PIPC y Dictamen de Seguridad Operativa).
- *PROFECO:* LFPC Art. 76 BIS (Términos comerciales e información en plataformas digitales).

#v(0.4em)

#text(size: 9.5pt, weight: "bold", fill: rgb("#0f172a"))[3. DESGLOSE DETALLADO DE MULTAS POTENCIALES POR INSTITUCIÓN]
#v(0.2em)

#table(
  columns: (1.1fr, 1.3fr, 2.3fr),
  fill: (x, y) => if y == 0 { rgb("#0f172a") } else if calc.even(y) { rgb("#f8fafc") } else { none },
  stroke: (x, y) => if y == 0 { (bottom: 1.5pt + rgb("#0f172a")) } else { (bottom: 0.5pt + rgb("#e2e8f0")) },
  table.header(
    [*#text(fill: white)[INSTITUCIÓN / NORMATIVA]*],
    [*#text(fill: white)[RANGO DE MULTA]*],
    [*#text(fill: white)[ORIGEN Y JUSTIFICACIÓN DEL PUNTAJE]*]
  ),
  [STPS (NOM-035/037/019)], [\$162,855 - \$434,280 MXN], [12 pts: Omisión de Comisiones Mixtas y políticas psicosociales.],
  [SAT (CFF Art. 69-B)], [\$200,000 - \$500,000 MXN], [10 pts: Falta de entregables y expediente de materialidad.],
  [INAI (Ley LFPDPPP)], [\$21,714 - \$347,424 MXN], [10 pts: {{INAI_OBS}}. Deficiencia en tratamiento de datos.],
  [IMSS (LSS / Opinión 32-D)], [\$30,000 - \$180,000 MXN], [7 pts: Inconsistencias registradas y riesgo de opinión negativa.],
  [PROTECCIÓN CIVIL], [\$50,000 - \$250,000 MXN], [5 pts: Ausencia de Programa Interno PIPC y riesgo de clausura.],
  [PROFECO (LFPC Art. 76 BIS)], [\$50,000 - \$150,000 MXN], [5 pts: Términos comerciales y contratos de adhesión no visibles.],
)

#pagebreak()

#text(size: 9.5pt, weight: "bold", fill: rgb("#0f172a"))[4. MATRIZ DE IMPACTO OPERATIVO Y SANCIONES ACCESORIAS]
#v(0.2em)
#table(
  columns: (1fr, 1.2fr, 2fr),
  fill: (x, y) => if y == 0 { rgb("#1e293b") } else if calc.even(y) { rgb("#f8fafc") } else { none },
  stroke: (x, y) => if y == 0 { (bottom: 1.5pt + rgb("#1e293b")) } else { (bottom: 0.5pt + rgb("#e2e8f0")) },
  table.header(
    [*#text(fill: white)[GRAVEDAD]*],
    [*#text(fill: white)[MEDIDA CAUTELAR]*],
    [*#text(fill: white)[IMPACTO EN LA OPERACIÓN B2B]*]
  ),
  [Riesgo Crítico], [Cancelación CSD (SAT)], [Incapacidad inmediata para facturar a clientes comerciales.],
  [Riesgo Alto], [Clausura Física (Prot. Civil)], [Colocación de sellos y suspensión inmediata del centro de trabajo.],
  [Riesgo Moderado], [Opinión 32-D Negativa (IMSS)], [Bloqueo de cobros y retención de pagos por clientes B2B.],
)

#v(0.4em)

#text(size: 9.5pt, weight: "bold", fill: rgb("#0f172a"))[5. COMPARATIVA DE COSTOS: SERVICIOS INDEPENDIENTES VS. BLINDAJE B2B]
#v(0.2em)
#table(
  columns: (2fr, 1.2fr, 1fr),
  fill: (x, y) => if y == 0 { rgb("#0f172a") } else if calc.even(y) { rgb("#f8fafc") } else { none },
  stroke: (x, y) => if y == 0 { (bottom: 1.5pt + rgb("#0f172a")) } else { (bottom: 0.5pt + rgb("#e2e8f0")) },
  table.header(
    [*#text(fill: white)[CONTRATACIÓN DE SERVICIOS POR SEPARADO]*],
    [*#text(fill: white)[COSTO MERCADO]*],
    [*#text(fill: white)[INCLUIDO]*]
  ),
  [Implementación y Carpetas NOM-035 / NOM-037 (Despacho Laboral)], [\$25,000.00 MXN], [SÍ],
  [Aviso de Privacidad e Implementación LFPDPPP (Consultor INAI)], [\$12,000.00 MXN], [SÍ],
  [Auditoría de Materialidad Fiscal CFF 69-B (Despacho Fiscal)], [\$35,000.00 MXN], [SÍ],
  [Programa Interno y Carpeta de Protección Civil (Tercero Acreditado)], [\$30,000.00 MXN], [SÍ],
  [Expediente Patrón-IMSS y Opinión 32-D (Gestoría Social)], [\$15,000.00 MXN], [SÍ],
  [Auditoría de Términos Comerciales LFPC (Abogado Comercial)], [\$10,000.00 MXN], [SÍ],
  [*COSTO TOTAL ESTIMADO CON DESPACHOS INDEPENDIENTES:*], [\$127,000.00 MXN], [*AHORRO 95%*]
)

#v(0.4em)

#text(size: 9.5pt, weight: "bold", fill: rgb("#0f172a"))[6. DIAGNÓSTICO DE HUELLA DIGITAL Y AUDITORÍA OSINT DETECTADA]
#v(0.2em)
#rect(width: 100%, fill: rgb("#f8fafc"), stroke: 1pt + rgb("#cbd5e1"), inset: 5pt, radius: 4pt)[
  #grid(
    columns: (1fr, 1fr, 1fr),
    [ *Certificado SSL:* \ {{SSL_STATUS}} ],
    [ *Aviso Privacidad:* \ {{PRIVACY_STATUS}} ],
    [ *Estatus Servidor:* \ Código {{HTTP_STATUS}} ]
  )
  #v(0.2em)
  *Banderas OSINT:* `STPS_NOM_035`, `INAI_LFPDPPP_PRIVACY`, `SAT_CFF_69B_CHECK`, `IMSS_32D_MONITOR`, `PROT_CIVIL_PIPC`, `PROFECO_76BIS`
]

#v(0.4em)

#text(size: 9.5pt, weight: "bold", fill: rgb("#0f172a"))[7. KITS Y CERTIFICADOS INCLUIDOS EN ESTE PAQUETE DE ACTIVACIÓN]
#v(0.2em)
- [X] *Certificado y Kit 01:* Política de Prevención NOM-035 y Protocolo NOM-037 (STPS).
- [X] *Certificado y Kit 02:* Aviso de Privacidad Integral Ajustado con Cláusulas ARCO (INAI).
- [X] *Certificado y Kit 03:* Expediente de Soporte y Materialidad Fiscal B2B (SAT Art. 69-B).
- [X] *Certificado y Kit 04:* Carpeta de Seguridad Operativa y Programa Interno PIPC (Protección Civil).
- [X] *Certificado y Kit 05:* Protocolo de Cumplimiento Patrón-IMSS y Expediente REPSE.

#v(0.4em)

#text(size: 9.5pt, weight: "bold", fill: rgb("#0f172a"))[8. FIRMA E INSTITUCIONALIDAD PERICIAL CORPORATIVA]
#v(0.2em)
*EMISOR RESPONSABLE:* Inmunidad B2B Consulting S.A.S. de C.V. \
*AUDITOR DESIGNADO:* C.P.C. Víctor Omar Malpica Guillen (Reg. CED-2026-B2B) \
*SELLO DIGITAL CORPORATIVO (SHA-256):* raw("{{AUDIT_HASH}}")

#v(0.5em)

// OFERTA DE ACTIVACIÓN CON CÁLCULO DEL 0.5% DEL RIESGO PROMEDIO (6 SECTORES)
#rect(width: 100%, fill: rgb("#fef2f2"), stroke: 1.5pt + rgb("#fca5a5"), inset: 7pt, radius: 5pt)[
  #align(center)[
    #text(fill: rgb("#dc2626"), weight: "bold", size: 10pt)[OFERTA DE ACTIVACIÓN DE BLINDAJE INTEGRAL: \$ {{ACTIVATION_PRICE}} MXN] \
    #v(0.1em)
    #text(fill: rgb("#991b1b"), size: 7.5pt)[*(Cuota fija correspondiente al 0.5% de la exposición promedio calculada: \$ {{AVG_RISK}} MXN)*] \
    #v(0.1em)
    #text(fill: rgb("#0f172a"), weight: "bold", size: 8pt)[Liga de Activación Inmediata: raw("https://inmunidadb2b.com/checkout?ref={{REF_TAX}}")]
  ]
]

#v(0.2em)
#align(center)[
  #text(size: 6pt, fill: rgb("#64748b"))[*CLÁUSULA DE CADUCIDAD:* Este dictamen pericial tiene una validez de 30 días naturales a partir de su emisión. La garantía inmunológica entra en vigor al activar el kit de blindaje.]
]
