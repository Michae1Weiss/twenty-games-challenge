If you hosting a game on GitHub create a branch **gh-pages** and add **.nojekyll** file at its root!

```bash
# from project root
git checkout --orphan gh-pages
git rm -rf .
touch .nojekyll
touch index.html
# add content to index.html, it's your landing page
git add .nojekyll index.html
git commit -m "initialize gh-pages"
git push -u origin gh-pages
git checkout main
```

## Push to gh-pages
```bash
SLUG=breakout
SRC=target/bevy_web/web-release/breakout
DEST=../gh-pages/$SLUG
rm -rI "$DEST"
cp -r "$SRC" "$DEST"
( cd ../gh-pages && git add "$SLUG" && git commit -m "ops: deploy $SLUG" && git push )
```

## Links
[**docs.github.com** - gh-pages](https://docs.github.com/en/pages/getting-started-with-github-pages/configuring-a-publishing-source-for-your-github-pages-site)
