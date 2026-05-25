use bevy::prelude::*;

use crate::audio::Music;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, pitch_music_with_time);
}

fn pitch_music_with_time(
    virtual_time: Res<Time<Virtual>>,
    music: Query<&AudioSink, With<Music>>, // your existing Music marker
) {
    let scale = virtual_time.relative_speed();
    // Don't drop a full octave — map [min..1.0] onto a gentler [0.6..1.0].
    let pitch = 0.8 + 0.2 * scale;
    for sink in &music {
        sink.set_speed(pitch);
    }
}
