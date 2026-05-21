```bash
ffmpeg -i game_01_breakout/audio-source/music/15sec-2022-01-18_-_A_Sad_Meme_-_www.FesliyanStudios.com.mp3 -c:a vorbis -strict -2 -q:a 4 game_01_breakout/assets/music/game-over-music.ogg && ffmpeg -i game_01_breakout/audio-source/music/prettyjohn1-background-music-505061.mp3 -c:a vorbis -strict -2 -q:a 4 game_01_breakout/assets/music/menu-music.ogg
```
