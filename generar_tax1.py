import os
from reportlab.lib.pagesizes import letter
from reportlab.lib import colors
from reportlab.platypus import SimpleDocTemplate, Paragraph, Spacer, Table, TableStyle
from reportlab.lib.styles import getSampleStyleSheet, ParagraphStyle
from reportlab.graphics.shapes import Drawing
from reportlab.graphics.charts.piecharts import Pie

os.makedirs('public/TAX_1', exist_ok=True)
pdf_filename = 'public/TAX_1/Dictamen_Ejecutivo_TAX_1.pdf'

doc = SimpleDocTemplate(pdf_filename, pagesize=letter)
story = []
styles = getSampleStyleSheet()

title_style = ParagraphStyle('TitleStyle', parent=styles['Heading1'], fontSize=16, leading=20, textColor=colors.HexColor('#0F172A'))
subtitle_style = ParagraphStyle('SubTitleStyle', parent=styles['Normal'], fontSize=10, leading=14, textColor=colors.HexColor('#475569'))

story.append(Paragraph('<b>FIRMA PERICIAL MALLEN</b> — DICTAMEN EJECUTIVO', title_style))
story.append(Paragraph('Módulo: TAX_1 | Evaluación de Cumplimiento Fiscal y Prevención de Riesgos', subtitle_style))
story.append(Spacer(1, 15))

# Gráfico de Dona de Riesgo
d = Drawing(180, 180)
pc = Pie()
pc.x = 15
pc.y = 15
pc.width = 150
pc.height = 150
pc.data = [85, 15]
pc.labels = ['85% Riesgo', '15% Cumplimiento']
pc.slices[0].fillColor = colors.HexColor('#DC2626')
pc.slices[1].fillColor = colors.HexColor('#059669')
pc.innerRadiusFraction = 0.55
d.add(pc)
story.append(d)
story.append(Spacer(1, 15))

# Matriz de Sanciones TAX_1
data_tabla = [
    ['Vector Regulatorio', 'Fundamento', 'Sanción Máxima (UMA)', 'Riesgo Estimado (MXN)'],
    ['SAT Materialidad', 'Art. 69-B CFF', '75% Facturado', 'Cancelación CSD + Penal'],
    ['STPS NOM-035', 'Art. 513 LFT', '5,000 UMA', '$542,850.00 MXN'],
    ['IMSS Prima Riesgo', 'Art. 72 LSS', '350 UMA', 'Capitales Constitutivos']
]
t = Table(data_tabla, colWidths=[130, 100, 110, 140])
t.setStyle(TableStyle([
    ('BACKGROUND', (0,0), (-1,0), colors.HexColor('#0F172A')),
    ('TEXTCOLOR', (0,0), (-1,0), colors.white),
    ('FONTSIZE', (0,0), (-1,-1), 8),
    ('GRID', (0,0), (-1,-1), 0.5, colors.HexColor('#CBD5E1')),
    ('ALIGN', (0,0), (-1,-1), 'CENTER')
]))
story.append(t)
doc.build(story)

print("✅ Dictamen Ejecutivo TAX_1 listo en public/TAX_1/")
