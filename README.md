# Twenty Games Challenge
Me trying to complete [The 20 Games Challenge](https://20gameschallenge.org/)

## Game 01 - Breakout
### Start a game in debug mode (with dynamic linking on)
```bash
cargo dev -p game_01_breakout
```
> Note: `cargo dev` is an alias for `cargo run --feature dev`

## How to
### Convert MP3 to OGG
```console
# Linux
ffmpeg -i input.mp3 -c:a libvorbis -q:a 4 output.ogg --enable-libvorbis
# macOS
brew install ffmpeg
# Example (macOS)
ffmpeg -i game_01_breakout/audio-source/music/game-music.mp3 -c:a vorbis -strict -2 -q:a 4 game_01_breakout/assets/music/game-music.ogg
```
