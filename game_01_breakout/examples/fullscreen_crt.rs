use bevy::{
    color::palettes::tailwind::RED_200,
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

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FullscreenMaterialPlugin::<CrtSettings>::default(), // ← the type changed
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, animate_crt) // ← drives `time`; without it scanlines freeze
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            ..OrthographicProjection::default_2d()
        }),
        CrtSettings {
            distortion: 0.35,
            scanline_intensity: 0.5,
            time: 0.2,
        },
    ));

    // cube
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(60.0))),
        MeshMaterial2d(materials.add(Color::from(RED_200))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

/// MUST match `CrtSettings` in crt.wgsl field-for-field: three f32, same order.
#[derive(Component, ExtractComponent, Clone, Copy, ShaderType, Default)]
struct CrtSettings {
    distortion: f32,
    scanline_intensity: f32,
    time: f32,
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

/// The shader's `time` field is just data — nothing updates it unless you do.
/// Real time so the CRT scanlines keep crawling even when the game is paused
/// or in bullet-time (a physical monitor doesn't slow down). Use Time<Virtual>
/// instead if you want the scanlines to freeze with the game.
fn animate_crt(time: Res<Time<Real>>, mut settings: Query<&mut CrtSettings>) {
    for mut s in &mut settings {
        s.time = time.elapsed_secs();
    }
}
