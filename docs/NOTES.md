**https://github.com/djeedai/bevy_hanabi/blob/main/examples/utils/mod.rs**:
```rust
#![allow(unused)]

use std::num::NonZeroU8;

use bevy::{
    camera::{visibility::RenderLayers, CameraOutputMode},
    log::LogPlugin,
    prelude::*,
    render::{settings::WgpuSettings, RenderDebugFlags, RenderPlugin},
    text::{TextColor, TextFont},
    ui::{
        widget::Text, BackgroundColor, BorderColor, BorderRadius, Display, Node, Overflow,
        PositionType, UiRect, Val, ZIndex,
    },
};
use wgpu::BlendState;

use crate::prelude::*;

/// Helper system to enable closing the example application by pressing the
/// escape key (ESC).
pub fn close_on_esc(mut ev_app_exit: MessageWriter<AppExit>, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::Escape) {
        ev_app_exit.write(AppExit::Success);
    }
}

/// Calculate a log filter for the LogPlugin based on the example app name.
pub fn get_log_filters(example_name: &str) -> String {
    [
        // The example app itself is at trace level so we can see everything
        &format!("{}=trace", example_name),
        // Default Hanabi to warn, probably don't need more
        "bevy_hanabi=warn",
        // Prevent HAL from dumping all naga-generated shader code in logs
        "wgpu_hal::dx12::device=warn",
        // Tune down the verbose Vulkan driver output
        "wgpu_hal::vulkan::instance=warn",
    ]
    .join(",")
}

#[derive(Default, Clone, Copy)]
pub enum DescPosition {
    #[default]
    LeftColumn,
    BottomRow,
}

#[derive(Default)]
pub struct DemoApp {
    name: String,
    desc: String,
    wgpu_settings: WgpuSettings,
    desc_position: DescPosition,
}

impl DemoApp {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..default()
        }
    }

    pub fn with_desc(mut self, desc: &str) -> Self {
        self.desc = desc.to_string();
        self
    }

    pub fn with_desc_position(mut self, desc_position: DescPosition) -> Self {
        self.desc_position = desc_position;
        self
    }

    pub fn with_wgpu_settings(mut self, wgpu_settings: WgpuSettings) -> Self {
        self.wgpu_settings = wgpu_settings;
        self
    }

    pub fn build(self) -> App {
        let mut app = App::default();
        app.insert_resource(ClearColor(Color::BLACK))
            .add_plugins(
                DefaultPlugins
                    .set(LogPlugin {
                        level: bevy::log::Level::INFO,
                        filter: get_log_filters(&self.name),
                        ..default()
                    })
                    .set(RenderPlugin {
                        render_creation: self.wgpu_settings.into(),
                        synchronous_pipeline_compilation: false,
                        debug_flags: RenderDebugFlags::empty(),
                    })
                    .set(WindowPlugin {
                        primary_window: Some(Window {
                            title: format!("🎆 Hanabi — {}", self.name),
                            ..default()
                        }),
                        ..default()
                    }),
            )
            .add_plugins(HanabiPlugin)
            .insert_resource(Demo {
                name: self.name,
                desc: self.desc,
                desc_position: self.desc_position,
            })
            .add_systems(Startup, spawn_demo_ui)
            .add_systems(Update, close_on_esc);

        app
    }
}

#[derive(Resource)]
pub struct Demo {
    pub name: String,
    pub desc: String,
    pub desc_position: DescPosition,
}

/// Error struct wrapping an app error code.
#[derive(Debug)]
pub struct ExampleFailedError(pub NonZeroU8);

impl std::fmt::Display for ExampleFailedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "App terminated with error code {}", self.0.get())
    }
}

impl std::error::Error for ExampleFailedError {}

/// Convert an [`AppExit`] into a `Result`, for error code propagation to the
/// OS.
pub trait AppExitIntoResult {
    fn into_result(self) -> Result<(), Box<dyn std::error::Error>>;
}

impl AppExitIntoResult for AppExit {
    fn into_result(self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            AppExit::Success => Ok(()),
            AppExit::Error(code) => Err(Box::new(ExampleFailedError(code))),
        }
    }
}

pub const COLOR_RED: Color = Color::linear_rgb(1., 0., 0.);
pub const COLOR_GREEN: Color = Color::linear_rgb(0., 1., 0.);
pub const COLOR_BLUE: Color = Color::linear_rgb(0., 0., 1.);
pub const COLOR_YELLOW: Color = Color::linear_rgb(1., 1., 0.);
pub const COLOR_CYAN: Color = Color::linear_rgb(0., 1., 1.);
pub const COLOR_OLIVE: Color = Color::linear_rgb(0.5, 0.5, 0.);
pub const COLOR_PURPLE: Color = Color::linear_rgb(0.5, 0., 0.5);

fn spawn_demo_ui(mut cmd: Commands, demo: Res<Demo>) {
    debug!("Spawning UI for demo {}", demo.name);

    // Camera
    let ui_camera = cmd
        .spawn((
            Camera2d,
            Projection::Orthographic(OrthographicProjection::default_2d()),
            Camera {
                order: 1000, // render UI above everything
                clear_color: ClearColorConfig::None,
                output_mode: CameraOutputMode::Write {
                    blend_state: Some(BlendState::ALPHA_BLENDING),
                    clear_color: ClearColorConfig::None,
                },
                ..default()
            },
            Name::new("UI camera"),
            RenderLayers::layer(63),
        ))
        .id();

    // Description UI panel
    let (left, top, right, bottom, width) = match demo.desc_position {
        DescPosition::LeftColumn => (Val::Vw(5.), Val::Vw(5.), Val::Auto, Val::Auto, Val::Vw(30.)),
        DescPosition::BottomRow => (Val::Vw(5.), Val::Auto, Val::Vw(5.), Val::Vw(5.), Val::Auto),
    };
    cmd.spawn((
        Node {
            display: Display::Block,
            position_type: PositionType::Absolute,
            overflow: Overflow::clip(),
            left,
            top,
            right,
            bottom,
            min_width: width,
            width,
            border: UiRect::all(Val::Px(1.)),
            border_radius: BorderRadius::all(Val::Px(8.)),
            ..default()
        },
        BackgroundColor(Color::linear_rgba(0., 0., 0., 0.8)),
        BorderColor::all(Color::linear_rgb(0.8, 0.8, 0.8)),
        ZIndex(3000),
        children![
            (
                Node {
                    padding: UiRect::all(Val::Px(3.)),
                    margin: UiRect::all(Val::Px(8.)),
                    ..default()
                },
                Text::new(demo.name.clone()),
                TextColor(Color::linear_rgb(1., 1., 1.)),
                TextFont::from_font_size(18.),
            ),
            (
                Node {
                    padding: UiRect::all(Val::Px(3.)),
                    margin: UiRect::all(Val::Px(8.)),
                    ..default()
                },
                Text::new(demo.desc.clone()),
                TextColor(Color::linear_rgb(0.8, 0.8, 0.8)),
                TextFont::from_font_size(12.),
            )
        ],
        UiTargetCamera(ui_camera),
    ));
}
```

