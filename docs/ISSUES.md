## 1. Ribbon ball effect: wgpu error: Validation Error
```console
thread '<unnamed>' (27984332) panicked at /Users/mishabeliy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wgpu-27.0.1/src/backend/wgpu_core.rs:2568:18:
wgpu error: Validation Error

Caused by:
  In a CommandEncoder
    In a set_bind_group command
      Dynamic binding offset index 1 with offset 256 would overrun the buffer bound to BindGroup with 'hanabi:bind_group:util_20_2' label 0 -> binding 1. Buffer size is 256 bytes, the binding binds bytes 0..256, meaning the maximum the binding can be offset is 0 bytes


note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Encountered a panic in system `bevy_render::renderer::render_system`!
```

### Steps to reproduce
1. Use commit state a09c077
2. Start the game
3. Click **Play**, then **Esc** in the pause menu and go to the main menu.
4. Click **Play** again - voilá.

### Related github issues:
- [**github.com/djeedai/bevy_hanabi/issues** - Panic when spawning multiple ribbon entities #438](https://github.com/djeedai/bevy_hanabi/issues/438)
- [**github.com/djeedai/bevy_hanabi/issues** - bound buffer range 0..592 does not fit in buffer of size 584 #55](https://github.com/djeedai/bevy_hanabi/issues/55)
- [**github.com/djeedai/bevy_hanabi/issues** - spawning two entities with same Handle<EffectAsset> results in wgpu compute shader validation error #399](https://github.com/djeedai/bevy_hanabi/issues/399)
