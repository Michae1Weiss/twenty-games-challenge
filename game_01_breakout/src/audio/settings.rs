use bevy::prelude::*;

#[derive(Resource, Debug, Clone)]
pub struct AudioSettings {
    pub music_volume: f32, // 0.0 .. 1.0
    pub sfx_volume: f32,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            music_volume: 0.6,
            sfx_volume: 0.8,
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<AudioSettings>().add_systems(
        Update,
        apply_volume.run_if(resource_changed::<AudioSettings>),
    );
}

fn apply_volume(settings: Res<AudioSettings>, mut global_volume: ResMut<GlobalVolume>) {
    // Bevy 0.18: GlobalVolume holds a Volume; adjust to your audio routing.
    // If you separate music/sfx with channels, route accordingly.
    global_volume.volume = bevy::audio::Volume::Linear(settings.music_volume);
    // ^ Adjust: if you have a music channel and sfx channel, set both.
    let _ = settings.sfx_volume; // route to your sfx channel resource
}
