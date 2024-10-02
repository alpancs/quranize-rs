const cacheKey = "quranize-sw-v4";

self.addEventListener("install", function () { self.skipWaiting(); });

self.addEventListener("fetch", function (event) {
    const { request } = event;
    const { url } = request;
    if (url.startsWith("https://") || url.startsWith("http://")) {
        event.respondWith(fetch(request)
            .then(response => {
                const clonedResponse = response.clone();
                caches.open(cacheKey).then(cache => cache.put(request, clonedResponse));
                return response;
            })
            .catch(error => caches.open(cacheKey)
                .then(cache => cache.match(request))
                .then(response => response ?? Promise.reject(error))
            )
        );
    }
});
