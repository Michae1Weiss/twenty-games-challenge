use bevy::prelude::*;

use crate::game::{
    HalfSize, TextureAssets,
    collision::{Collider, Collision, CollisionResponse},
};

pub(super) fn plugin(app: &mut App) {
    app.add_message::<BrickDestroyed>()
        .add_systems(Update, destroy_on_collision);
}

const BRICK_SIZE: Vec2 = Vec2::new(80., 40.);

#[derive(Component)]
pub struct Brick;

#[derive(Message)]
pub struct BrickDestroyed {
    position: Vec2,
}

impl BrickDestroyed {
    pub fn new(position: Vec2) -> Self {
        Self { position }
    }
}

pub struct SpawnBricks<M>
where
    M: Bundle + Copy,
{
    n_rows: u8,
    n_columns: u8,
    marker: M,
}

impl<M> SpawnBricks<M>
where
    M: Bundle + Copy,
{
    pub fn new(n_rows: u8, n_columns: u8, marker: M) -> Self {
        Self {
            n_rows,
            n_columns,
            marker,
        }
    }
}

impl<M> Command for SpawnBricks<M>
where
    M: Bundle + Copy,
{
    fn apply(self, world: &mut World) -> () {
        // TODO: remove redundand clone
        let texture =
            world.resource_scope(|_, textures: Mut<TextureAssets>| textures.brick.clone());

        for row in 0..self.n_rows {
            for column in 0..self.n_columns {
                world.spawn((
                    Brick,
                    Sprite {
                        image: texture.clone(),
                        custom_size: Some(BRICK_SIZE),
                        ..default()
                    },
                    Transform::from_xyz(
                        -480.0 + BRICK_SIZE.x * column as f32,
                        240.0 - BRICK_SIZE.y * row as f32,
                        0.0,
                    ),
                    HalfSize(BRICK_SIZE / 2.0),
                    self.marker,
                    Collider::Aabb {
                        half_size: BRICK_SIZE / 2.0,
                    },
                    CollisionResponse::ReflectOrPierce,
                ));
            }
        }
    }
}

fn destroy_on_collision(
    mut commands: Commands,
    bricks: Query<&Transform, With<Brick>>,
    mut collisions: MessageReader<Collision>,
    mut brick_destroyed_writer: MessageWriter<BrickDestroyed>,
) {
    for collision in collisions.read() {
        if let Ok(translation) = bricks.get(collision.hit) {
            commands.entity(collision.hit).despawn();
            brick_destroyed_writer.write(BrickDestroyed::new(translation.translation.xy()));
        }
    }
}
