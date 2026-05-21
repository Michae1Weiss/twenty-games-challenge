use bevy::prelude::*;

use crate::audio::{Music, Sfx, Volume};

#[derive(Resource, Debug, Clone)]
pub struct AudioSettings {
    pub music: Volume,
    pub sfx: Volume,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            music: Volume::new(0.6),
            sfx: Volume::new(0.8),
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<AudioSettings>().add_systems(
        Update,
        (apply_music, apply_sfx).run_if(resource_changed::<AudioSettings>),
    );
}

fn apply_music(settings: Res<AudioSettings>, mut sinks: Query<&mut AudioSink, With<Music>>) {
    for mut sink in &mut sinks {
        sink.set_volume(bevy::audio::Volume::Linear(settings.music.perceptual()));
    }
}
fn apply_sfx(settings: Res<AudioSettings>, mut sinks: Query<&mut AudioSink, With<Sfx>>) {
    for mut sink in &mut sinks {
        sink.set_volume(bevy::audio::Volume::Linear(settings.sfx.perceptual()));
    }
}
