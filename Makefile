project = capman-rs

main : build copy
test : main serve
prepare : main zip

build :
	cargo build --target wasm32-unknown-unknown --profile wasm-release

copy :
	mkdir -p  ./html/assets/sprites ./html/assets/fonts
	cp -r ./assets/html/index.html ./html/
	cp -r ./assets/sprites/* ./html/assets/sprites/
	cp -r ./assets/fonts/* ./html/assets/fonts/
	wasm-bindgen --no-typescript --target web \
                 --out-dir ./html/ \
                 --out-name "$(project)" \
                 ./target/wasm32-unknown-unknown/wasm-release/*.wasm
	wasm-opt -Oz --output ./html/$(project)_bg.wasm ./html/$(project)_bg.wasm
	echo "Total size: `gzip -9 < ./html/$(project)_bg.wasm  | wc -c`"

zip :
	zip -r $(project).zip ./html/sprites/assets ./html/$(project).wasm ./html/*.js ./html/index.html 

# Spin-up a container serving the app
serve :
	docker rm $(project)_nginx -f
	docker run --rm --name $(project)_nginx \
	      -p 8080:80 \
	      -v ./html/:/usr/share/nginx/html:ro \
	      -d nginx
	xdg-open http://127.0.0.1:8080/index.html
