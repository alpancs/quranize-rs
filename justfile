# build quranize wasm
wasm:
	cd web-app && wasm-pack build \
		--release \
		--target=web \
		--no-typescript \
		--out-dir=public/scripts/quranize \
		--out-name=quranize
