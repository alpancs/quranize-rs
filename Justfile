vue_version         := "3.4.21"
bulma_version       := "0.9.4"
fontawesome_version := "6.5.1"

build-web-app: build-quranize-wasm get-vue get-bulma get-fontawesome

build-quranize-wasm:
    cd web-app && wasm-pack build \
        --release \
        --target=web \
        --no-typescript \
        --out-dir=web-app/scripts/quranize \
        --out-name=quranize

get-vue:
    wget https://cdn.jsdelivr.net/npm/vue@{{vue_version}}/dist/vue.esm-browser.js -O web-app/web-app/scripts/vue.esm-browser.js

get-bulma:
    wget https://cdn.jsdelivr.net/npm/bulma@{{bulma_version}}/css/bulma.css -O web-app/web-app/styles/bulma.css

get-fontawesome:
    wget https://use.fontawesome.com/releases/v{{fontawesome_version}}/fontawesome-free-{{fontawesome_version}}-web.zip
    unzip fontawesome-free-{{fontawesome_version}}-web.zip
    rm -rf fontawesome-free-{{fontawesome_version}}-web.zip web-app/web-app/styles/fontawesome
    mv fontawesome-free-{{fontawesome_version}}-web web-app/web-app/styles/fontawesome

run-server:
    static-web-server \
        --root=web-app/web-app \
        --port=5000 \
        --cache-control-headers=false \
        --log-level=info
