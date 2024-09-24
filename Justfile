vue_version         := "3.4.21"
bulma_version       := "0.9.4"
fontawesome_version := "6.5.1"
public_dir          := "web-app/public"

build-web-app: build-wasm get-vue get-bulma get-fontawesome

build-wasm:
    rm -rf {{public_dir}}/scripts/quranize
    wasm-pack build \
        --no-typescript \
        --target=web \
        --release \
        --out-dir=public/scripts/quranize \
        --out-name=quranize \
        --no-pack \
        web-app

get-vue:
    wget https://cdn.jsdelivr.net/npm/vue@{{vue_version}}/dist/vue.esm-browser.js -O {{public_dir}}/scripts/vue.esm-browser.js

get-bulma:
    wget https://cdn.jsdelivr.net/npm/bulma@{{bulma_version}}/css/bulma.css -O {{public_dir}}/styles/bulma.css

get-fontawesome:
    rm -rf {{public_dir}}/styles/fontawesome
    wget https://use.fontawesome.com/releases/v{{fontawesome_version}}/fontawesome-free-{{fontawesome_version}}-web.zip
    unzip -q fontawesome-free-{{fontawesome_version}}-web.zip && rm -rf fontawesome-free-{{fontawesome_version}}-web.zip
    mv fontawesome-free-{{fontawesome_version}}-web {{public_dir}}/styles/fontawesome

run-server:
    static-web-server \
        --root={{public_dir}} \
        --port=5000 \
        --cache-control-headers=false \
        --log-level=info
