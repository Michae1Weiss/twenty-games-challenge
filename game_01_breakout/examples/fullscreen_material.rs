//! Demonstrates how to write a custom fullscreen shader
//!
//! This is currently limited to 3d only but work is in progress to make it work in 2d
//! Ref: https://github.com/bevyengine/bevy/blob/main/examples/shader_advanced/fullscreen_material.rs

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
            FullscreenMaterialPlugin::<FullscreenEffect>::default(),
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // camera
    commands.spawn((
        Camera2d::default(),
        Projection::Orthographic(OrthographicProjection {
            ..OrthographicProjection::default_2d()
        }),
        FullscreenEffect { intensity: 0.005 },
    ));

    // cube
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(60.0))),
        MeshMaterial2d(materials.add(Color::from(RED_200))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

// This is the struct that will be sent to your shader
//
// Currently, this doesn't support AsBindGroup so you can only use it to send a struct to your
// shader. We are working on adding AsBindGroup support in the future so you can bind anything you
// need.
#[derive(Component, ExtractComponent, Clone, Copy, ShaderType, Default)]
struct FullscreenEffect {
    // For this example, this is used as the intensity of the effect, but you can pass in any valid
    // ShaderType
    //
    // In the future, you will be able to use a full bind group
    intensity: f32,
}

impl FullscreenMaterial for FullscreenEffect {
    // The shader that will be used
    fn fragment_shader() -> ShaderRef {
        "shaders/fullscreen_effect.wgsl".into()
    }

    // This let's you specify a list of edges used to order when your effect pass will run
    //
    // This example is a post processing effect so it will run after tonemapping but before the end
    // post processing pass.
    //
    // In 2d you would need to use [`Node2d`] instead of [`Node3d`]
    fn node_edges() -> Vec<InternedRenderLabel> {
        vec![
            Node2d::Tonemapping.intern(),
            // The label is automatically generated from the name of the struct
            Self::node_label().intern(),
            Node2d::EndMainPassPostProcessing.intern(),
        ]
    }
}
