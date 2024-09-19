# build quranize wasm
wasm:
	cd web-app && wasm-pack build \
		--release \
		--target=web \
		--no-typescript \
		--out-dir=public/scripts/quranize \
		--out-name=quranize

# run static web server
server:
	static-web-server \
		--root=web-app/public \
		--port=8080 \
		--log-level=info \
		--cache-control-headers=false
