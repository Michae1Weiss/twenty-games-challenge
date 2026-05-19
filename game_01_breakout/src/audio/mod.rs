use bevy::prelude::*;

pub mod assets;
mod music;
mod settings;
pub mod sfx;

pub use sfx::PlaySfx;

#[derive(Component)]
struct Sfx;

#[derive(Component)]
struct Music;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(sfx::plugin);
}
