build:
	wasm-pack build --target web

deploy:
	git branch -D gh-pages
	git branch gh-pages
	git checkout gh-pages
	wasm-pack build --target web
	rm pkg/.gitignore
	git add .
	git commit -m "deploy" || true
	git push --set-upstream origin gh-pages --force
	git checkout main
