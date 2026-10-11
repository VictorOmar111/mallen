#!/bin/bash

echo "🚀 Compilando worker_inegi_engine en modo RELEASE..."
cargo build --release --bin server

if [ $? -eq 0 ]; then
    echo "✅ Compilacion exitosa. Deteniendo instancias previas..."
    pkill -f "target/release/server" || true
    
    echo "🌐 Iniciando Servidor Web en segundo plano en el puerto 8080..."
    nohup ./target/release/server > server.log 2>&1 &
    
    echo "🎉 Servidor escuchando. Puedes revisar el log con: tail -f server.log"
else
    echo "❌ Error al compilar el servidor."
fi
