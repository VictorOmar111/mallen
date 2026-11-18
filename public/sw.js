// FIRMA MALLEN® — Lightweight Non-Blocking PWA Service Worker (v90)
// Usa estrategia Network-Only / Pass-Through para nunca congelar ni interceptar la carga de la página
const CACHE_NAME = "mallen-pwa-v90";

self.addEventListener("install", () => {
  self.skipWaiting();
});

self.addEventListener("activate", (event) => {
  event.waitUntil(
    caches.keys().then((keys) =>
      Promise.all(keys.map((k) => caches.delete(k)))
    ).then(() => self.clients.claim())
  );
});

self.addEventListener("fetch", () => {
  // Dejar que el navegador maneje todas las peticiones directamente a máxima velocidad sin bloqueos de caché
  return;
});
