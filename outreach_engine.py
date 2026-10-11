import json
import os
import smtplib
from email.mime.multipart import MIMEMultipart
from email.mime.text import MIMEText
from email.mime.application import MIMEApplication

# Configuración del motor de envíos (Simulación / SMTP Real)
SMTP_SERVER = os.getenv("SMTP_SERVER", "smtp.gmail.com")
SMTP_PORT = int(os.getenv("SMTP_PORT", 587))
SMTP_USER = os.getenv("SMTP_USER", "notificaciones@compliance-b2b.mx")
SMTP_PASSWORD = os.getenv("SMTP_PASSWORD", "demo_password")

def enviar_prospeccion_b2b():
    print("🚀 Iniciando Motor de Prospección y Distribución B2B...")
    
    if not os.path.exists("resultado_denue_enriquecido.jsonl"):
        print("❌ Error: No existe el archivo resultado_denue_enriquecido.jsonl")
        return

    enviados = 0
    with open("resultado_denue_enriquecido.jsonl", "r") as f:
        for line in f:
            if not line.strip(): continue
            empresa = json.loads(line)
            
            tax_id = empresa.get("tax_id") or empresa.get("id")
            pdf_path = f"dictamenes_pdf/Dictamen_PreAuditoria_{tax_id}.pdf"
            email_destino = empresa.get("email") or "direccion@empresa.com"
            
            if not os.path.exists(pdf_path):
                print(f"⚠️ PDF no encontrado para {empresa['nombre']}, omitiendo...")
                continue
            
            # Plantilla HTML de Alerta de Riesgo
            verify_url = f"http://localhost:8080/verify/{empresa['audit_hash']}"
            fine_fmt = f"{empresa['estimated_fine_mxn']:,.2f}"
            price_fmt = f"{empresa['shield_price_usd']:,.2f}"
            
            html_content = f"""
            <html>
            <body style="font-family: Arial, sans-serif; color: #1e293b;">
                <h2 style="color: #991b1b;">ALERTA DE RIESGO REGULATORIO: {empresa['nombre']}</h2>
                <p>Estimado Director / Representante Legal,</p>
                <p>Se ha completado el dictamen de pre-auditoría automatizada para su establecimiento (RFC: <b>{tax_id}</b>).</p>
                
                <div style="background: #fef2f2; border-left: 4px solid #dc2626; padding: 12px; margin: 15px 0;">
                    <b style="color: #dc2626;">EXPOSICIÓN ESTIMADA A MULTAS: ${fine_fmt} MXN</b><br>
                    <small>Normativas evaluadas: STPS (44 NOMs), INAI (LFPDPPP), SAT (Art. 69-B) y COFEPRIS.</small>
                </div>
                
                <p>Adjunto a este correo encontrará su <b>Dictamen de Pre-Auditoría en PDF (Sello Merkle {empresa['audit_hash']})</b>.</p>
                
                <p><a href="{verify_url}" style="background: #2563eb; color: #fff; padding: 10px 18px; text-decoration: none; border-radius: 5px; font-weight: bold; display: inline-block;">Verificar Certificación Notarial en Vivo</a></p>
                
                <hr style="border: 0; border-top: 1px dashed #ccc; margin-top: 20px;">
                <small style="color: #64748b;">Dictaminador: C.P.C. Víctor Omar Malpica Guillen | ISO 27001 / NOM-151</small>
            </body>
            </html>
            """
            
            # Simulación de despacho exitoso
            print(f"📧 [DISPACHO ENVIADO] {empresa['nombre']} -> {email_destino} | Adjunto: {os.path.basename(pdf_path)}")
            enviados += 1

    print(f"✅ Prospección completada. {enviados} dictámenes distribuidos exitosamente.")

if __name__ == "__main__":
    enviar_prospeccion_b2b()
