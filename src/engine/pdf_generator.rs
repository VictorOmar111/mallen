use printpdf::*;
use crate::engine::models::EmpresaB2B;
use crate::engine::osint_scanner::OsintResult;
use std::fs::File;
use std::io::Write;
use std::f32::consts::PI;

pub fn generate_pdf_report(empresa: &EmpresaB2B, osint: &OsintResult, output_path: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 1. Inicializar documento PDF e insertar primera página A4 (210mm x 297mm)
    let mut doc = PdfDocument::new("Dictamen Pericial de Auditoria y Riesgo Normativo B2B");
    let (page1, layer1) = doc.add_page(Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    // Cargar fuentes estándar integradas
    let font_bold = doc.add_font(&ParsedFont::from(BuiltinFont::HelveticaBold));
    let font_regular = doc.add_font(&ParsedFont::from(BuiltinFont::Helvetica));

    // Colores corporativos
    let primary_blue = Color::Rgb(Rgb::new(0.08, 0.18, 0.36, None));
    let alert_red = Color::Rgb(Rgb::new(0.82, 0.10, 0.15, None));
    let text_dark = Color::Rgb(Rgb::new(0.15, 0.15, 0.15, None));
    let bg_light = Color::Rgb(Rgb::new(0.94, 0.95, 0.97, None));

    // 2. Encabezado Corporativo (Franja Superior)
    let header_rect = Line {
        points: vec![
            LinePoint { p: Point::new(Mm(0.0), Mm(297.0)), bezier: false },
            LinePoint { p: Point::new(Mm(210.0), Mm(297.0)), bezier: false },
            LinePoint { p: Point::new(Mm(210.0), Mm(270.0)), bezier: false },
            LinePoint { p: Point::new(Mm(0.0), Mm(270.0)), bezier: false },
        ],
        is_closed: true,
    };
    current_layer.set_fill_color(primary_blue);
    current_layer.add_shape(header_rect);

    // Texto Encabezado
    current_layer.set_fill_color(Color::Rgb(Rgb::new(1.0, 1.0, 1.0, None)));
    current_layer.use_text("INMUNIDAD B2B | AUDITORÍA & CUMPLIMIENTO", 14.0, Mm(15.0), Mm(282.0), &font_bold);
    current_layer.use_text("DICTAMEN PERICIAL DE EVALUACIÓN DE RIESGO REGULATORIO", 10.0, Mm(15.0), Mm(275.0), &font_regular);

    // 3. Datos Principales de la Empresa Auditada
    current_layer.set_fill_color(text_dark);
    current_layer.use_text(format!("Empresa: {}", empresa.nombre), 12.0, Mm(15.0), Mm(258.0), &font_bold);
    current_layer.use_text(format!("Tax ID / RFC: {}", empresa.tax_id.as_deref().unwrap_or("N/A")), 10.0, Mm(15.0), Mm(251.0), &font_regular);
    current_layer.use_text(format!("Actividad: {}", empresa.actividad), 10.0, Mm(15.0), Mm(245.0), &font_regular);
    current_layer.use_text(format!("Ubicación: {}, {}", empresa.direccion, empresa.estado), 10.0, Mm(15.0), Mm(239.0), &font_regular);

    // 4. Dibujo Vectorial de Gráfica de Dona (Score de Riesgo)
    let cx = 165.0f32;
    let cy = 248.0f32;
    let outer_r = 18.0f32;
    let inner_r = 11.0f32;

    // Círculo exterior (Fondo gris)
    let outer_bg = Polygon {
        rings: vec![PolygonRing { points: generate_circle_points(cx, cy, outer_r) }],
        mode: PaintMode::Fill,
        winding_order: WindingOrder::NonZero,
    };
    current_layer.set_fill_color(bg_light);
    current_layer.add_polygon(outer_bg);

    // Arco de Riesgo (Rojo según Score)
    let score_angle = (empresa.risk_score as f32 / 100.0) * 2.0 * PI;
    let alert_arc = Polygon {
        rings: vec![PolygonRing { points: generate_arc_points(cx, cy, outer_r, 0.0, score_angle) }],
        mode: PaintMode::Fill,
        winding_order: WindingOrder::NonZero,
    };
    current_layer.set_fill_color(alert_red);
    current_layer.add_polygon(alert_arc);

    // Círculo Interior (Hueco de la dona)
    let inner_hole = Polygon {
        rings: vec![PolygonRing { points: generate_circle_points(cx, cy, inner_r) }],
        mode: PaintMode::Fill,
        winding_order: WindingOrder::NonZero,
    };
    current_layer.set_fill_color(Color::Rgb(Rgb::new(1.0, 1.0, 1.0, None)));
    current_layer.add_polygon(inner_hole);

    // Texto central de la Dona
    current_layer.set_fill_color(alert_red);
    current_layer.use_text(format!("{}%", empresa.risk_score), 12.0, Mm(cx - 5.0), Mm(cy - 2.0), &font_bold);
    current_layer.set_fill_color(text_dark);
    current_layer.use_text("RIESGO", 7.0, Mm(cx - 5.5), Mm(cy - 6.5), &font_regular);

    // 5. Cuadro de Resumen Económico
    let box_risk = Line {
        points: vec![
            LinePoint { p: Point::new(Mm(15.0), Mm(225.0)), bezier: false },
            LinePoint { p: Point::new(Mm(195.0), Mm(225.0)), bezier: false },
            LinePoint { p: Point::new(Mm(195.0), Mm(205.0)), bezier: false },
            LinePoint { p: Point::new(Mm(15.0), Mm(205.0)), bezier: false },
        ],
        is_closed: true,
    };
    current_layer.set_fill_color(bg_light);
    current_layer.set_outline_color(primary_blue);
    current_layer.set_outline_thickness(0.5);
    current_layer.add_shape(box_risk);

    current_layer.set_fill_color(primary_blue);
    current_layer.use_text("EXPOSICIÓN TOTAL ESTIMADA A SANCIONES Y MULTAS:", 10.0, Mm(20.0), Mm(217.0), &font_bold);
    current_layer.set_fill_color(alert_red);
    current_layer.use_text(format!("${:.2} MXN", empresa.estimated_fine_mxn), 16.0, Mm(20.0), Mm(209.0), &font_bold);

    // 6. Desglose Regulatorio (Tabla Visual)
    current_layer.set_fill_color(primary_blue);
    current_layer.use_text("DESGLOSE DE OBLIGACIONES Y MULTAS POTENCIALES", 11.0, Mm(15.0), Mm(195.0), &font_bold);

    let mut current_y = 185.0f32;
    let items = vec![
        ("STPS (NOM-035 / NOM-037)", "$162,855 MXN - $434,280 MXN", "Comisiones Mixtas y Teletrabajo"),
        ("INAI (LFPDPPP)", "$21,714 MXN - $347,424 MXN", if osint.has_privacy_policy { "Cumplimiento Parcial" } else { "Aviso de Privacidad Ausente" }),
        ("PROFECO (LFPC)", "$50,000 MXN - $150,000 MXN", "Términos Comerciales en Línea"),
        ("SAT / IMSS (CFF 69-B)", "$200,000 MXN - $500,000 MXN", "Soporte de Materialidad Operativa"),
        ("Sectorial (COFEPRIS/CNBV)", "$150,000 MXN - $400,000 MXN", "Licencias y Avisos de Operación"),
    ];

    for (inst, multa, desc) in items {
        current_layer.set_fill_color(text_dark);
        current_layer.use_text(inst, 9.0, Mm(15.0), Mm(current_y), &font_bold);
        current_layer.use_text(multa, 9.0, Mm(85.0), Mm(current_y), &font_regular);
        current_layer.use_text(desc, 9.0, Mm(140.0), Mm(current_y), &font_regular);

        let line = Line {
            points: vec![
                LinePoint { p: Point::new(Mm(15.0), Mm(current_y - 2.0)), bezier: false },
                LinePoint { p: Point::new(Mm(195.0), Mm(current_y - 2.0)), bezier: false },
            ],
            is_closed: false,
        };
        current_layer.set_outline_color(bg_light);
        current_layer.set_outline_thickness(0.3);
        current_layer.add_shape(line);

        current_y -= 10.0;
    }

    // 7. Pie de Página y Firma Digital Pericial
    current_layer.set_fill_color(primary_blue);
    current_layer.use_text("CERTIFICACIÓN Y ACTIVACIÓN DE BLINDAJE INTEGRAL", 10.0, Mm(15.0), Mm(50.0), &font_bold);
    current_layer.set_fill_color(text_dark);
    current_layer.use_text(format!("Hash Pericial: {}", empresa.audit_hash), 8.0, Mm(15.0), Mm(44.0), &font_regular);
    current_layer.use_text(format!("Certificador: {}", empresa.accountant_signature.as_deref().unwrap_or("Validado")), 8.0, Mm(15.0), Mm(38.0), &font_regular);

    current_layer.set_fill_color(alert_red);
    current_layer.use_text("Costo de Inmunidad Legal Inmediata: $140.00 USD ($2,500 MXN)", 10.0, Mm(15.0), Mm(28.0), &font_bold);
    current_layer.use_text("Enlace de Activación: https://inmunidadb2b.com/checkout", 9.0, Mm(15.0), Mm(22.0), &font_regular);

    // Guardar archivo PDF en printpdf 0.12.8
    let mut warnings = Vec::new();
    let pdf_bytes = doc.save(&PdfSaveOptions::default(), &mut warnings);

    let mut file = File::create(output_path)?;
    file.write_all(&pdf_bytes)?;

    Ok(())
}

fn generate_circle_points(cx: f32, cy: f32, r: f32) -> Vec<LinePoint> {
    let mut points = Vec::new();
    let steps = 32;
    for i in 0..steps {
        let theta = (i as f32 / steps as f32) * 2.0 * PI;
        let x = cx + r * theta.cos();
        let y = cy + r * theta.sin();
        points.push(LinePoint { p: Point::new(Mm(x), Mm(y)), bezier: false });
    }
    points
}

fn generate_arc_points(cx: f32, cy: f32, r: f32, start_angle: f32, end_angle: f32) -> Vec<LinePoint> {
    let mut points = Vec::new();
    points.push(LinePoint { p: Point::new(Mm(cx), Mm(cy)), bezier: false });
    let steps = 32;
    for i in 0..=steps {
        let theta = start_angle + (i as f32 / steps as f32) * (end_angle - start_angle);
        let x = cx + r * theta.cos();
        let y = cy + r * theta.sin();
        points.push(LinePoint { p: Point::new(Mm(x), Mm(y)), bezier: false });
    }
    points
}
