import json
import os
import time
import smtplib
from email.mime.multipart import MIMEMultipart
from email.mime.text import MIMEText

# Configuración SMTP (Recomendado: AWS SES / Postmark / Resend con dominio satélite)
SMTP_SERVER = os.getenv("SMTP_SERVER", "email-smtp.us-east-1.amazonaws.com")
SMTP_PORT = int(os.getenv("SMTP_PORT", 587))
SMTP_USER = os.getenv("SMTP_USER", "")
SMTP_PASSWORD = os.getenv("SMTP_PASSWORD", "")
SENDER_EMAIL = os.getenv("SENDER_EMAIL", "notificaciones@auditoria-mallen.mx")
BASE_URL = os.getenv("MALLEN_BASE_URL", "https://mallen.lat")
DRY_RUN = os.getenv("DRY_RUN", "true").lower() == "true"

def construir_mensaje_correo(empresa: dict) -> MIMEMultipart:
    tax_id = empresa.get("tax_id") or empresa.get("id", "SIN-RFC")
    nombre = empresa.get("nombre", "Representante Legal")
    email_destino = empresa.get("email", "direccion@empresa.com")
    audit_hash = empresa.get("audit_hash", "pending")
    fine_fmt = f"{empresa.get('estimated_fine_mxn', 0.0):,.2f}"
    
    verify_url = f"{BASE_URL}/verify/{audit_hash}?rfc={tax_id}"

    msg = MIMEMultipart("alternative")
    msg["Subject"] = f"Diagnóstico de Cumplimiento Público — {nombre} ({tax_id})"
    msg["From"] = f"C.P.C. Víctor Omar Malpica | Firma Pericial Mallen <{SENDER_EMAIL}>"
    msg["To"] = email_destino
    # Header List-Unsubscribe obligatorio desde 2024+ para no caer en SPAM de Google/Microsoft
    msg["List-Unsubscribe"] = f"<{BASE_URL}/unsubscribe?email={email_destino}>"

    text_content = (
        f"Estimado Representante Legal de {nombre},\n\n"
        f"Se ha generado el pre-diagnóstico automatizado de cumplimiento normativo sobre registros públicos para {tax_id}.\n"
        f"Exposición estimada por brechas visibles (STPS/INAI/PROFECO/SAT): ${fine_fmt} MXN.\n\n"
        f"Consulte y descargue su expediente técnico con sello criptográfico SHA-256 en:\n{verify_url}\n\n"
        f"Atentamente,\nC.P.C. Víctor Omar Malpica Guillén | Firma Pericial Mallen"
    )

    html_content = f"""
    <html>
    <body style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Arial, sans-serif; color: #1e293b; line-height: 1.6; max-width: 620px; margin: 0 auto;">
        <div style="border-bottom: 2px solid #0f172a; padding-bottom: 10px; margin-bottom: 20px;">
            <strong style="color: #0f172a; font-size: 16px;">FIRMA PERICIAL MALLEN — AUDITORÍA B2B</strong>
        </div>
        
        <p>Estimado Director General / Representante Legal de <b>{nombre}</b>,</p>
        <p>Como parte de nuestro monitoreo de cumplimiento corporativo sobre fuentes públicas (INEGI DENUE, portales digitales e indicadores STPS/INAI/SAT), hemos identificado omisiones técnicas visibles vinculadas al registro <b>{tax_id}</b>.</p>
        
        <div style="background: #fff1f2; border-left: 4px solid #e11d48; padding: 14px; margin: 18px 0; border-radius: 4px;">
            <span style="color: #be123c; font-weight: bold; font-size: 15px;">CONTINGENCIA ESTIMADA EN MULTAS: ${fine_fmt} MXN</span><br>
            <small style="color: #475569;">Rubros evaluados: NOMs STPS aplicables por giro SCIAN, Aviso de Privacidad LFPDPPP, Comercio Electrónico LFPC y Materialidad Art. 69-B CFF.</small>
        </div>
        
        <p>Para proteger la seguridad informática de su servidor de correo, no adjuntamos archivos binarios. Puede visualizar su <b>Pre-Dictamen Pericial (Hash SHA-256: <code>{audit_hash[:12]}...</code>)</b> directamente en nuestra bóveda verificada:</p>
        
        <p style="margin: 24px 0;">
            <a href="{verify_url}" style="background: #0f172a; color: #ffffff; padding: 12px 22px; text-decoration: none; border-radius: 6px; font-weight: 600; display: inline-block;">
                Consultar Pre-Dictamen y Sello de Tiempo
            </a>
        </p>
        
        <hr style="border: 0; border-top: 1px solid #e2e8f0; margin-top: 28px;">
        <small style="color: #64748b; display: block;">
            Dictaminador: C.P.C. Víctor Omar Malpica Guillén | Evidencia Criptográfica SHA-256 / Fecha Cierta.<br>
            Este es un informe preventivo privado basado en información pública (OSINT). Si desea excluir su dominio de futuros diagnósticos, <a href="{BASE_URL}/unsubscribe?email={email_destino}" style="color: #64748b;">haga clic aquí</a>.
        </small>
    </body>
    </html>
    """

    msg.attach(MIMEText(text_content, "plain", "utf-8"))
    msg.attach(MIMEText(html_content, "html", "utf-8"))
    return msg

def enviar_prospeccion_b2b():
    print("🚀 Iniciando Motor de Prospección y Distribución B2B...")
    archivo_entrada = "resultado_denue_enriquecido.jsonl"
    
    if not os.path.exists(archivo_entrada):
        print(f"❌ Error: No existe el archivo {archivo_entrada}")
        return

    enviados = 0
    server = None

    if not DRY_RUN and SMTP_USER and SMTP_PASSWORD:
        server = smtplib.SMTP(SMTP_SERVER, SMTP_PORT, timeout=15)
        server.starttls()
        server.login(SMTP_USER, SMTP_PASSWORD)

    try:
        with open(archivo_entrada, "r", encoding="utf-8") as f:
            for line in f:
                if not line.strip():
                    continue
                empresa = json.loads(line)
                email_destino = empresa.get("email")
                if not email_destino or "@" not in email_destino:
                    continue

                msg = construir_mensaje_correo(empresa)

                if DRY_RUN or server is None:
                    print(f"🧪 [DRY-RUN] {empresa.get('nombre')} -> {email_destino} | Hash: {empresa.get('audit_hash', '')[:10]}")
                else:
                    server.send_message(msg)
                    print(f"📧 [ENVIADO REAL] {empresa.get('nombre')} -> {email_destino}")
                    time.sleep(1.2)  # Rate-limiting para proteger la reputación SMTP

                enviados += 1
    finally:
        if server:
            server.quit()

    print(f"✅ Prospección completada. {enviados} notificaciones procesadas.")

if __name__ == "__main__":
    enviar_prospeccion_b2b()
