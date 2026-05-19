use bevy::app::App;

mod audio;
mod game;
mod theme;
mod third_party;
mod ui;

pub fn plugin(app: &mut App) {
    app.add_plugins(game::plugin);
}
