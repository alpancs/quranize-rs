run-app-web:
    cd app/web && npm run dev

build-wasm:
    rm -rf app/web/src/workers/quranize/quranize-wasm
    wasm-pack build \
        --target=web \
        --release \
        --out-dir=../app/web/src/workers/quranize/quranize-wasm \
        quranize-wasm

build-app-web:
    cd app/web && npm ci && npm run build

prepare-deployment: build-wasm build-app-web

deploy: prepare-deployment
    wrangler pages deploy app/web/dist --project-name=quranize
