build:
	wasm-pack build --target web

deploy:
	git checkout gh-pages
	wasm-pack build --target web
	rm pkg/.gitignore
	git add .
	git commit -m "deploy"
	git push
	echo "*" >> pkg/.gitignore
	git checkout main
