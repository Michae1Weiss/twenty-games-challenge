If you hosting a game on GitHub create a branch **gh-pages** and add **.nojekyll** file at its root!

```bash
git worktree add ../gh-pages gh-pages
touch .nojekyll
echo "This file makes sure that Github Pages doesn't process mdBook's output." >> .nojekyll
```

```yaml
      - name: 🔗 Symlink Latest Build
        run: |
          mkdir latest-symlink-dir
          ln -s ./${{ github.ref_name }} latest
          mv latest latest-symlink-dir
```

## Links
[**docs.github.com** - gh-pages](https://docs.github.com/en/pages/getting-started-with-github-pages/configuring-a-publishing-source-for-your-github-pages-site)
