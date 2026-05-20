use bevy::prelude::*;

mod interaction;
mod main_menu;
mod pause_menu;
mod settings_menu;
mod widget;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        main_menu::plugin,
        pause_menu::plugin,
        interaction::plugin,
        settings_menu::plugin,
    ));
}
