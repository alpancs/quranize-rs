run-app-web:
    cd app/web && npm run dev

build-wasm:
    wasm-pack build --target=web --release quranize-wasm
    rm -rf app/web/src/workers/quranize/quranize-wasm
    mv quranize-wasm/pkg app/web/src/workers/quranize/quranize-wasm

build-app-web:
    cd app/web && npm ci && npm run build

prepare-deployment: build-wasm build-app-web

deploy: prepare-deployment
    wrangler pages deploy app/web/dist --project-name=quranize