**https://github.com/djeedai/bevy_hanabi/blob/main/examples/gradient.rs**:
```rust
use std::f32::consts::PI;

use bevy::{
    camera::visibility::RenderLayers, core_pipeline::tonemapping::Tonemapping, prelude::*,
    render::view::Hdr,
};
use bevy_hanabi::prelude::*;

mod utils;
use utils::*;

const DEMO_DESC: &str = include_str!("gradient.txt");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_exit = utils::DemoApp::new("gradient")
        .with_desc(DEMO_DESC)
        .with_desc_position(DescPosition::BottomRow)
        .build()
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
    app_exit.into_result()
}

fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn a camera. For this example, we also demonstrate that we can use an HDR
    // camera.
    commands.spawn((
        Transform::from_translation(Vec3::Z * 100.),
        Camera3d::default(),
        Hdr,
        Tonemapping::None,
        // For this example, we assign to the camera a specific render layer (3)
        // different from the default (0) to demonstrate it works.
        RenderLayers::layer(3),
    ));

    let texture_handle: Handle<Image> = asset_server.load("cloud.png");

    let mut gradient = bevy_hanabi::Gradient::new();
    gradient.add_key(0.0, Vec4::new(0.5, 0.5, 0.5, 1.0));
    gradient.add_key(0.1, Vec4::new(0.5, 0.5, 0.0, 1.0));
    gradient.add_key(0.4, Vec4::new(0.5, 0.0, 0.0, 1.0));
    gradient.add_key(1.0, Vec4::splat(0.0));

    let writer = ExprWriter::new();

    let age = writer.lit(0.).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    let lifetime = writer.lit(5.).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let init_pos = SetPositionSphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        radius: writer.lit(1.).expr(),
        dimension: ShapeDimension::Volume,
    };

    let init_vel = SetVelocitySphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        speed: writer.lit(2.).expr(),
    };

    // Use texture slot #0 in ParticleTextureModifier
    let texture_slot = writer.lit(0u32).expr();

    // Define that texture slot (giving it a name for convenience)
    let mut module = writer.finish();
    module.add_texture_slot("color");

    let effect = effects.add(
        EffectAsset::new(32768, SpawnerSettings::rate(1000.0.into()), module)
            .with_name("gradient")
            .init(init_pos)
            .init(init_vel)
            .init(init_age)
            .init(init_lifetime)
            .render(ParticleTextureModifier {
                texture_slot,
                sample_mapping: ImageSampleMapping::ModulateOpacityFromR,
            })
            .render(ColorOverLifetimeModifier::new(gradient)),
    );

    commands
        .spawn((
            Name::new("effect"),
            ParticleEffect::new(effect),
            // We need to spawn the effect with the same render layer as the camera, otherwise it
            // won't be rendered.
            RenderLayers::layer(3),
            // We need to bind a texture to the slot #0 we created above
            EffectMaterial {
                images: vec![texture_handle.clone()],
            },
        ))
        .with_children(|p| {
            // Reference cube to visualize the emit origin

            let mut mat: StandardMaterial = utils::COLOR_CYAN.into();
            mat.unlit = true;
            p.spawn((
                Mesh3d(meshes.add(Cuboid {
                    half_size: Vec3::splat(1.0),
                })),
                MeshMaterial3d(materials.add(mat)),
                RenderLayers::layer(3),
            ));
        });
}

/// Calculate a position over a Lemniscate of Bernoulli curve ("infinite
/// symbol").
///
/// The fractional part of `time` determines the parametric position over the
/// curve. The `radius` of the Lemniscate curve is the distance from the center
/// to the edge of any of the left or right loops. The curve loops extend from
/// -X to +X.
fn lemniscate(time: f32, radius: f32) -> Vec2 {
    // The Lemniscate is defined in polar coordinates by the equation r² = a² ⨯
    // cos(2θ), where a is the radius of the Lemniscate (distance from origin to
    // loop edge). This equation is defined only for values of θ in the
    // [-π/4:π/4] range. Each value yields two possible values for r, one
    // positive and one negative, corresponding to the two loops of the
    // Lemniscates (left and right). So we solve for θ ∈ [-π/4:π/4], and make θ
    // vary back and forth in the [-π/4:π/4] range. Then depending on the
    // direction we flip the sign of r. This produces a continuous
    // parametrization of the curve.

    const TWO_PI: f32 = PI * 2.0;
    const PI_OVER_4: f32 = PI / 4.0;

    // Scale the parametric position over the curve to the [0:2*π] range
    let theta = time.fract() * TWO_PI;

    // This variant produces a linear parametrization of theta (triangular signal).
    // Because the parameter r changes much faster around 0 when solving the
    // polar equation, this makes the position "go faster" around the center,
    // which is generally not wanted. let (theta, sign) = if theta <= PI_OVER_2
    // {     (theta - PI_OVER_4, 1.0)
    // } else {
    //     (3.0 * PI_OVER_4 - theta, -1.0)
    // };

    // That alternative variant "slows down" the parametric position around zero by
    // converting the linear θ variations with a sine function, making it move
    // slower around the edges ±π/4 where r tends to zero. This does not produce
    // an exact constant speed, but is visually close.
    let sign = theta.cos().signum();
    let theta = theta.sin() * PI_OVER_4;

    // Solve the polar equation to build the parametric position. Clamp to positive
    // values for r2 due to numerical errors infrequently yielding negative values.
    let r2 = (radius * radius * (theta * 2.0).cos()).max(0.);
    let r = r2.sqrt().copysign(sign);

    // Convert to cartesian coordinates
    let x = r * theta.cos();
    let y = r * theta.sin();
    Vec2::new(x, y)
}

fn update(time: Res<Time>, mut query: Query<&mut Transform, With<ParticleEffect>>) {
    const ALPHA_OFFSET: f32 = PI * 0.41547;
    const SPEED_OFFSET: f32 = 2.57;
    let mut alpha_off = 0.0;
    let mut speed = 4.25;
    for mut transform in query.iter_mut() {
        let alpha = time.elapsed_secs() * PI / speed + alpha_off;
        let radius = 50.0;
        transform.translation = lemniscate(alpha, radius).extend(0.0);
        alpha_off += ALPHA_OFFSET;
        speed += SPEED_OFFSET;
    }
}
```

**https://github.com/djeedai/bevy_hanabi/blob/main/examples/gradient.txt**:
```text
This example shows a basic particle effect setup.

The particles are emitted at constant rate every frame, as the emitter moves around the scene (changes its Transform). The particles are rendered with a texture modulating their opacity, to give them a noise/smoke shape, and a color gradient to change their color and opacity over their lifetime, and eventually make them fade out.
```