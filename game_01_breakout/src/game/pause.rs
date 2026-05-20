use bevy::prelude::*;

use crate::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Pause), pause_virtual_time)
        .add_systems(OnExit(Screen::Pause), unpause_virtual_time);
}

fn pause_virtual_time(mut virtual_time: ResMut<Time<Virtual>>) {
    virtual_time.pause();
}
fn unpause_virtual_time(mut virtual_time: ResMut<Time<Virtual>>) {
    virtual_time.unpause();
}
