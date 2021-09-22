watch:
	fswatch -o -r ./src|xargs -I {} wasm-pack build --target web --out-name wasm --out-dir ./dist
start:
	cd dist && python -m SimpleHTTPServer 8080


