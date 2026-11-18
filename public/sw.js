const CACHE_NAME = "mallen-pwa-v89";
const CORE_ASSETS = [
  "/",
  "/normativa",
  "/radar-dof",
  "/css/enterprise-linear.css",
  "/css/mallen-high-impact.css",
  "/js/quantum-lights.js",
  "/icons/icon-192.svg",
  "/icons/icon-512.svg"
];

self.addEventListener("install", (event) => {
  self.skipWaiting();
  event.waitUntil(
    caches.open(CACHE_NAME).then((cache) => cache.addAll(CORE_ASSETS).catch(() => {}))
  );
});

self.addEventListener("activate", (event) => {
  event.waitUntil(
    caches.keys().then((keys) =>
      Promise.all(keys.filter((k) => k !== CACHE_NAME).map((k) => caches.delete(k)))
    ).then(() => self.clients.claim())
  );
});

self.addEventListener("fetch", (event) => {
  if (event.request.method !== "GET") return;
  const url = new URL(event.request.url);
  if (url.pathname.startsWith("/api/") || url.pathname.startsWith("/download/")) return;

  event.respondWith(
    fetch(event.request)
      .then((networkResp) => {
        if (networkResp && networkResp.status === 200 && url.origin === self.location.origin) {
          const clone = networkResp.clone();
          caches.open(CACHE_NAME).then((cache) => cache.put(event.request, clone));
        }
        return networkResp;
      })
      .catch(() => caches.match(event.request))
  );
});
