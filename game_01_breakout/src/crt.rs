use bevy::{
    core_pipeline::{
        core_2d::graph::Node2d,
        fullscreen_material::{FullscreenMaterial, FullscreenMaterialPlugin},
    },
    prelude::*,
    render::{
        extract_component::ExtractComponent,
        render_graph::{InternedRenderLabel, RenderLabel},
        render_resource::ShaderType,
    },
    shader::ShaderRef,
};

#[derive(Component, ExtractComponent, Clone, Copy, ShaderType)]
pub struct CrtSettings {
    pub distortion: f32,
    pub scanline_intensity: f32,
    pub time: f32,
}

impl Default for CrtSettings {
    fn default() -> Self {
        Self {
            distortion: 0.02,
            scanline_intensity: 0.05,
            time: 0.0,
        }
    }
}

impl FullscreenMaterial for CrtSettings {
    fn fragment_shader() -> ShaderRef {
        "shaders/crt.wgsl".into()
    }
    fn node_edges() -> Vec<InternedRenderLabel> {
        vec![
            Node2d::Tonemapping.intern(),
            Self::node_label().intern(),
            Node2d::EndMainPassPostProcessing.intern(),
        ]
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(FullscreenMaterialPlugin::<CrtSettings>::default())
        .add_systems(Update, drive_crt);
}

/// `time` from REAL time (the monitor never slows). `distortion`/`scanline`
/// ramp with bullet-time's `relative_speed` — at full speed it's the baseline,
/// in slow-mo it warps harder. Reads the same scalar the audio pitch does.
fn drive_crt(
    real_time: Res<Time<Real>>,
    virtual_time: Res<Time<Virtual>>,
    mut settings: Query<&mut CrtSettings>,
) {
    let slow = 1.0 - virtual_time.relative_speed(); // 0.0 at full speed → ~0.8 at slowest
    for mut s in &mut settings {
        s.time = real_time.elapsed_secs();
        s.distortion = 0.04 + slow * 0.05;
        s.scanline_intensity = 0.01 + slow * 0.35;
    }
}
