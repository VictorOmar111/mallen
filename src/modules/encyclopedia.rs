use axum::response::Html;

pub struct NormaEducativa {
    pub codigo: &'static str,
    pub titulo: &'static str,
    pub incumplimiento_pct: u8,
    pub fuente_estadistica: &'static str,
    pub rango_multa: &'static str,
    pub historia_dof: &'static str,
    pub de_que_trata: &'static str,
    pub pasos_cumplimiento: &'static [&'static str],
    pub entregable_kit: &'static str,
}

pub fn render_norma_category(category: &str) -> Html<String> {
    let (siglas, titulo, subtitulo, resumen_ejecutivo, color_tw, border_top, normas): (
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
        Vec<NormaEducativa>,
    ) = match category {
        "stps" => (
            "STPS • LEY FEDERAL DEL TRABAJO Y LAS 39 NORMAS OFICIALES MEXICANAS",
            "Secretaría del Trabajo y Previsión Social (Catálogo Completo de las 39 NOMs)",
            "Historia en el DOF, porcentaje real de centros sancionados en México y guía paso a paso para cada categoría de NOMs de la STPS.",
            "El marco inspectivo laboral mexicano emana del Artículo 123 Constitucional de 1917, la Ley Federal del Trabajo de 1970 y el Reglamento Federal de Seguridad y Salud en el Trabajo (DOF 13-nov-2014). Se estructura en 5 familias de Normas Oficiales Mexicanas: Seguridad (12 NOMs), Salud y Agentes (8 NOMs), Organización (6 NOMs), Factores Psicosociales/Ergonomía/Teletrabajo (3 NOMs) y Específicas de Sector (7 NOMs + Ferroviario/Agrícola). Conforme al Artículo 992 de la LFT, las multas de 50 a 5,000 UMAs pueden multiplicarse por cada trabajador afectado.",
            "blue",
            "border-blue-600",
            vec![
                NormaEducativa {
                    codigo: "NOM-035-STPS-2018 • Art. 132 Fracc. XVI y Art. 994 Fracc. V LFT",
                    titulo: "Factores de Riesgo Psicosocial en el Trabajo — Identificación, Análisis y Prevención",
                    incumplimiento_pct: 71,
                    fuente_estadistica: "71% de los centros de trabajo inspeccionados presentan omisiones en la aplicación de las Guías de Referencia I, II o III, o carecen de evidencia firmada de su Política de Prevención Psicosocial.",
                    rango_multa: "250 a 5,000 UMAs ($28,285.00 a $565,700.00 MXN)",
                    historia_dof: "Publicada en el Diario Oficial de la Federación (DOF) el 23 de octubre de 2018. Surgió tras la reforma al RFSST de 2014 para atender el estrés laboral crónico y la violencia organizacional. Entró en vigor en dos etapas: el 23 de octubre de 2019 (política escrita, medidas de prevención y Guía I) y el 23 de octubre de 2020 (evaluación psicosocial Guía II y Entorno Organizacional Guía III).",
                    de_que_trata: "Obliga a todos los patrones en México a identificar, analizar y prevenir factores de riesgo psicosocial (cargas excesivas, jornadas superiores a la LFT, interferencia trabajo-familia, liderazgo negativo y violencia laboral), escalando las obligaciones para centros de 1 a 15, de 16 a 50 y de más de 50 trabajadores.",
                    pasos_cumplimiento: &[
                        "Emitir, firmar y difundir físicamente la Política de Prevención de Riesgos Psicosociales (Guía de Referencia IV).",
                        "Aplicar al 100% de la plantilla la Guía de Referencia I para identificar trabajadores que sufrieron acontecimientos traumáticos severos y canalizarlos a atención clínica.",
                        "Aplicar cada 2 años la Guía de Referencia II (de 16 a 50 empleados) o la Guía de Referencia III (más de 50 empleados) y resguardar los cuestionarios firmados.",
                        "Mantener activo un buzón o mecanismo confidencial de quejas por violencia laboral con bitácora de actuación."
                    ],
                    entregable_kit: "02_Politica_Prevencion_Psicosocial_NOM035_STPS.pdf y Certificado NOM-035",
                },
                NormaEducativa {
                    codigo: "NOM-037-STPS-2023 • Artículos 330-A al 330-K de la LFT",
                    titulo: "Teletrabajo — Condiciones de Seguridad y Salud en el Trabajo Remoto",
                    incumplimiento_pct: 78,
                    fuente_estadistica: "78% de las empresas con esquemas de Home Office o trabajo híbrido (>40% del tiempo fuera del local) carecen del adendum contractual, de la lista de verificación ergonómica y del pago proporcional de internet y electricidad.",
                    rango_multa: "250 a 5,000 UMAs ($28,285.00 a $565,700.00 MXN)",
                    historia_dof: "Nació con la reforma al Capítulo XII Bis de la LFT publicada en el DOF el 11 de enero de 2021. La norma técnica definitiva NOM-037-STPS-2023 se publicó en el DOF el 8 de junio de 2023 y cobró vigencia obligatoria el 5 de diciembre de 2023.",
                    de_que_trata: "Regula las condiciones ergonómicas, psicosociales y de seguridad eléctrica para empleados que laboren más del 40% de su jornada en un domicilio distinto al centro de trabajo utilizando tecnologías de la información.",
                    pasos_cumplimiento: &[
                        "Integrar el listado de teletrabajadores y firmar el adendum contractual que estipule los insumos entregados (silla ergonómica, cómputo) y el pago mensual proporcional de electricidad e internet (Art. 330-E Fracc. III LFT).",
                        "Publicar la Política de Teletrabajo reconociendo expresamente el Derecho a la Desconexión Digital fuera de la jornada laboral.",
                        "Aplicar la Lista de Verificación de Condiciones de Seguridad y Salud en el espacio de trabajo remoto validada por la Comisión Mixta."
                    ],
                    entregable_kit: "04_Politica_Teletrabajo_y_Desconexion_NOM037_STPS.pdf y Certificado NOM-037",
                },
                NormaEducativa {
                    codigo: "NOM-036-1-STPS-2018 • Ergonomía y Manejo Manual de Cargas",
                    titulo: "Factores de Riesgo Ergonómico en el Trabajo — Manejo Manual de Cargas",
                    incumplimiento_pct: 68,
                    fuente_estadistica: "68% de los comercios, restaurantes, almacenes y plantas donde el personal levanta, empuja o transporta objetos mayores a 3 kg de forma cotidiana carecen del análisis ergonómico y de vigilancia médica.",
                    rango_multa: "250 a 5,000 UMAs ($28,285.00 a $565,700.00 MXN)",
                    historia_dof: "Publicada en el DOF el 23 de noviembre de 2018 (en vigor desde el 2 de enero de 2020 y con límites de masa máxima ajustados en 2023/2024) para reducir los trastornos musculoesqueléticos lumbares, que representan la primera causa de enfermedad de trabajo en el IMSS.",
                    de_que_trata: "Aplica a todos los centros de trabajo donde existan trabajadores que realicen actividades de levantamiento, bajada, empuje, tracción o transporte manual de cargas superiores a 3 kg de manera cotidiana (más de una vez al día).",
                    pasos_cumplimiento: &[
                        "Respetar las cargas máximas manuales permitidas (25 kg para hombres adultos, 20 kg o 15 kg bajo criterios preventivos para mujeres, 7 kg para menores y prohibición para embarazadas).",
                        "Elaborar el Análisis de los Factores de Riesgo Ergonómico utilizando las tablas de evaluación rápida de los Apéndices I y II de la norma.",
                        "Proporcionar ayudas mecánicas (diablitos, patines hidráulicos, carretillas) y capacitar al personal en higiene de columna con constancia DC-3."
                    ],
                    entregable_kit: "Diagnóstico Ergonómico NOM-036-1 en Dictamen Pericial Ejecutivo",
                },
                NormaEducativa {
                    codigo: "NOM-019-STPS-2011 y NOM-030-STPS-2009 • Organización de la Seguridad",
                    titulo: "Comisiones Mixtas de Seguridad e Higiene (NOM-019) y Servicios Preventivos SST (NOM-030)",
                    incumplimiento_pct: 62,
                    fuente_estadistica: "62% de los centros de trabajo reciben emplazamientos por no exhibir el Acta Constitutiva de la Comisión Mixta con Registro Patronal IMSS, las 4 actas de recorridos trimestrales o el Diagnóstico Integral NOM-030.",
                    rango_multa: "250 a 5,000 UMAs por cada norma ($28,285.00 a $565,700.00 MXN)",
                    historia_dof: "La NOM-030-STPS-2009 se publicó en el DOF el 22 de diciembre de 2009 y la NOM-019-STPS-2011 el 13 de abril de 2011, constituyendo el pilar organizacional obligatorio desde el primer trabajador contratado.",
                    de_que_trata: "La NOM-019 exige constituir una Comisión bipartita (Coordinador, Secretario y Vocales) que realiza recorridos trimestrales e investiga accidentes; la NOM-030 exige designar formalmente a un Responsable de Seguridad y Salud en el Trabajo (SST) y elaborar el Diagnóstico y Programa Anual.",
                    pasos_cumplimiento: &[
                        "Firmar el Acta Constitutiva de la Comisión Mixta con todos los requisitos del Numeral 7.4 (Razón Social, RFC, Registro Patronal IMSS, domicilio, giro SCIAN, plantilla y turnos).",
                        "Designar por escrito al Responsable SST conforme al Numeral 4.1 de la NOM-030 y actualizar anualmente el Programa o Relación de Acciones Preventivas.",
                        "Levantar y archivar cada 3 meses el Acta de Recorrido de Verificación de la Comisión Mixta."
                    ],
                    entregable_kit: "03_Acta_Constitutiva_Comision_Seguridad_NOM019_y_NOM030.pdf y Certificados NOM-019 / NOM-030",
                },
                NormaEducativa {
                    codigo: "NOM-017-STPS-2008, NOM-018-STPS-2015, NOM-026-STPS-2008 y NOM-028-STPS-2012",
                    titulo: "Organización: Equipo de Protección Personal (EPP), Sistema SGA Químico, Señalética y Procesos Críticos",
                    incumplimiento_pct: 55,
                    fuente_estadistica: "55% de las empresas entregan uniforme o equipo de protección sin haber elaborado la Matriz Técnica por Puesto (NOM-017) o utilizan productos químicos de limpieza/producción sin Hojas de Datos de Seguridad bajo el Sistema Globalmente Armonizado (NOM-018).",
                    rango_multa: "250 a 5,000 UMAs ($28,285.00 a $565,700.00 MXN)",
                    historia_dof: "NOM-026 (DOF 25-nov-2008), NOM-017 (DOF 9-dic-2008), NOM-028 (DOF 6-sep-2012) y NOM-018-STPS-2015 (DOF 9-oct-2015, que sustituyó el antiguo rombo NFPA por los 9 pictogramas internacionales SGA de Naciones Unidas).",
                    de_que_trata: "Regula la determinación y entrega documentada de EPP por región anatómica (NOM-017); la identificación y comunicación de peligros por sustancias químicas con Hojas de Datos de Seguridad de 16 secciones (NOM-018); los colores y señales de seguridad e identificación de tuberías (NOM-026); y la administración de seguridad en procesos químicos mayores (NOM-028).",
                    pasos_cumplimiento: &[
                        "Elaborar la Matriz de EPP por puesto de trabajo (NOM-017) y conservar acuses de entrega con firma autógrafa de cada empleado.",
                        "Etiquetar todo envase o recipiente de productos químicos (incluyendo desengrasantes, solventes y sanitizantes) con pictogramas SGA y mantener sus HDS en español (NOM-018).",
                        "Señalizar extintores, rutas de evacuación, riesgos eléctricos y tuberías conforme a las formas y colores de la NOM-026."
                    ],
                    entregable_kit: "Cédulas de Organización (NOM-017, 018, 026 y 028) en Dictamen Ejecutivo",
                },
                NormaEducativa {
                    codigo: "NOM-001, NOM-002, NOM-004, NOM-005, NOM-006, NOM-009, NOM-020, NOM-022, NOM-027, NOM-029, NOM-033 y NOM-034",
                    titulo: "Las 12 NOMs de Seguridad Física, Incendios, Maquinaria, Electricidad y Trabajos Peligrosos",
                    incumplimiento_pct: 59,
                    fuente_estadistica: "59% de las visitas de inspección en materia de Condiciones Generales de Seguridad detectan falta de Clasificación de Grado de Riesgo de Incendio (NOM-002), ausencia de Diagrama Unifilar eléctrico (NOM-029) o falta de expediente de compresores/calderas (NOM-020).",
                    rango_multa: "250 a 5,000 UMAs por cada NOM incumplida ($28,285.00 a $565,700.00 MXN)",
                    historia_dof: "Conforman el catálogo de Seguridad Industrial publicado en el DOF: NOM-001 (Edificios, 2008), NOM-002 (Incendios, 2010), NOM-004 (Maquinaria y bloqueo LOTO, 1999), NOM-005 (Manejo de sustancias químicas, 1998), NOM-006 (Almacenamiento y montacargas, 2014), NOM-009 (Trabajos en altura >1.80 m, 2011), NOM-020 (Recipientes sujetos a presión y calderas, 2011), NOM-022 (Electricidad estática y tierras físicas, 2015), NOM-027 (Soldadura y corte, 2008), NOM-029 (Mantenimiento eléctrico, 2011), NOM-033 (Espacios confinados, 2015) y NOM-034 (Trabajadores con discapacidad, 2016).",
                    de_que_trata: "Establecen los estándares físicos de infraestructura, protección contra incendios, guardas y candadeo en maquinaria, estiba en almacenes, permisos de trabajo peligroso (altura, corte/soldadura, espacios confinados), pararrayos/tierras físicas y accesibilidad para personas con discapacidad.",
                    pasos_cumplimiento: &[
                        "Contar siempre con los 4 documentos universales de seguridad física: (1) Verificación anual de instalaciones NOM-001; (2) Estudio de Grado de Riesgo de Incendio y bitácora de extintores NOM-002; (3) Diagrama Unifilar y cuadro de cargas NOM-029; y (4) Medición anual de resistencia de red de puesta a tierra NOM-022.",
                        "Si cuenta con compresores, hidroneumáticos o calderas (NOM-020), maquinaria (NOM-004), montacargas (NOM-006) o trabajos en altura/soldadura/espacios confinados (NOM-009/027/033), emitir permisos firmados por turno y certificar al personal con constancias DC-3."
                    ],
                    entregable_kit: "Matriz Completa de las 12 NOMs de Seguridad en Dictamen Ejecutivo",
                },
                NormaEducativa {
                    codigo: "NOM-010, NOM-011, NOM-012, NOM-013, NOM-014, NOM-015, NOM-024 y NOM-025",
                    titulo: "Las 8 NOMs de Salud e Higiene Industrial (Iluminación, Ruido, Temperaturas, Vibraciones y Radiaciones)",
                    incumplimiento_pct: 61,
                    fuente_estadistica: "61% de los establecimientos inspeccionados omiten el estudio de niveles de iluminación en luxes (NOM-025) o los estudios de ruido >85 dB(A) (NOM-011) y temperaturas elevadas/abatidas en cocinas, hornos o cámaras frías (NOM-015).",
                    rango_multa: "250 a 5,000 UMAs ($28,285.00 a $565,700.00 MXN)",
                    historia_dof: "Publicadas en el DOF para prevenir enfermedades profesionales: NOM-010 (Agentes químicos contaminantes del ambiente laboral, 2014), NOM-011 (Ruido, 2001), NOM-012 (Radiaciones ionizantes/Rayos X, 2012), NOM-013 (Radiaciones no ionizantes, 1993), NOM-014 (Presiones ambientales anormales, 2000), NOM-015 (Condiciones térmicas elevadas o abatidas, 2001), NOM-024 (Vibraciones, 2001) y NOM-025 (Iluminación, 2008).",
                    de_que_trata: "Obligan a reconocer, evaluar mediante laboratorios acreditados ante la EMA (Entidad Mexicana de Acreditación) y controlar los agentes físicos y químicos presentes en el ambiente laboral para que no rebasen los Límites Máximos Permisibles de Exposición (LMPE).",
                    pasos_cumplimiento: &[
                        "Realizar cada 2 años la evaluación de niveles de iluminación en luxes (NOM-025) en oficinas, áreas de atención, cocinas, talleres y almacenes.",
                        "En cocinas industriales, panaderías, hornos o cámaras de refrigeración/congelación: evaluar el índice de estrés térmico conforme a la NOM-015 y llevar vigilancia médica del personal expuesto.",
                        "En áreas con ruido igual o superior a 80 dB(A): implementar el Programa de Conservación de la Audición, audiometrías anuales y dotación de tapones/conchas acústicas con factor de atenuación NRR calculado (NOM-011)."
                    ],
                    entregable_kit: "Evaluación de las 8 NOMs de Salud e Higiene Industrial en Dictamen",
                },
                NormaEducativa {
                    codigo: "NOM-003, NOM-007, NOM-008, NOM-016, NOM-023, NOM-031 y NOM-032",
                    titulo: "Las 7 NOMs Específicas de Sector (Construcción NOM-031, Agrícola, Forestal, Minas y Ferrocarriles)",
                    incumplimiento_pct: 65,
                    fuente_estadistica: "65% de las obras de construcción y remodelación inspeccionadas bajo la NOM-031-STPS-2011 carecen del Análisis de Riesgos Potenciales por fase de obra y del Plan de Atención a Emergencias.",
                    rango_multa: "250 a 5,000 UMAs ($28,285.00 a $565,700.00 MXN) y suspensión inmediata de trabajos",
                    historia_dof: "Creadas para sectores de alta siniestralidad: NOM-003 (Insumos fitosanitarios/plaguicidas, 1999), NOM-007 (Instalaciones y maquinaria agrícola, 2000), NOM-008 (Aprovechamiento forestal maderable y aserraderos, 2013), NOM-016 (Operación y mantenimiento de ferrocarriles, 2001), NOM-023 (Minas subterráneas y a cielo abierto, 2012), NOM-031 (Construcción y obras civiles, 2011) y NOM-032 (Minas subterráneas de carbón, 2008).",
                    de_que_trata: "Activan protocolos especializados de seguridad industrial según los primeros dígitos del código SCIAN de la empresa (Construcción SCIAN 23, Agricultura/Forestal SCIAN 11, Minería SCIAN 21 y Transporte Ferroviario SCIAN 482).",
                    pasos_cumplimiento: &[
                        "En obras de construcción o remodelación (NOM-031-STPS-2011): clasificar el tamaño de la obra (Pequeña, Mediana o Grande), integrar el Programa de Seguridad y Salud en la Obra y exigir permisos firmados para excavaciones, demoliciones, andamios y grúas.",
                        "Verificar en la sección SCIAN de su Dictamen Ejecutivo cuál de las 7 normas específicas resulta aplicable a su giro."
                    ],
                    entregable_kit: "Cruce Automático de las 7 NOMs Específicas por Código SCIAN en Dictamen",
                },
            ],
        ),
        "sat" => (
            "SAT • CÓDIGO FISCAL DE LA FEDERACIÓN (CFF), LISR Y LIVA",
            "Servicio de Administración Tributaria (Materialidad, Buzón, CFDI 4.0, CSD y E-Commerce)",
            "Historia en el DOF, jurisprudencia de la SCJN, porcentajes reales de rechazo en auditoría y guía paso a paso para cada artículo fiscal.",
            "Desde la Reforma Hacendaria de 2014 y las reformas de 2020-2024, el SAT fiscaliza mediante algoritmos que cruzan en tiempo real los CFDI 4.0 contra listas negras (Art. 69 y 69-B), declaraciones provisionales, Buzón Tributario y plataformas digitales. Contar únicamente con la factura XML y la transferencia bancaria ya no acredita la deducción.",
            "emerald",
            "border-emerald-600",
            vec![
                NormaEducativa {
                    codigo: "Artículo 69-B y Artículo 69 del CFF • Jurisprudencia SCJN 2a./J. 161/2019 (10a.)",
                    titulo: "Materialidad y Sustancia Económica B2B (EFOS/EDOS), Fecha Cierta y Listas Negras",
                    incumplimiento_pct: 81,
                    fuente_estadistica: "81% de las deducciones por servicios intangibles, consultoría, marketing, mantenimiento o proveeduría B2B revisadas en auditorías del SAT son rechazadas porque el contribuyente solo exhibe el CFDI y un contrato simple sin Fecha Cierta ni entregables.",
                    rango_multa: "Rechazo del 100% de la deducción (30% ISR + 16% IVA) + Multa del 55% al 75% (Art. 76 CFF) y riesgo penal (Art. 113 Bis CFF)",
                    historia_dof: "El Artículo 69-B fue publicado en el DOF el 9 de diciembre de 2013 (en vigor el 1 de enero de 2014). El 6 de diciembre de 2019, la Segunda Sala de la Suprema Corte de Justicia de la Nación publicó la Jurisprudencia obligatoria 2a./J. 161/2019 (10a.), determinando que todo contrato privado exhibido ante el SAT debe contar con Fecha Cierta para acreditar que no fue elaborado con posterioridad.",
                    de_que_trata: "Faculta al SAT para publicar en el DOF y en su portal a quienes emiten comprobantes sin contar con activos, personal o infraestructura (Art. 69-B) y a los contribuyentes con créditos firmes o no localizados (Art. 69). Quien dedujo esos CFDI (EDO) tiene 30 días para acreditar la materialidad real de la operación.",
                    pasos_cumplimiento: &[
                        "Celebrar Contrato Marco de Prestación de Servicios con cláusulas de entregables y dotarlo de Fecha Cierta Digital inmutable (sello criptográfico SHA-256 / Merkle conforme al Código de Comercio y NOM-151) en la fecha de su firma.",
                        "Integrar por cada operación una Bitácora de Materialidad con: (1) Cotización y Orden de Compra; (2) Entregable verificable (reportes, planos, bitácoras, memoria fotográfica); (3) Opinión 32-D positiva del proveedor; y (4) CFDI 4.0 y REP.",
                        "Cruzar mensualmente el RFC de todos sus proveedores contra los listados completos del Art. 69 y Art. 69-B del SAT."
                    ],
                    entregable_kit: "05_Contrato_Marco_Materialidad_Fecha_Cierta_y_Bitacora_69B.pdf y Certificado SAT-CFF69B",
                },
                NormaEducativa {
                    codigo: "Artículo 17-K, Artículo 86-C y Artículo 86-D del CFF",
                    titulo: "Buzón Tributario Obligatorio y Medios de Contacto Actualizados",
                    incumplimiento_pct: 39,
                    fuente_estadistica: "39% de los contribuyentes presentan medios de contacto desactualizados (correos de ex-contadores o teléfonos dados de baja), ocasionando que los actos de fiscalización se notifiquen legalmente por estrados sin que la empresa se entere.",
                    rango_multa: "$3,850.00 a $11,540.00 MXN de multa directa (Art. 86-D CFF) + Notificación por estrados y restricción de CSD (Art. 17-H Bis Fracc. VIII)",
                    historia_dof: "Creado en la Reforma Fiscal publicada en el DOF el 9 de diciembre de 2013 y vuelto obligatorio con sanción pecuniaria directa en la reforma del 9 de diciembre de 2019.",
                    de_que_trata: "Obliga a todos los contribuyentes inscritos en el RFC a habilitar su Buzón Tributario y registrar y mantener actualizados al menos un correo electrónico (hasta cinco) y un número de teléfono móvil nacional.",
                    pasos_cumplimiento: &[
                        "Ingresar al Buzón Tributario con e.firma, verificar que aparezca el acuse de registro tanto del correo electrónico institucional de la dirección como del teléfono móvil, y atender cualquier aviso dentro de los 3 días hábiles siguientes."
                    ],
                    entregable_kit: "Verificación de Obligaciones Art. 17-K en Dictamen Pericial Ejecutivo",
                },
                NormaEducativa {
                    codigo: "Artículos 29 y 29-A del CFF • Anexo 20 CFDI 4.0, Carta Porte 3.1 y Complemento de Pago (REP)",
                    titulo: "Facturación Electrónica CFDI 4.0, Complemento Carta Porte 3.1, REP y Nómina Digital",
                    incumplimiento_pct: 49,
                    fuente_estadistica: "49% de las empresas incurren en emisión extemporánea del Complemento de Recepción de Pagos (REP) después del día 5 del mes siguiente, o movilizan mercancías sin el Complemento Carta Porte 3.1.",
                    rango_multa: "$400.00 a $600.00 MXN por cada CFDI sin complemento; $19,700.00 a $112,650.00 MXN por omisión de Carta Porte (Art. 84 Fracc. IV inc. d CFF) + No deducibilidad del gasto",
                    historia_dof: "La versión 4.0 del Anexo 20 se publicó en la Reforma Fiscal 2022 (única válida desde el 1 de abril de 2023), el plazo de 5 días para el REP entró en vigor en 2022 y la versión 3.1 del Complemento Carta Porte inició vigencia el 17 de julio de 2024.",
                    de_que_trata: "Exige validar en todo CFDI 4.0 el RFC, Nombre/Razón Social sin régimen societario, Código Postal y Régimen Fiscal del receptor; emitir el REP en operaciones en parcialidades (PPD) a más tardar el quinto día natural del mes siguiente al pago; y emitir Carta Porte 3.1 antes de trasladar bienes por tramos federales.",
                    pasos_cumplimiento: &[
                        "Solicitar Constancia de Situación Fiscal actualizada a clientes y proveedores para timbrar con exactitud el Código Postal y Uso del CFDI.",
                        "Bloquear pagos a proveedores que no hayan entregado el Complemento de Recepción de Pagos (REP) de las facturas PPD del mes anterior antes del día 5.",
                        "Emitir CFDI de Traslado o de Ingreso con Complemento Carta Porte 3.1 antes de que cualquier vehículo de carga salga a carretera federal."
                    ],
                    entregable_kit: "Motor de Facturación CFDI 4.0 Anexo 20 y Protocolo Carta Porte en Expediente",
                },
                NormaEducativa {
                    codigo: "Artículo 28 del CFF y Artículos 33 y 34 del Reglamento del CFF • Anexo 24",
                    titulo: "Contabilidad Electrónica, Envío de Balanza de Comprobación y Beneficiario Controlador (Art. 32-B Ter)",
                    incumplimiento_pct: 74,
                    fuente_estadistica: "74% de las PYMES y sociedades mercantiles presentan rezagos en el envío mensual de sus archivos XML de Catálogo de Cuentas y Balanza de Comprobación o carecen del expediente de Beneficiario Controlador que forma parte de la contabilidad.",
                    rango_multa: "$7,110.00 a $21,310.00 MXN por cada mes no enviado de Contabilidad Electrónica (Art. 81 Fracc. XLI y 82 Fracc. XXXVIII CFF); y de $1,500,000.00 a $2,000,000.00 MXN por omitir Beneficiario Controlador (Art. 84-M CFF)",
                    historia_dof: "La Contabilidad Electrónica se incorporó en la Reforma Fiscal de 2014 (avalada por la Segunda Sala de la SCJN en julio de 2016) y el expediente de Beneficiario Controlador se integró al Art. 28 y 32-B Ter del CFF el 12 de noviembre de 2021.",
                    de_que_trata: "Obliga a llevar los registros y asientos contables en medios electrónicos conforme al código agrupador del Anexo 24, enviar mensualmente la Balanza de Comprobación por el Buzón Tributario (los primeros 3 días hábiles del segundo mes posterior para personas morales) y conservar los 22 datos de cada Beneficiario Controlador.",
                    pasos_cumplimiento: &[
                        "Vincular cada póliza contable electrónica con el folio fiscal (UUID) de 36 caracteres del CFDI 4.0 que ampara la operación.",
                        "Mantener al día los acuses mensuales de envío de Balanza de Comprobación y la Balanza de Cierre (Balanza 13) junto con el expediente firmado de Beneficiarios Controladores."
                    ],
                    entregable_kit: "Protocolo de Contabilidad Electrónica Art. 28 y Beneficiario Controlador en Dictamen",
                },
                NormaEducativa {
                    codigo: "Artículo 27, Artículo 17-H Bis y Artículo 32-D del CFF",
                    titulo: "Domicilio Fiscal Localizado, Opinión de Cumplimiento 32-D y Protección de Sellos Digitales (CSD)",
                    incumplimiento_pct: 42,
                    fuente_estadistica: "42% de las restricciones temporales de Sellos Digitales (Art. 17-H Bis) ocurren por aparecer el domicilio fiscal como No Localizado o por discrepancias entre los CFDI emitidos y los ingresos declarados.",
                    rango_multa: "Restricción inmediata del Certificado de Sello Digital (parálisis de facturación) + Multa de $4,480.00 a $13,430.00 MXN por datos falsos o no actualizados en el RFC (Art. 79 y 80 CFF)",
                    historia_dof: "El Artículo 27 del CFF fue reestructurado integralmente el 9 de diciembre de 2019 (y reformado en 2021) creando el catálogo de obligaciones en materia de RFC para socios, accionistas, representantes legales y verificación física de domicilios.",
                    de_que_trata: "Obliga a mantener el domicilio fiscal en estatus de Localizado (con infraestructura, personal y contabilidad física en el sitio), presentar el Aviso de Socios y Accionistas en el RFC cada vez que se modifique el capital, y mantener la Opinión 32-D en sentido Positivo.",
                    pasos_cumplimiento: &[
                        "Verificar en la Constancia de Situación Fiscal que el estatus del domicilio diga expresamente Localizado y solicitar verificación ocular del SAT si aparece como Sin Verificar.",
                        "Presentar el Aviso de Actualización de Socios o Accionistas dentro de los 30 días hábiles siguientes a cualquier asamblea que modifique la tenencia accionaria (Art. 27 Apartado B Fracc. VI CFF)."
                    ],
                    entregable_kit: "Diagnóstico de Estatus RFC, CSD y Opinión 32-D en Dictamen Ejecutivo",
                },
                NormaEducativa {
                    codigo: "Artículos 113-A a 113-D de la LISR y Artículos 18-B a 18-M de la LIVA",
                    titulo: "Régimen Fiscal de Comercio Electrónico y Plataformas Digitales de Intermediación",
                    incumplimiento_pct: 46,
                    fuente_estadistica: "46% de los comercios que venden a través de plataformas tecnológicas o pasarelas digitales presentan errores en la conciliación de retenciones de ISR (1%) e IVA (8%) o en las declaraciones informativas del día 17.",
                    rango_multa: "Bloqueo temporal del acceso a internet del servicio digital en México (Art. 18-H Bis LIVA) y retención máxima del 20% de ISR + 16% de IVA por no proporcionar RFC válido",
                    historia_dof: "Incorporado a la Ley del ISR y a la Ley del IVA mediante decreto publicado en el DOF el 9 de diciembre de 2019 (en vigor desde el 1 de junio de 2020), con tasas unificadas de retención desde enero de 2021.",
                    de_que_trata: "Regula la tributación de quienes enajenan bienes, prestan servicios u otorgan hospedaje mediante plataformas tecnológicas, aplicaciones informáticas y similares, así como las obligaciones de retención y entero de las propias plataformas.",
                    pasos_cumplimiento: &[
                        "Registrar ante la plataforma tecnológica su RFC, CURP, cuenta CLABE y domicilio fiscal para aplicar la tasa reducida de retención (1% en enajenación de bienes y 4% en prestación de servicios) en lugar de la retención punitiva del 20% ISR / 16% IVA.",
                        "Cruzar mensualmente los CFDI de retenciones e información de pagos emitidos por la plataforma contra sus ingresos declarados."
                    ],
                    entregable_kit: "Evaluación Fiscal de E-Commerce y Plataformas Digitales en Dictamen",
                },
            ],
        ),
        "imss" => (
            "IMSS E INFONAVIT • LEY DEL SEGURO SOCIAL (LSS), RACERF Y LINFONAVIT",
            "Instituto Mexicano del Seguro Social e INFONAVIT (6 Vectores Patronales Completos)",
            "Historia legislativa, estadísticas de fiscalización electrónica y guía paso a paso para cada artículo de la LSS y del INFONAVIT.",
            "Promulgada originalmente en 1943 y regida por la Ley del Seguro Social vigente desde el 1 de julio de 1997, la seguridad social mexicana dota al IMSS y al INFONAVIT de facultades de Organismos Fiscales Autónomos. Hoy ambos institutos cruzan en tiempo real los XML de CFDI de Nómina 4.0 contra el SUA y el padrón REPSE.",
            "purple",
            "border-purple-600",
            vec![
                NormaEducativa {
                    codigo: "Artículos 15 y 27 a 30 de la LSS y Artículo 29 de la LINFONAVIT",
                    titulo: "Afiliación, Altas, Bajas y Conciliación de Salario Base de Cotización (SBC) Real vs. DENUE",
                    incumplimiento_pct: 57,
                    fuente_estadistica: "57% de las cartas invitación y auditorías del IMSS detectan discrepancias entre el personal ocupado físico o los conceptos timbrados en CFDI de Nómina (bonos, comisiones, premios, vales) y el Salario Base de Cotización declarado en el SUA.",
                    rango_multa: "20 a 350 UMAs ($2,262.80 a $39,599.00 MXN por trabajador no inscrito o con salario inferior — Art. 304-A y 304-B LSS) + Capitales Constitutivos (Art. 77 LSS) y multa del 40% al 100% de las cuotas omitidas",
                    historia_dof: "Regulado en los Artículos 15 y 27 de la LSS (DOF 21-dic-1995) y el Artículo 29 de la Ley del INFONAVIT (DOF 24-abr-1972). El cruce algorítmico entre CFDI de Nómina y SUA opera de forma automatizada desde 2018.",
                    de_que_trata: "Obliga a todo patrón a registrarse e inscribir a sus trabajadores en el IMSS dentro de un plazo no mayor de 5 días hábiles (preferentemente el día hábil anterior al inicio de labores para evitar Capitales Constitutivos en caso de accidente el primer día) e integrar al SBC todas las percepciones salvo las excluidas expresamente por el Artículo 27.",
                    pasos_cumplimiento: &[
                        "Dar de alta en IDSE a todo trabajador un día hábil antes de su ingreso físico al centro de trabajo.",
                        "Conciliar cada mes el visor de nómina del SAT contra las cédulas EMA/EBA del SUA, vigilando que premios de puntualidad/asistencia no rebasen el 10% del SBC y vales de despensa no superen el 40% de la UMA.",
                        "Presentar los avisos bimestrales de modificación salarial por percepciones variables en los primeros 5 días hábiles de enero, marzo, mayo, julio, septiembre y noviembre."
                    ],
                    entregable_kit: "06_Protocolo_Blindaje_Solidario_REPSE_ICSOE_SISSUB.pdf y Cédula SBC",
                },
                NormaEducativa {
                    codigo: "Artículo 15-A LSS • Artículos 12 a 15 LFT • Art. 29 Bis LINFONAVIT (ICSOE y SISSUB)",
                    titulo: "Reforma de Subcontratación, Padrón REPSE y Reportes Cuatrimestrales ICSOE / SISSUB",
                    incumplimiento_pct: 38,
                    fuente_estadistica: "38% de las empresas con registro REPSE o que contratan servicios especializados omiten o presentan fuera de plazo las informativas cuatrimestrales ICSOE (IMSS) y SISSUB (INFONAVIT) del día 17 de enero, mayo y septiembre.",
                    rango_multa: "500 a 2,000 UMAs ($56,570.00 a $226,280.00 MXN por cada informativa ICSOE omitida — Art. 304-A Fracc. XXII LSS); y de 2,000 a 50,000 UMAs ($226,280.00 a $5,657,000.00 MXN — Art. 1004-C LFT)",
                    historia_dof: "Publicado en el DOF el 23 de abril de 2021 mediante la reforma a 8 leyes federales que prohibió la subcontratación de personal y creó el registro público REPSE (con vigencia de 3 años y renovación obligatoria).",
                    de_que_trata: "Permite únicamente la subcontratación de servicios u obras especializadas que no formen parte del objeto social ni de la actividad económica preponderante de la beneficiaria, obligando al prestador a presentar cada cuatrimestre los informes de contratos celebrados en las plataformas ICSOE del IMSS y SISSUB del INFONAVIT.",
                    pasos_cumplimiento: &[
                        "Si presta servicios especializados: mantener vigente su folio REPSE y presentar a más tardar el día 17 de enero, mayo y septiembre las informativas ICSOE y SISSUB (incluso informativa Sin Información si no celebró contratos nuevos en el cuatrimestre).",
                        "Si contrata servicios especializados: recabar mensualmente del contratista los 7 comprobantes del Art. 27 Fracc. V LISR para garantizar la deducción de ISR y el acreditamiento de IVA."
                    ],
                    entregable_kit: "Checklist Mensual de 7 Comprobantes REPSE en 06_Protocolo_IMSS_REPSE_32D.pdf",
                },
                NormaEducativa {
                    codigo: "Artículos 71, 72, 73, 74 y 75 de la LSS y Artículos 18 a 38 y 196 del RACERF",
                    titulo: "Clasificación de Empresas y Revisión Anual de la Prima del Seguro de Riesgos de Trabajo",
                    incumplimiento_pct: 44,
                    fuente_estadistica: "44% de los patrones que sufrieron accidentes o enfermedades de trabajo omiten presentar la Determinación Anual de la Prima en febrero o excluyen indebidamente recaídas y formatos ST-7/ST-2.",
                    rango_multa: "Rectificación de oficio de Clase o Prima con cobro retroactivo y multa de 20 a 210 UMAs (Art. 304-A Fracc. XV LSS)",
                    historia_dof: "Establecido en el Capítulo III del Título Segundo de la LSS de 1997 y en el Catálogo de Actividades del Artículo 196 del RACERF (DOF 1-nov-2002).",
                    de_que_trata: "Obliga a autoclasificarse en una de las 5 Clases de Riesgo (Clase I: 0.54355% a Clase V: 7.58875%) según la peligrosidad de su actividad y maquinaria, y a recalcular cada mes de febrero la prima anual conforme a los días subsidiados, incapacidades permanentes y defunciones del año anterior.",
                    pasos_cumplimiento: &[
                        "Integrar el expediente completo de riesgos de trabajo ocurridos del 1 de enero al 31 de diciembre (formatos ST-7, ST-2, ST-3 y ST-9).",
                        "Presentar durante el mes de febrero la Determinación Anual de la Prima de Riesgos de Trabajo vía IDSE."
                    ],
                    entregable_kit: "Certificado_Cumplimiento_IMSS32D.pdf y Protocolo de Prima de Riesgo",
                },
                NormaEducativa {
                    codigo: "Artículo 39 de la LSS y Artículo 32-D del CFF • Emisiones EMA y EBA",
                    titulo: "Pago Oportuno de Cuotas (EMA/EBA) y Opinión de Cumplimiento 32-D Positiva en Seguridad Social",
                    incumplimiento_pct: 35,
                    fuente_estadistica: "35% de las empresas proveedoras ven retenidos sus pagos corporativos o quedan descalificadas en licitaciones por aparecer su Opinión 32-D del IMSS como Negativa debido a diferencias menores en las cédulas EMA o EBA.",
                    rango_multa: "Recargos moratorios, actualización, multa del 40% al 100% del crédito (Art. 304 LSS) y bloqueo de contratación pública y privada",
                    historia_dof: "Artículo 39 de la LSS (DOF 21-dic-1995), Reglas de Opinión de Cumplimiento (DOF 27-abr-2022) y Buzón IMSS obligatorio (DOF 5-ago-2024).",
                    de_que_trata: "Las cuotas obrero-patronales se causan por mensualidades vencidas y deben pagarse a más tardar el día 17 del mes inmediato siguiente. Para obtener Opinión 32-D Positiva se requiere no tener créditos firmes ni diferencias en los últimos 4 periodos.",
                    pasos_cumplimiento: &[
                        "Descargar el día 1 de cada mes las emisiones anticipadas EMA (mensual) y EBA (bimestral) desde IDSE para confrontarlas contra su archivo SUA antes de pagar el día 17.",
                        "Mantener habilitado el Buzón IMSS y hacer pública la autorización de consulta 32-D para sus clientes corporativos."
                    ],
                    entregable_kit: "Protocolo de Confronta EMA/EBA y Opinión 32-D en Kit 06",
                },
                NormaEducativa {
                    codigo: "Artículo 15-A párrafo tercero y Artículo 15 Fracción VI de la LSS (Responsabilidad Solidaria y SIROC)",
                    titulo: "Retención documental, Responsabilidad Solidaria en Servicios B2B y Registro de Obras SIROC",
                    incumplimiento_pct: 63,
                    fuente_estadistica: "63% de los propietarios o empresas que contratan obras de construcción, remodelación o mantenimiento en sus instalaciones no exigen el registro SIROC ni las constancias de pago SUA del contratista.",
                    rango_multa: "Exigibilidad del 100% de las cuotas obrero-patronales omitidas por el contratista más accesorios directamente contra el cliente beneficiario",
                    historia_dof: "Regulado en la reforma del 23 de abril de 2021 (Art. 15-A LSS) y en el Acuerdo del sistema SIROC publicado en el DOF el 13 de julio de 2017.",
                    de_que_trata: "Hace al cliente o propietario del inmueble responsable solidario del pago de las cuotas al IMSS de los trabajadores utilizados por sus contratistas cuando éstos incumplen sus obligaciones patronales.",
                    pasos_cumplimiento: &[
                        "Establecer como condición contractual para liberar cualquier pago de obra o servicio especializado la entrega previa de la Opinión 32-D positiva del IMSS/INFONAVIT, el pago SUA del mes y, en caso de obra civil o remodelación, el Acuse de Registro SIROC."
                    ],
                    entregable_kit: "Cláusulas de Indemnidad y Responsabilidad Solidaria en Kit 05 y Kit 06",
                },
                NormaEducativa {
                    codigo: "Artículos 304, 304-A y 304-B de la Ley del Seguro Social",
                    titulo: "Catálogo de las 22 Infracciones Directas y Sanciones Administrativas de la LSS",
                    incumplimiento_pct: 48,
                    fuente_estadistica: "48% de las multas impuestas bajo el Artículo 304-A derivan de la presentación extemporánea de avisos afiliatorios o de no conservar la documentación patronal durante el plazo de 5 años.",
                    rango_multa: "20 a 350 UMAs ($2,262.80 a $39,599.00 MXN) en supuestos ordinarios y hasta 2,000 UMAs ($226,280.00 MXN) en subcontratación",
                    historia_dof: "Adicionado al Título Sexto de la LSS mediante decreto publicado en el DOF el 20 de diciembre de 2001 y ampliado en la reforma de abril de 2021.",
                    de_que_trata: "Tipifica las 22 conductas infractoras de los patrones ante el IMSS (no registrarse, no inscribir trabajadores, no comunicar modificaciones de salario, no determinar la prima de riesgo, obstaculizar visitas domiciliarias o no presentar la informativa ICSOE) y permite la condonación de multas si se cumplen de forma espontánea antes de notificación.",
                    pasos_cumplimiento: &[
                        "Realizar auditoría preventiva trimestral de avisos IDSE para corregir de forma espontánea (sin multa conforme al Art. 304-C LSS) cualquier omisión antes de que medie requerimiento en el Buzón IMSS."
                    ],
                    entregable_kit: "Diagnóstico Integral de Riesgo Art. 304-A LSS en Dictamen Ejecutivo",
                },
            ],
        ),
        "inai" => (
            "SABG / EX-INAI • LEY FEDERAL DE PROTECCIÓN DE DATOS PERSONALES (LFPDPPP)",
            "Autoridad Garante Federal (SABG) • Los 6 Pilares de la LFPDPPP y su Reglamento",
            "Historia legislativa en el DOF, transición a la SABG, porcentajes reales de sanción y guía paso a paso para cada bloque de artículos.",
            "Publicada en el DOF el 5 de julio de 2010 y su Reglamento el 21 de diciembre de 2011, la LFPDPPP obliga a toda persona física o moral privada que trate datos de clientes, empleados o proveedores. Tras la reforma constitucional de diciembre de 2024 y la legislación secundaria de 2025, la autoridad verificadora y sancionadora es la Secretaría Anticorrupción y Buen Gobierno (SABG), manteniéndose vigentes todas las multas de 100 a 320,000 UMAs (que se duplican cuando involucran datos sensibles o biométricos).",
            "cyan",
            "border-cyan-600",
            vec![
                NormaEducativa {
                    codigo: "Artículos 8, 15 y 16 de la LFPDPPP y Lineamientos del Aviso de Privacidad (DOF 17/01/2013)",
                    titulo: "Aviso de Privacidad en sus 3 Modalidades: Integral, Simplificado y Corto",
                    incumplimiento_pct: 73,
                    fuente_estadistica: "73% de las empresas verificadas tienen un texto genérico o copiado en su página web que omite alguno de los 6 requisitos del Artículo 16 o no ofrece el mecanismo de 5 días para negar finalidades secundarias de mercadotecnia.",
                    rango_multa: "100 a 160,000 UMAs ($11,314.00 a $18,102,400.00 MXN — Art. 63 Fracc. II y 64 Fracc. II LFPDPPP)",
                    historia_dof: "Nació con la LFPDPPP (DOF 5-jul-2010) y se reglamentó al detalle en los Lineamientos del Aviso de Privacidad publicados en el DOF el 17 de enero de 2013.",
                    de_que_trata: "Materializa el Principio de Información: obliga a poner a disposición del titular el Aviso de Privacidad ANTES de recabar sus datos, utilizando el Aviso Integral en portales web y carpetas físicas, el Simplificado en formularios de contacto y el Corto en espacios reducidos como tickets o cámaras de videovigilancia (CCTV).",
                    pasos_cumplimiento: &[
                        "Publicar en el pie de página de su dominio web el Aviso de Privacidad Integral con los 6 requisitos del Art. 16: (1) Identidad y domicilio completo del responsable; (2) Datos personales sometidos a tratamiento; (3) Finalidades primarias y secundarias separadas; (4) Opciones para limitar el uso o divulgación; (5) Medios para ejercer Derechos ARCO; y (6) Transferencias y procedimiento de cambios al aviso.",
                        "Colocar el Aviso de Privacidad Corto impreso a la vista en recepción y zonas con cámaras CCTV."
                    ],
                    entregable_kit: "01_Aviso_de_Privacidad_Integral_ARCO_y_Biometricos.pdf y versión editable Word",
                },
                NormaEducativa {
                    codigo: "Artículos 22 al 35 de la LFPDPPP y Artículos 87 al 110 del RLFPDPPP",
                    titulo: "Mecanismos, Plazos y Procedimiento de Atención a los Derechos ARCO",
                    incumplimiento_pct: 64,
                    fuente_estadistica: "64% de los procedimientos de protección de derechos se inician porque la empresa no responde al ciudadano dentro del plazo máximo legal de 20 días hábiles o exige requisitos adicionales no previstos en el Artículo 29.",
                    rango_multa: "200 a 320,000 UMAs ($22,628.00 a $36,204,800.00 MXN — Art. 63 Fracc. VII y 64 Fracc. III LFPDPPP)",
                    historia_dof: "Capítulo IV de la LFPDPPP (DOF 5-jul-2010), que reglamentó en el sector privado el ejercicio de los 4 derechos constitucionales del Artículo 16 párrafo segundo.",
                    de_que_trata: "Garantiza a todo titular (cliente, ex-empleado, prospecto o proveedor) el derecho de solicitar el Acceso a sus datos, su Rectificación, su Cancelación (previo periodo de bloqueo del Art. 25) o su Oposición al tratamiento, así como la revocación de su consentimiento.",
                    pasos_cumplimiento: &[
                        "Establecer en su Aviso de Privacidad un correo electrónico oficial activo y un domicilio físico para recibir solicitudes ARCO.",
                        "Llevar una bitácora con acuse de recibo fechado: comunicar la determinación adoptada en un plazo máximo de 20 días hábiles (Art. 32 LFPDPPP) y, de resultar procedente, hacerla efectiva dentro de los 15 días hábiles siguientes."
                    ],
                    entregable_kit: "Procedimiento y Formulario Oficial de Derechos ARCO en Kit 01",
                },
                NormaEducativa {
                    codigo: "Artículos 3 Fracción VI, 9, 16 párrafo último y 64 Fracción IV de la LFPDPPP",
                    titulo: "Tratamiento de Datos Personales Sensibles (Salud, Biométricos de Asistencia y Financieros)",
                    incumplimiento_pct: 84,
                    fuente_estadistica: "84% de las empresas que utilizan huella dactilar o reconocimiento facial para control de asistencia o que resguardan certificados médicos y cuentas bancarias de nómina lo hacen sin contar con el consentimiento expreso y firma autógrafa del titular.",
                    rango_multa: "Hasta el DOBLE de las multas ordinarias: de 400 a 640,000 UMAs ($45,256.00 a $72,409,600.00 MXN — Art. 64 Fracc. IV LFPDPPP)",
                    historia_dof: "Artículo 9 de la LFPDPPP (DOF 5-jul-2010) y Guía para el Tratamiento de Datos Biométricos (2018).",
                    de_que_trata: "Exige obtener el consentimiento expreso y por escrito del titular (mediante su firma autógrafa, firma electrónica o mecanismo de autenticación) para tratar cualquier dato sensible (estado de salud, historia clínica, huella dactilar, iris, rostro, voz, afiliación sindical, creencias) y consentimiento expreso para datos financieros o patrimoniales (Art. 8 párrafo cuarto).",
                    pasos_cumplimiento: &[
                        "Recabar en el expediente de cada trabajador y visitante el Formato de Consentimiento Expreso y por Escrito con firma autógrafa antes de capturar su huella dactilar, rostro o datos médicos."
                    ],
                    entregable_kit: "Acta de Consentimiento Expreso Biométrico y Patrimonial en Kit 01",
                },
                NormaEducativa {
                    codigo: "Artículos 19, 20 y 63 Fracción VIII de la LFPDPPP y Artículos 57 a 66 del RLFPDPPP",
                    titulo: "Medidas de Seguridad Técnicas, Físicas y Administrativas (Documento de Seguridad y SSL/TLS)",
                    incumplimiento_pct: 79,
                    fuente_estadistica: "79% de las organizaciones muestran un aviso de privacidad en su portal pero carecen del Documento de Seguridad interno con inventario de datos personales, análisis de brecha y cifrado en bases de datos.",
                    rango_multa: "200 a 320,000 UMAs ($22,628.00 a $36,204,800.00 MXN — Art. 63 Fracc. VIII y 64 Fracc. III LFPDPPP)",
                    historia_dof: "Capítulo III del Reglamento de la LFPDPPP publicado en el DOF el 21 de diciembre de 2011.",
                    de_que_trata: "Obliga a mantener medidas de seguridad técnicas (cifrado TLS 1.3, control de accesos, respaldos), físicas (resguardo bajo llave de expedientes en papel) y administrativas integradas en un Documento de Seguridad con los 7 rubros del Artículo 61 del Reglamento.",
                    pasos_cumplimiento: &[
                        "Mantener vigente el certificado SSL/TLS en todos sus dominios y subdominios web.",
                        "Elaborar y actualizar anualmente el inventario de datos personales, matriz de privilegios de usuarios y bitácora de incidentes de seguridad."
                    ],
                    entregable_kit: "Auditoría Técnica TLS/SSL y Custodia de Documento de Seguridad en Kit 01",
                },
                NormaEducativa {
                    codigo: "Artículos 36 y 37 de la LFPDPPP y Artículos 49 a 56 y 67 a 75 del RLFPDPPP",
                    titulo: "Cláusulas de Transferencia a Terceros y Contratos de Remisión con Encargados (Despachos y Nube)",
                    incumplimiento_pct: 67,
                    fuente_estadistica: "67% de las empresas envían bases de datos de nómina, clientes o facturación a despachos contables externos, agencias de marketing o proveedores de software sin haber firmado cláusulas contractuales de Encargado.",
                    rango_multa: "200 a 320,000 UMAs por transferir datos a terceros sin comunicarles el Aviso de Privacidad o fuera de las excepciones del Art. 37 (Art. 63 Fracc. XII LFPDPPP)",
                    historia_dof: "Capítulo V de la LFPDPPP (DOF 5-jul-2010) y Capítulo II del RLFPDPPP (DOF 21-dic-2011), que introdujo la distinción jurídica entre Transferencia (a otro Responsable) y Remisión (a un Encargado que presta un servicio al Responsable).",
                    de_que_trata: "Regula la comunicación de datos personales a terceros nacionales o extranjeros. Exige cláusula de aceptación en el Aviso de Privacidad para transferencias (salvo las 7 excepciones del Art. 37) y contratos con cláusulas de confidencialidad y seguridad para los Encargados (despachos, maquiladoras de nómina, CRM y hosting).",
                    pasos_cumplimiento: &[
                        "Incluir en los contratos con su despacho contable, proveedor de nómina y proveedores tecnológicos las cláusulas de Encargado del Artículo 50 del Reglamento (tratar datos solo conforme a instrucciones del responsable, guardar confidencialidad y suprimir los datos al terminar el contrato)."
                    ],
                    entregable_kit: "Cláusulas de Transferencia y Remisión a Encargados en Kit 01",
                },
                NormaEducativa {
                    codigo: "Artículo 17 Fracción I y Artículo 30 de la LFPDPPP",
                    titulo: "Designación Formal del Oficial de Privacidad o Departamento de Datos Personales",
                    incumplimiento_pct: 61,
                    fuente_estadistica: "61% de las empresas inspeccionadas no cuentan con el acta interna que designa formalmente a la persona o departamento responsable de tramitar solicitudes ARCO y fomentar la protección de datos dentro de la organización.",
                    rango_multa: "100 a 160,000 UMAs por incumplir el principio de responsabilidad y gobernanza interna (Art. 63 y 64 LFPDPPP)",
                    historia_dof: "Establecido de manera obligatoria en el Artículo 30 de la LFPDPPP desde el 5 de julio de 2010.",
                    de_que_trata: "Todo responsable debe designar a una persona, o departamento de datos personales, quien dará trámite a las solicitudes de los titulares y fomentará la protección de datos personales en el interior de la organización.",
                    pasos_cumplimiento: &[
                        "Suscribir el Acta de Designación del Oficial de Privacidad e imprimir su nombre o cargo y correo oficial dentro del Aviso de Privacidad Integral."
                    ],
                    entregable_kit: "Nombramiento Formal del Oficial de Privacidad en 01_Aviso_de_Privacidad_Integral.pdf",
                },
            ],
        ),
        "pld" => (
            "LOS 9 MÓDULOS SECTORIALES DINÁMICOS Y PREVENCIÓN DE LAVADO DE DINERO (LFPIORPI)",
            "Regulación Sectorial por Giro SCIAN (UIF/CNBV, COFEPRIS, PIPC, SEMARNAT, SICT, REPSE, SSPC, SEP y SECTUR)",
            "Historia en el DOF, porcentaje real de empresas clausuradas o multadas y guía paso a paso para cada uno de los 9 Módulos Sectoriales.",
            "Además de las obligaciones generales laborales y fiscales, el motor pericial de Firma Pericial Mallen activa dinámicamente los 9 Módulos Sectoriales según el código SCIAN de 6 dígitos de la empresa, la superficie de sus instalaciones (>100 m²) y la naturaleza de sus operaciones financieras o comerciales.",
            "amber",
            "border-amber-600",
            vec![
                NormaEducativa {
                    codigo: "Módulo Sectorial 01 • CNBV / CONDUSEF / LFPIORPI (Artículos 17, 18, 21, 32, 53 y 54)",
                    titulo: "Fintech, Sector Financiero y las 16 Actividades Vulnerables Antilavado (SHCP / UIF / SAT)",
                    incumplimiento_pct: 69,
                    fuente_estadistica: "69% de las empresas no financieras que otorgan préstamos intercompañía (mutuos), arrendan inmuebles, desarrollan obras inmobiliarias o comercializan vehículos/activos virtuales rebasando los umbrales en UMAs omiten su alta en el portal SPPLD y los avisos del día 17.",
                    rango_multa: "10,000 a 65,000 UMAs ($1,131,400.00 a $7,354,100.00 MXN) o del 10% al 100% del valor del acto u operación por cada aviso omitido (Arts. 53 y 54 LFPIORPI)",
                    historia_dof: "La Ley Federal para la Prevención e Identificación de Operaciones con Recursos de Procedencia Ilícita (LFPIORPI) fue publicada en el DOF el 17 de octubre de 2012 (en vigor el 17 de julio de 2013), complementada por la Ley Fintech (DOF 9-mar-2018).",
                    de_que_trata: "Obliga a quienes realicen cualquiera de las 16 Actividades Vulnerables del Artículo 17 (préstamos/mutuos, construcción o venta de inmuebles, arrendamiento, vehículos, blindaje, joyería, traslado de valores, servicios profesionales independientes, fe pública, comercio exterior y activos virtuales) a identificar al cliente y Dueño Beneficiario, restringir el uso de efectivo (Art. 32) y enviar Avisos a la UIF.",
                    pasos_cumplimiento: &[
                        "Darse de alta en el Portal SPPLD del SAT, designar Oficial de Cumplimiento (Art. 20) y contar con su Manual de Cumplimiento PLD.",
                        "Integrar el Expediente Único de Identificación de cada cliente o contraparte ANTES del acto y presentar los Avisos o Informes en Cero a más tardar el día 17 de cada mes."
                    ],
                    entregable_kit: "Dictamen de Actividades Vulnerables LFPIORPI en Expediente Ejecutivo",
                },
                NormaEducativa {
                    codigo: "Módulo Sectorial 02 • COFEPRIS (Arts. 198, 200 Bis y 202 LGS • NOM-251-SSA1 y NOM-051-SCFI/SSA1)",
                    titulo: "Salud, Alimentos/Restaurantes, Cosméticos, Suplementos y Farmacia (DIGIPRIS)",
                    incumplimiento_pct: 52,
                    fuente_estadistica: "52% de los restaurantes, pizzerías, cafeterías, clínicas, consultorios, farmacias y comercializadoras de alimentos/cosméticos inspeccionados presentan omisiones en su Aviso de Funcionamiento DIGIPRIS, bitácoras NOM-251 o etiquetado NOM-051.",
                    rango_multa: "Aseguramiento precautorio de mercancía, suspensión de trabajos o servicios y multas de 2,000 a 16,000 UMAs ($226,280.00 a $1,810,240.00 MXN — Art. 416 a 422 LGS)",
                    historia_dof: "Ley General de Salud (DOF 7-feb-1984), creación de COFEPRIS (DOF 5-jul-2001), NOM-251-SSA1-2009 para higiene en alimentos y bebidas (DOF 1-mar-2010) y reforma de sellos octagonales de la NOM-051 (DOF 27-mar-2020).",
                    de_que_trata: "Exige contar con Licencia Sanitaria (establecimientos de alto riesgo médico/farmacéutico) o Aviso de Funcionamiento y Responsable Sanitario en DIGIPRIS antes de operar, así como cumplir las bitácoras de higiene en preparación de alimentos (NOM-251) y etiquetado frontal (NOM-051).",
                    pasos_cumplimiento: &[
                        "Mantener exhibido el Acuse del Aviso de Funcionamiento DIGIPRIS con la clave SCIAN exacta de la actividad del establecimiento.",
                        "En restaurantes, pizzerías y servicios de alimentos (NOM-251-SSA1-2009): llevar bitácoras firmadas de temperaturas de refrigeración (<4°C) y congelación (-18°C), desinfección de agua/insumos, limpieza por áreas y certificado vigente de fumigación expedido por empresa con Licencia Sanitaria COFEPRIS."
                    ],
                    entregable_kit: "Módulo Sectorial COFEPRIS y Checklist NOM-251 / NOM-051 en Dictamen",
                },
                NormaEducativa {
                    codigo: "Módulo Sectorial 03 • PROTECCIÓN CIVIL ESTATAL Y MUNICIPAL (Arts. 39, 40 y 74 a 78 LGPC)",
                    titulo: "Programa Interno de Protección Civil (PIPC >100 m² o Afluencia) y Dictámenes Estructural, Eléctrico y Gas",
                    incumplimiento_pct: 66,
                    fuente_estadistica: "66% de los establecimientos comerciales, restaurantes, oficinas e industrias con superficie mayor a 100 m² o afluencia de público operan con su Programa Interno de Protección Civil (PIPC) vencido o sin visto bueno anual.",
                    rango_multa: "Clausura temporal o definitiva inmediata del inmueble con colocación de sellos + Multas de 1,000 a 20,000 UMAs según ley estatal",
                    historia_dof: "El Sistema Nacional de Protección Civil (SINAPROC) se fundó en 1986; la Ley General de Protección Civil vigente se publicó en el DOF el 6 de junio de 2012 y su Reglamento el 13 de mayo de 2014.",
                    de_que_trata: "Obliga a todo centro de trabajo con superficie superior a 100 m², manejo de gas/químicos o afluencia de público a contar con su PIPC vigente elaborado por Tercer Acreditado / Consultor Registrado, acompañado de dictámenes técnicos de instalaciones.",
                    pasos_cumplimiento: &[
                        "Integrar el Acta Constitutiva de la Unidad Interna de Protección Civil y las 4 Brigadas (Evacuación, Primeros Auxilios, Prevención y Combate de Incendios, y Comunicación).",
                        "Contar con Póliza de Seguro de Responsabilidad Civil vigente, Dictamen de Instalaciones Eléctricas (UVIE), Dictamen de Instalaciones de Gas LP o Natural (obligatorio en restaurantes y plantas) y Constancia de Seguridad Estructural (DRO)."
                    ],
                    entregable_kit: "Certificado_Seguridad_ProtCivil.pdf y Guía de Unidad Interna PIPC",
                },
                NormaEducativa {
                    codigo: "Módulo Sectorial 04 • SEMARNAT / ASEA / PROFEPA (LGEEPA Arts. 109 Bis y 150 • LGPGIR Arts. 40 a 48)",
                    titulo: "Impacto Ambiental, Licencia Ambiental Única (LAU), COA y Generador de Residuos Peligrosos",
                    incumplimiento_pct: 59,
                    fuente_estadistica: "59% de los talleres, plantas, imprentas, clínicas y restaurantes generadores de residuos peligrosos o de manejo especial (aceites, grasas, solventes, estopas, RPBI) carecen de su registro de generador y manifiestos de recolección autorizada.",
                    rango_multa: "Clausura total o parcial por PROFEPA y multas de 30 a 50,000 UMAs ($3,394.00 a $5,657,000.00 MXN — Art. 171 LGEEPA)",
                    historia_dof: "LGEEPA (DOF 28-ene-1988), LGPGIR (DOF 8-oct-2003) y Ley de la Agencia de Seguridad, Energía y Ambiente ASEA (DOF 11-ago-2014).",
                    de_que_trata: "Regula las emisiones a la atmósfera, descargas de aguas residuales y el registro y disposición de Residuos Peligrosos (Microgenerador <400 kg/año; Pequeño 400 kg a 10 ton/año; Gran Generador >10 ton/año) y Residuos Peligrosos Biológico-Infecciosos (NOM-087-SEMARNAT-SSA1-2002).",
                    pasos_cumplimiento: &[
                        "Obtener el Número de Registro Ambiental (NRA) como Generador de Residuos Peligrosos o de Manejo Especial y entregar los residuos únicamente a transportistas autorizados conservando los Manifiestos sellados por 5 años."
                    ],
                    entregable_kit: "Módulo Sectorial SEMARNAT / PROFEPA en Dictamen Ejecutivo",
                },
                NormaEducativa {
                    codigo: "Módulo Sectorial 05 • SICT / CAPUFE (Ley de Caminos, Puentes y Autotransporte Federal y NOM-087-SCT)",
                    titulo: "Transporte Federal de Carga, Fletes, Logística, Bitácoras de Horas y Pesos/Dimensiones (NOM-012-SCT)",
                    incumplimiento_pct: 48,
                    fuente_estadistica: "48% de las unidades de carga y flotillas corporativas inspeccionadas en carreteras federales presentan incumplimientos en la Bitácora de Horas de Servicio del conductor (NOM-087-SCT-2-2017) o en la verificación físico-mecánica (NOM-068-SCT).",
                    rango_multa: "Detención y remisión de la unidad al depósito federal + Multas de hasta 500 UMAs por infracción (Art. 70 a 74 LCPAF)",
                    historia_dof: "Ley de Caminos, Puentes y Autotransporte Federal (DOF 22-dic-1993), NOM-068-SCT-2-2014 (DOF 19-ene-2015), NOM-012-SCT-2-2017 (DOF 26-dic-2017) y NOM-087-SCT-2-2017 (DOF 28-jun-2018).",
                    de_que_trata: "Obliga a contar con permiso de la SICT para autotransporte federal o transporte privado de carga, póliza de seguro de responsabilidad civil por al menos 19,000 UMAs, dictamen anual físico-mecánico y de bajas emisiones, bitácora de pausas de conducción (30 minutos de descanso por cada 5 horas) y Carta Porte 3.1.",
                    pasos_cumplimiento: &[
                        "Portar en cada vehículo de carga o reparto foráneo: Tarjeta de Circulación SICT, Póliza de Seguro vigente, Dictamen Físico-Mecánico NOM-068, Bitácora de Horas de Servicio NOM-087 y CFDI con Complemento Carta Porte 3.1."
                    ],
                    entregable_kit: "Módulo Sectorial SICT / Logística en Dictamen Ejecutivo",
                },
                NormaEducativa {
                    codigo: "Módulos Sectoriales 06, 07, 08 y 09 • REPSE B2B • SSPC (Seguridad Privada) • SEP (RVOE) • SECTUR (RNT)",
                    titulo: "Servicios Especializados REPSE, Seguridad Privada (DGSP), Educación (RVOE) y Turismo/Hospitalidad (RNT)",
                    incumplimiento_pct: 54,
                    fuente_estadistica: "54% de las empresas de seguridad privada, agencias/hoteles turísticos y centros de capacitación operan sin revalidación anual de su permiso SSPC, sin inscripción en el Registro Nacional de Turismo (RNT) o sin mención expresa del estatus de RVOE.",
                    rango_multa: "Clausura y multas de hasta 5,000 UMAs en Seguridad Privada (Art. 42 LFSP); hasta 7,000 UMAs en Educación (Art. 171 LGE); y suspensión en Turismo (Art. 69 LGT)",
                    historia_dof: "Ley Federal de Seguridad Privada (DOF 6-jul-2006), Ley General de Turismo (DOF 17-jun-2009), Ley General de Educación (DOF 30-sep-2019) y Reforma de Subcontratación REPSE (DOF 23-abr-2021).",
                    de_que_trata: "Regula los 4 regímenes de autorización especial de servicios: (6) Padrón REPSE e informativas ICSOE/SISSUB; (7) Autorización federal de la Dirección General de Seguridad Privada (SSPC/DGSP) cuando se opera en dos o más estados y registro de personal en el RNPSP; (8) Reconocimiento de Validez Oficial de Estudios (RVOE) ante la SEP o aviso de estudios sin RVOE; y (9) Inscripción obligatoria en el Registro Nacional de Turismo (RNT) para hoteles, restaurantes turísticos, agencias de viajes y transportadoras.",
                    pasos_cumplimiento: &[
                        "Seguridad Privada (SSPC): revalidar anualmente la autorización federal/estatal y consultar antecedentes del personal operativo en el RNPSP.",
                        "Turismo y Hospitalidad (SECTUR): tramitar y exhibir el certificado del Registro Nacional de Turismo (RNT) vigente por 2 años y cumplir la NOM-010-TUR-2001.",
                        "Educación y Capacitación (SEP/STPS): exhibir el número de acuerdo RVOE en toda publicidad o indicar expresamente cuando los cursos sean de educación continua sin RVOE (contando con registro de Agente Capacitador Externo DC-5 ante la STPS)."
                    ],
                    entregable_kit: "Evaluación de los Módulos 06, 07, 08 y 09 por Giro SCIAN en Dictamen Ejecutivo",
                },
            ],
        ),
        _ => (
            "PROFECO Y SECRETARÍA DE ECONOMÍA • LEY FEDERAL DE PROTECCIÓN AL CONSUMIDOR (LFPC)",
            "Procuraduría Federal del Consumidor y Secretaría de Economía (Los 6 Vectores de la LFPC y NOM-151)",
            "Historia en el DOF, monitoreo de tiendas virtuales, estadísticas reales de sanción y guía paso a paso por artículo de la LFPC.",
            "Promulgada originalmente en 1975 y regida por la ley vigente de 1992, la Ley Federal de Protección al Consumidor (LFPC) es de orden público e interés social e irrenunciable. PROFECO supervisa tanto establecimientos físicos (restaurantes, comercios, exhibición de precios totales con IVA y prohibición de propina obligatoria) como tiendas en línea y portales B2B/B2C.",
            "teal",
            "border-teal-600",
            vec![
                NormaEducativa {
                    codigo: "Artículo 76 BIS y 76 BIS 1 de la LFPC y Norma Mexicana NMX-COE-001-SCFI-2018",
                    titulo: "Comercio Electrónico, Portales Digitales B2B/B2C y Transparencia de Datos del Proveedor",
                    incumplimiento_pct: 56,
                    fuente_estadistica: "56% de los sitios web comerciales revisados por el Monitoreo de Tiendas Virtuales de PROFECO omiten publicar su domicilio físico real en territorio nacional, sus teléfonos de atención o las características detalladas del bien o servicio.",
                    rango_multa: "$701.15 a $3,995,249.26 MXN (Artículos 127 y 128 LFPC) y colocación de sellos de advertencia digital",
                    historia_dof: "El Capítulo VIII Bis (Artículo 76 BIS) fue adicionado a la LFPC mediante decreto publicado en el DOF el 29 de mayo de 2000 y complementado el 30 de abril de 2019 con la norma mexicana de comercio electrónico NMX-COE-001-SCFI-2018.",
                    de_que_trata: "Obliga a todo proveedor que ofrezca, comercialice o venda bienes o servicios por medios electrónicos a informar su domicilio físico en México, teléfonos, correo, características reales de los productos y monto total a pagar en Moneda Nacional con IVA incluido (Art. 7 BIS).",
                    pasos_cumplimiento: &[
                        "Publicar de forma visible en el pie de página de su sitio web su Razón Social, RFC, domicilio físico completo en México, teléfono fijo y correo electrónico de atención.",
                        "Mostrar siempre precios totales en Pesos Mexicanos (MXN) con IVA e impuestos incluidos. En restaurantes y servicios (Art. 7 BIS y Art. 10 LFPC): exhibir carta con precios totales a la entrada y no incluir ni cargar propina sin consentimiento expreso del cliente."
                    ],
                    entregable_kit: "07_Terminos_Condiciones_Ecommerce_Art76BIS_y_Garantias.pdf y versión editable Word",
                },
                NormaEducativa {
                    codigo: "Artículo 76 BIS Fracciones III y VII, Artículo 56 y Artículos 77 a 82 de la LFPC",
                    titulo: "Políticas de Devolución, Cancelación, Revocación en 5 Días Hábiles y Garantía Mínima de 90 Días",
                    incumplimiento_pct: 63,
                    fuente_estadistica: "63% de los comercios y portales web exhiben leyendas de Garantía de 30 días o Sin cambios ni devoluciones, las cuales violan directamente el mínimo legal de 90 días del Artículo 77 y el derecho de revocación del Artículo 56 de la LFPC.",
                    rango_multa: "$980.78 a $3,839,580.66 MXN (Art. 128 LFPC) + reposición forzosa del bien o reembolso más bonificación mínima del 20% del precio pagado (Art. 92 TER LFPC)",
                    historia_dof: "El plazo mínimo de garantía comercial fue incrementado de 60 a 90 días naturales mediante reforma al Artículo 77 publicada en el DOF el 11 de enero de 2018.",
                    de_que_trata: "Otorga 5 días hábiles para revocar sin penalización compras celebradas a distancia o fuera del local comercial (Art. 56 LFPC) y obliga a que toda póliza de garantía tenga una vigencia mínima de 90 días naturales y se redacte en español con los datos del Artículo 78.",
                    pasos_cumplimiento: &[
                        "Actualizar sus pólizas de garantía y términos web a un plazo mínimo de 90 días naturales, indicando claramente el domicilio en México donde el consumidor puede hacerla efectiva.",
                        "Publicar el procedimiento claro para devoluciones, cancelaciones, reembolsos en un máximo de 15 días y atención de reclamaciones."
                    ],
                    entregable_kit: "Póliza de Garantía de 90 Días y Procedimiento de Reembolso en Kit 07 PROFECO",
                },
                NormaEducativa {
                    codigo: "Artículos 85, 86, 86 BIS, 87 y 90 de la LFPC",
                    titulo: "Contratos de Adhesión, Registro Público (RRPAC) y Prohibición de Cláusulas Abusivas",
                    incumplimiento_pct: 72,
                    fuente_estadistica: "72% de las empresas de servicios que emplean formatos estandarizados de contratación carecen de número de registro RRPAC ante PROFECO o incluyen cláusulas unilaterales nulas conforme al Artículo 90.",
                    rango_multa: "Ineficacia jurídica del contrato no registrado (Art. 86 TER LFPC) y multas de $980.78 a $3,839,580.66 MXN (Art. 128 LFPC)",
                    historia_dof: "Regulado en el Capítulo X de la LFPC (DOF 24-dic-1992) y operado mediante el Registro Público de Contratos de Adhesión en Línea (RRPAC).",
                    de_que_trata: "Define como contrato de adhesión todo documento elaborado unilateralmente por el proveedor en formatos uniformes. Cuando una NOM o disposición sectorial exige su registro previo (inmobiliarias NOM-247, eventos sociales, tintorerías, talleres mecánicos, tiempos compartidos, telecomunicaciones), no surte efectos contra el consumidor si no ostenta su folio RRPAC.",
                    pasos_cumplimiento: &[
                        "Purgar de todos sus contratos y términos web las cláusulas prohibidas por el Artículo 90 de la LFPC (modificación unilateral de términos, liberación de responsabilidad civil del proveedor o penas convencionales no equitativas)."
                    ],
                    entregable_kit: "Certificado_Comercial_PROFECO.pdf y Términos Alineados al Art. 85-90 LFPC",
                },
                NormaEducativa {
                    codigo: "Artículos 32, 32 BIS y 128 de la Ley Federal de Protección al Consumidor",
                    titulo: "Publicidad Veraz, Comprobable y Prohibición de Prácticas Comerciales Engañosas",
                    incumplimiento_pct: 47,
                    fuente_estadistica: "47% de los procedimientos sancionadores por publicidad en redes sociales y sitios web derivan del uso de afirmaciones absolutas o superlativas no comprobables técnicamente o promociones sin vigencia ni restricciones claras.",
                    rango_multa: "Orden precautoria de suspensión inmediata de la publicidad (Art. 35 LFPC) y multa de $980.78 a $3,839,580.66 MXN (Art. 128 LFPC)",
                    historia_dof: "Reformado mediante decretos publicados en el DOF el 4 de febrero de 2004 y el 11 de enero de 2018 (y complementado en 2023 con la Guía de Publicidad para Influencers y Medios Digitales de PROFECO).",
                    de_que_trata: "Establece que toda información o publicidad relativa a bienes, productos o servicios que se difunda por cualquier medio o forma deberá ser veraz, comprobable, clara y exenta de textos, diálogos, sonidos, imágenes, marcas o denominaciones que induzcan o puedan inducir a error o confusión por engañosas o abusivas.",
                    pasos_cumplimiento: &[
                        "En toda promoción, descuento u oferta publicada en web o redes sociales (Art. 46 LFPC): indicar siempre las condiciones, el plazo de vigencia o el volumen exacto de piezas disponibles.",
                        "Conservar el soporte técnico o documental que acredite cualquier atributo cuantitativo prometido en su publicidad comercial."
                    ],
                    entregable_kit: "Lineamientos de Publicidad Veraz Art. 32 en 07_Terminos_Ecommerce_PROFECO_76BIS.pdf",
                },
                NormaEducativa {
                    codigo: "Artículo 128 BIS y Artículo 76 BIS Fracción II de la LFPC • NOM-151-SCFI-2016",
                    titulo: "Seguridad en Transacciones Financieras Digitales, Pasarelas de Pago y Conservación NOM-151",
                    incumplimiento_pct: 53,
                    fuente_estadistica: "53% de las denuncias graves en comercio digital involucran cobros no reconocidos o portales que procesan pagos sin estándares de autenticación 3D Secure / TLS ni sellado de integridad NOM-151.",
                    rango_multa: "Caso particularmente grave: Multa de $196,634.08 a $5,505,754.46 MXN y clausura total o parcial hasta por 90 días (Art. 128 BIS LFPC)",
                    historia_dof: "El Artículo 128 BIS fue adicionado en la reforma publicada en el DOF el 4 de febrero de 2004 y actualizado el 11 de enero de 2018; la NOM-151-SCFI-2016 se publicó en el DOF el 30 de marzo de 2017.",
                    de_que_trata: "Exige utilizar elementos técnicos confiables para brindar seguridad y confidencialidad a la información financiera proporcionada por el consumidor (tokenización PCI-DSS, cifrado TLS 1.3 y autenticación 3D Secure) y conservar los mensajes de datos íntegros conforme al Código de Comercio y la NOM-151.",
                    pasos_cumplimiento: &[
                        "Operar el cobro con tarjeta exclusivamente mediante pasarelas tokenizadas certificadas PCI-DSS Nivel 1 con regla obligatoria de 3D Secure.",
                        "Sellar cada transacción y contrato electrónico con huella criptográfica SHA-256 e incluirlo en el Árbol de Merkle de Fecha Cierta."
                    ],
                    entregable_kit: "Certificado_Digital_NOM151.pdf y Manifiesto de Fecha Cierta SCJN",
                },
            ],
        ),
    };

    let mut modulos_html = String::new();
    for n in &normas {
        let mut pasos_li = String::new();
        for paso in n.pasos_cumplimiento {
            pasos_li.push_str(&format!(
                r##"<li class="flex items-start gap-2.5 text-xs text-slate-200 leading-relaxed">
                    <span class="text-{color_tw}-400 font-bold mt-0.5">▪</span>
                    <span>{paso}</span>
                </li>"##
            ));
        }

        modulos_html.push_str(&format!(
            r##"<div class="quantum-card rounded-2xl p-6 sm:p-8 border border-{color_tw}-500/30 mb-8">
                <div class="flex flex-col lg:flex-row lg:items-center justify-between gap-4 pb-5 mb-6 border-b border-white/10">
                    <div>
                        <span class="inline-block text-[11px] font-bold font-mono-code text-{color_tw}-400 bg-{color_tw}-950/60 border border-{color_tw}-500/30 px-3 py-1 rounded-lg mb-2">
                            {codigo}
                        </span>
                        <h2 class="text-lg sm:text-xl font-extrabold text-white tracking-tight">{titulo_norma}</h2>
                    </div>
                    <div class="lg:text-right bg-slate-950/70 border border-red-500/30 rounded-xl px-4 py-3 shrink-0">
                        <div class="flex items-center lg:justify-end gap-2 mb-1">
                            <span class="text-[10px] font-mono-code uppercase text-slate-400">Incumplimiento en México:</span>
                            <span class="text-sm font-extrabold font-mono-code text-red-400">{pct}%</span>
                        </div>
                        <div class="w-full lg:w-56 bg-slate-800 h-2 rounded-full overflow-hidden mb-1.5">
                            <div class="bg-gradient-to-r from-amber-500 to-red-500 h-full rounded-full" style="width: {pct}%"></div>
                        </div>
                        <span class="text-[10px] font-mono-code text-amber-300 block">Sanción: {multa}</span>
                    </div>
                </div>

                <div class="bg-red-950/20 border border-red-500/25 rounded-xl p-4 mb-6 text-xs font-mono-code text-slate-200 leading-relaxed">
                    <strong class="text-red-400 uppercase block text-[10px] mb-1">Estadística Real de Fiscalización e Inspección:</strong>
                    {estadistica}
                </div>

                <div class="grid md:grid-cols-2 gap-6 mb-6">
                    <div class="bg-slate-950/55 border border-white/10 rounded-xl p-5">
                        <span class="text-[10px] font-bold font-mono-code text-{color_tw}-400 uppercase tracking-wider block mb-2">
                            Historia y Origen en el Diario Oficial (DOF)
                        </span>
                        <p class="text-xs text-slate-300 leading-relaxed font-mono-code">
                            {historia}
                        </p>
                    </div>
                    <div class="bg-slate-950/55 border border-white/10 rounded-xl p-5">
                        <span class="text-[10px] font-bold font-mono-code text-emerald-400 uppercase tracking-wider block mb-2">
                            ¿De qué se trata y a quién obliga?
                        </span>
                        <p class="text-xs text-slate-300 leading-relaxed font-mono-code">
                            {trata}
                        </p>
                    </div>
                </div>

                <div class="bg-slate-950/75 border border-{color_tw}-500/25 rounded-xl p-5 mb-5">
                    <span class="text-xs font-bold font-mono-code text-white uppercase tracking-wider block mb-3">
                        Guía Educativa Paso a Paso: ¿Cómo cumplir al 100% ante una inspección?
                    </span>
                    <ul class="space-y-2.5 font-mono-code">
                        {pasos_li}
                    </ul>
                </div>

                <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3 pt-4 border-t border-white/10 text-xs font-mono-code">
                    <div class="text-slate-300">
                        <span class="text-slate-500">Instrumento en su Kit Mallen 360°:</span>
                        <strong class="text-emerald-400 ml-1">{entregable}</strong>
                    </div>
                    <a href="/#express-check" class="bg-{color_tw}-600/20 hover:bg-{color_tw}-600/40 text-{color_tw}-300 border border-{color_tw}-500/40 font-bold px-4 py-2 rounded-lg transition text-center">
                        Auditar este precepto con mi RFC
                    </a>
                </div>
            </div>"##,
            color_tw = color_tw,
            codigo = n.codigo,
            titulo_norma = n.titulo,
            pct = n.incumplimiento_pct,
            multa = n.rango_multa,
            estadistica = n.fuente_estadistica,
            historia = n.historia_dof,
            trata = n.de_que_trata,
            pasos_li = pasos_li,
            entregable = n.entregable_kit
        ));
    }

    let html = format!(
        r##"<!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{titulo} — Enciclopedia Normativa B2B | Firma Pericial Mallen</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;800&family=JetBrains+Mono:wght@400;500;600&display=swap" rel="stylesheet">
    <style>body {{ font-family: 'Inter', sans-serif; }} .font-mono-code {{ font-family: 'JetBrains Mono', monospace; }}</style>
    <link rel="stylesheet" href="/css/quantum-glass.css?v=2026.15">
    <script src="/js/quantum-lights.js?v=2026.15" defer></script>
</head>
<body class="bg-slate-950 text-slate-100 min-h-screen p-6 md:p-12 border-t-4 {border_top}">
    <div class="max-w-6xl mx-auto">
        <div class="flex flex-col sm:flex-row sm:items-center justify-between pb-6 mb-8 border-b border-white/15 gap-4">
            <div>
                <span class="text-xs font-bold text-{color_tw}-400 font-mono-code uppercase tracking-wider block">FIRMA PERICIAL MALLEN • ENCICLOPEDIA DE CUMPLIMIENTO B2B</span>
                <span class="text-xs text-slate-400 font-mono-code">{siglas}</span>
            </div>
            <div class="flex flex-wrap gap-2 text-[11px] font-mono-code">
                <a href="/normas/stps" class="px-3 py-1.5 rounded-lg bg-slate-900 border border-white/10 hover:border-blue-400 text-slate-300 transition">STPS (39 NOMs)</a>
                <a href="/normas/sat" class="px-3 py-1.5 rounded-lg bg-slate-900 border border-white/10 hover:border-emerald-400 text-slate-300 transition">SAT / CFF</a>
                <a href="/normas/imss" class="px-3 py-1.5 rounded-lg bg-slate-900 border border-white/10 hover:border-purple-400 text-slate-300 transition">IMSS / INFONAVIT</a>
                <a href="/normas/inai" class="px-3 py-1.5 rounded-lg bg-slate-900 border border-white/10 hover:border-cyan-400 text-slate-300 transition">LFPDPPP / SABG</a>
                <a href="/normas/pld" class="px-3 py-1.5 rounded-lg bg-slate-900 border border-white/10 hover:border-amber-400 text-slate-300 transition">9 Sectoriales & PLD</a>
                <a href="/normas/comercio" class="px-3 py-1.5 rounded-lg bg-slate-900 border border-white/10 hover:border-teal-400 text-slate-300 transition">PROFECO & NOM-151</a>
                <a href="/" class="px-3 py-1.5 rounded-lg bg-{color_tw}-600 text-white font-bold transition">Portal Principal</a>
            </div>
        </div>

        <div class="quantum-laser-console rounded-2xl p-8 mb-10">
            <span class="text-xs font-bold font-mono-code text-{color_tw}-400 uppercase tracking-widest block mb-2">{siglas}</span>
            <h1 class="text-2xl sm:text-4xl font-extrabold text-white mb-3 tracking-tight">{titulo}</h1>
            <p class="text-sm text-slate-300 font-mono-code mb-4">{subtitulo}</p>
            <p class="text-xs text-slate-400 font-mono-code leading-relaxed border-t border-white/10 pt-4">
                {resumen_ejecutivo}
            </p>
        </div>

        {modulos_html}

        <div class="quantum-card rounded-2xl p-8 text-center border border-emerald-500/30 mt-10">
            <h3 class="text-xl font-extrabold text-white mb-2">¿Desea auditar el cumplimiento de su empresa en estos preceptos?</h3>
            <p class="text-xs text-slate-300 font-mono-code max-w-2xl mx-auto mb-6">
                Ingrese su RFC y sitio web en nuestro motor pericial para cuantificar su exposición en UMAs vigentes y obtener sus 10 Certificados y 7 Instrumentos Jurídicos personalizados.
            </p>
            <a href="/#express-check" class="inline-block bg-gradient-to-r from-blue-600 to-emerald-600 hover:from-blue-500 hover:to-emerald-500 text-white font-bold text-xs font-mono-code px-8 py-4 rounded-xl transition uppercase tracking-wider shadow-lg shadow-emerald-600/20">
                Ejecutar Diagnóstico Pericial 360° con mi RFC
            </a>
        </div>
    </div>
</body>
</html>"##,
        titulo = titulo,
        border_top = border_top,
        color_tw = color_tw,
        siglas = siglas,
        subtitulo = subtitulo,
        resumen_ejecutivo = resumen_ejecutivo,
        modulos_html = modulos_html
    );

    Html(html)
}
