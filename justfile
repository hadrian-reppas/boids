build:
	wasm-pack build --target web

deploy:
	wasm-pack build --target web
	rm pkg/.gitignore
	git checkout gh-pages
	git add .
	git commit -m "deploy"
	git push
	echo "*" >> pkg/.gitignore
	git checkout main
