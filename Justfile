vue_version         := "3.5.10"
bulma_version       := "1.0.2"
fontawesome_version := "5.15.4"
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
    wget https://cdn.jsdelivr.net/npm/bulma@{{bulma_version}}/css/bulma.css.map -O {{public_dir}}/styles/bulma.css.map

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

deploy: prepare-deployment
    wrangler pages deploy {{public_dir}} --project-name=quranize
    just get-vue get-bulma 2> /dev/null

prepare-deployment: build-wasm get-fontawesome
    wget https://cdn.jsdelivr.net/npm/vue@{{vue_version}}/dist/vue.esm-browser.prod.js -O {{public_dir}}/scripts/vue.esm-browser.js
    wget https://cdn.jsdelivr.net/npm/bulma@{{bulma_version}}/css/bulma.min.css -O {{public_dir}}/styles/bulma.css
