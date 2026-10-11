import os
from reportlab.lib.pagesizes import letter
from reportlab.lib import colors
from reportlab.platypus import SimpleDocTemplate, Paragraph, Spacer, Table, TableStyle
from reportlab.lib.styles import getSampleStyleSheet, ParagraphStyle
from reportlab.graphics.shapes import Drawing
from reportlab.graphics.charts.piecharts import Pie

os.makedirs('dictamenes_pdf', exist_ok=True)

rfcs = ['LMX150720KLM', 'LNO080512XYZ', 'TCE990101789', 'TIM120315ABC', 'TST200101QWE']

for rfc in rfcs:
    pdf_filename = f'dictamenes_pdf/Dictamen_PreAuditoria_{rfc}.pdf'
    doc = SimpleDocTemplate(pdf_filename, pagesize=letter)
    story = []
    styles = getSampleStyleSheet()
    
    title_style = ParagraphStyle('TitleStyle', parent=styles['Heading1'], fontSize=16, leading=20, textColor=colors.HexColor('#0F172A'))
    body_style = ParagraphStyle('BodyStyle', parent=styles['Normal'], fontSize=9, leading=13, textColor=colors.HexColor('#334155'))
    
    story.append(Paragraph(f'<b>FIRMA PERICIAL MALLEN</b> — DICTAMEN PERICIAL', title_style))
    story.append(Paragraph(f'Referencia Auditada: <b>{rfc}</b> | Validez NOM-151-SCFI-2016', body_style))
    story.append(Spacer(1, 15))
    
    # Gráfica de Dona (Pie Chart de Riesgo)
    d = Drawing(160, 160)
    pc = Pie()
    pc.x = 10
    pc.y = 10
    pc.width = 140
    pc.height = 140
    pc.data = [85, 15]
    pc.labels = ['85% Riesgo', '15% Cumplimiento']
    pc.slices[0].fillColor = colors.HexColor('#DC2626')  # Rojo
    pc.slices[1].fillColor = colors.HexColor('#059669')  # Verde esmeralda
    pc.innerRadiusFraction = 0.55                        # Hueco central (Dona)
    d.add(pc)
    story.append(d)
    story.append(Spacer(1, 15))
    
    data_tabla = [
        ['Vector Regulatorio', 'Sanción Máxima (UMA)', 'Riesgo Estimado (MXN)'],
        ['STPS NOM-035 / NOM-037', '5,000 UMA', '$542,850.00 MXN'],
        ['SAT Art. 69-B CFF', '75% Facturado', 'Cancelación CSD'],
        ['IMSS Art. 72 LSS', '350 UMA', 'Capitales Constitutivos']
    ]
    t = Table(data_tabla, colWidths=[180, 130, 140])
    t.setStyle(TableStyle([
        ('BACKGROUND', (0,0), (-1,0), colors.HexColor('#0F172A')),
        ('TEXTCOLOR', (0,0), (-1,0), colors.white),
        ('FONTSIZE', (0,0), (-1,-1), 8),
        ('GRID', (0,0), (-1,-1), 0.5, colors.HexColor('#CBD5E1')),
        ('ALIGN', (0,0), (-1,-1), 'CENTER')
    ]))
    story.append(t)
    doc.build(story)

print("✅ Documentos PDF de prueba generados exitosamente en dictamenes_pdf/")
