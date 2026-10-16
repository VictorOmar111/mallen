#set page(paper: "a4", margin: (x: 1.8cm, top: 2cm, bottom: 2.2cm), header: align(right)[#text(size: 8pt, fill: rgb("#64748b"))[FIRMA PERICIAL MALLEN | ACREDITACIÓN TÉCNICA Y NORMATIVA B2B]], footer: [#line(length: 100%, stroke: 0.5pt + rgb("#cbd5e1")) #grid(columns: (1fr, 1fr), align(left)[#text(size: 7.5pt, fill: rgb("#64748b"))[Folio Registral: #text(weight: "bold", fill: rgb("#0f172a"))[CERT-STPS-035]]], align(right)[#text(size: 7.5pt, fill: rgb("#64748b"))[Constancia NOM-151-SCFI / Merkle Validated]])])
#set text(size: 9.5pt, fill: rgb("#0f172a"))

#block(fill: rgb("#0b0f19"), inset: 16pt, radius: 6pt, width: 100%, [#grid(columns: (1fr, auto), align: horizon, [#text(size: 14pt, weight: "bold", fill: rgb("#ffffff"))[CERTIFICADO DE ACREDITACIÓN TÉCNICA] #v(4pt) #text(size: 10pt, fill: rgb("#38bdf8"), weight: "bold")[NOM-035 — Factores de Riesgo Psicosocial]], [#rect(fill: rgb("#1e293b"), inset: 8pt, radius: 4pt)[#text(size: 8.5pt, fill: rgb("#34d399"), weight: "bold")[ESTATUS: ACREDITADO]]])])

#v(10pt)
#rect(width: 100%, fill: rgb("#f8fafc"), stroke: 1pt + rgb("#e2e8f0"), inset: 12pt, radius: 4pt)[#grid(columns: (1fr, 1fr), row-gutter: 8pt, [*Entidad Auditada:* Empresa Demo], [*RFC / Tax ID:* TEST1234], [*Giro / Sector:* Servicios], [*Jurisdiccion:* CDMX], [*Auditor Cargo:* C.P.C. Reg. #948201], [*Hash de Inalterabilidad:* SHA256-W16-CONFIRMED])]

#v(10pt)
== 1. Marco Juridico, Alcance & Fundamentacion Pericial
El Consejo Pericial de Firma Pericial Mallen hace constar que la entidad *Empresa Demo* ha sido sometida a los procedimientos de auditoria cuantitativa, revision documental y peritaje de campo.

#v(10pt)
== 2. Matriz Parametrica de Inmunidad & Defendabilidad
#table(columns: (2.2fr, 1fr, 1fr), fill: (x, y) => if y == 0 { rgb("#1e293b") } else if calc.even(y) { rgb("#f8fafc") } else { rgb("#ffffff") }, stroke: 0.5pt + rgb("#cbd5e1"), align: (col, row) => if row == 0 { center } else { left }, [#text(fill: white, weight: "bold")[Concepto Evaluado]], [#text(fill: white, weight: "bold")[Resultado]], [#text(fill: white, weight: "bold")[Nivel de Riesgo]], [Auditoria Documental & Papeles de Trabajo], [SATISFACTORIO], [Inmune (0% Riesgo)], [Sustancia Operativa e Infraestructura], [VERIFICADO], [Cumplimiento Alto], [Cadena de Inalterabilidad Criptografica], [SELLADO], [Validez Plena])

#pagebreak()

#block(fill: rgb("#1e293b"), inset: 12pt, radius: 4pt, width: 100%, [#text(size: 12pt, weight: "bold", fill: rgb("#ffffff"))[CÉDULA DE AUDITORÍA PERICIAL Y EVACUACIÓN DE PRUEBAS]])

#v(10pt)
== 3. Detalle de Papeles de Trabajo Auditados
#table(columns: (1.5fr, 2fr, 1fr), fill: (x, y) => if y == 0 { rgb("#0b0f19") } else if calc.even(y) { rgb("#f8fafc") } else { rgb("#ffffff") }, stroke: 0.5pt + rgb("#cbd5e1"), align: (col, row) => if row == 0 { center } else { left }, [#text(fill: white, weight: "bold")[Codigo Elemento]], [#text(fill: white, weight: "bold")[Estructura de Evidencia Auditada]], [#text(fill: white, weight: "bold")[Estado]], [PT-01], [Polizas, Contratos y Anexos de Servicio], [VALIDADO], [PT-02], [Formatos Oficiales y Registros de Control], [VERIFICADO])

#v(25pt)
#grid(columns: (1fr, 1fr), align: center, [#line(length: 70%, stroke: 0.8pt + rgb("#0f172a")) #v(2pt) #text(weight: "bold", size: 9pt)[FIRMA PERICIAL COLEGIADA]\n #text(size: 8pt, fill: rgb("#64748b"))[Firma Pericial Mallen B2B]\n #text(size: 7.5pt, fill: rgb("#94a3b8"))[Cédula Profesional Reg. #948201], [#line(length: 70%, stroke: 0.8pt + rgb("#0f172a")) #v(2pt) #text(weight: "bold", size: 9pt)[AUDITOR RESPONSABLE DE SECTOR]\n #text(size: 8pt, fill: rgb("#64748b"))[Consejo de Dictaminacion W16]\n #text(size: 7.5pt, fill: rgb("#94a3b8"))[Certificacion C.P.C. #104829]])
