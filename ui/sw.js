// Najpierw sieć (zawsze świeża wersja), z pamięci tylko gdy brak internetu.
const PAMIEC = 'biuro-filar';

self.addEventListener('install', () => self.skipWaiting());
self.addEventListener('activate', (e) => e.waitUntil(self.clients.claim()));

self.addEventListener('fetch', (e) => {
  if (e.request.method !== 'GET' || new URL(e.request.url).origin !== location.origin) return;
  e.respondWith(
    fetch(e.request)
      .then((odp) => {
        const kopia = odp.clone();
        caches.open(PAMIEC).then((c) => c.put(e.request, kopia));
        return odp;
      })
      .catch(() => caches.match(e.request))
  );
});
