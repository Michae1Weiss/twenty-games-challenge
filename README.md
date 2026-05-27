# Twenty Games Challenge
Me trying to complete [The 20 Games Challenge](https://20gameschallenge.org/)

## Try it out :)
- [Game #1 - Breakout (Web)](https://michae1weiss.github.io/twenty-games-challenge/breakout/)

## Catalog
### Game #1 - Breakout
#### Start a game in debug mode (with dynamic linking on)
```bash
cargo dev -p game_01_breakout
```
> Note: `cargo dev` is an alias for `cargo run --feature dev`

## How to
### Build for web
**Pre-requirement**: Install [bevy_cli](https://github.com/TheBevyFlock/bevy_cli). 
```bash
bevy build -p breakout --release web --bundle
# Test locally
(sleep 5 && python3 -m webbrowser -t "http://localhost:8000") &
python3 -m http.server 8000 --directory target/bevy_web/web-release/breakout
```

### Convert MP3 to OGG
```bash
# Linux
ffmpeg -i input.mp3 -c:a libvorbis -q:a 4 output.ogg --enable-libvorbis
# macOS
brew install ffmpeg
# Example (macOS)
ffmpeg -i game_01_breakout/audio-source/music/game-music.mp3 -c:a vorbis -strict -2 -q:a 4 game_01_breakout/assets/music/game-music.ogg
```
