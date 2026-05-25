use bevy::prelude::*;

use crate::game::collision::{Collider, CollisionResponse, SpinEffect};

#[derive(Component)]
pub struct Wall(pub Plane2d); // TODO: Plane2d redundand? See crate::game::collision::Collider::HalfPlane

pub struct SpawnWalls<M: Bundle + Copy> {
    canvas_size: Vec2, // TODO: rename (maybe?)
    marker: M,
}

impl<M: Bundle + Copy> SpawnWalls<M> {
    pub fn new(canvas_size: Vec2, marker: M) -> Self {
        Self {
            canvas_size,
            marker,
        }
    }
}

impl<M: Bundle + Copy> Command for SpawnWalls<M> {
    fn apply(self, world: &mut World) -> () {
        // Left wall
        world.spawn((
            Wall(Plane2d::new(Vec2::X)),
            Transform::from_xyz(-self.canvas_size.x / 2.0, 0.0, 0.0),
            self.marker,
            Collider::HalfPlane { normal: Dir2::X },
            CollisionResponse::Reflect,
            SpinEffect::Clear,
        ));
        // Right wall
        world.spawn((
            Wall(Plane2d::new(Vec2::NEG_X)),
            Transform::from_xyz(self.canvas_size.x / 2.0, 0.0, 0.0),
            self.marker,
            Collider::HalfPlane {
                normal: Dir2::NEG_X,
            },
            CollisionResponse::Reflect,
            SpinEffect::Clear,
        ));
        // Bottom wall
        world.spawn((
            Wall(Plane2d::new(Vec2::Y)),
            Transform::from_xyz(0.0, -self.canvas_size.y / 2.0, 0.0),
            self.marker,
            Collider::HalfPlane { normal: Dir2::Y },
            CollisionResponse::Reflect,
            SpinEffect::Clear,
        ));
        // Top wall
        world.spawn((
            Wall(Plane2d::new(Vec2::NEG_Y)),
            Transform::from_xyz(0.0, self.canvas_size.y / 2.0, 0.0),
            self.marker,
            Collider::HalfPlane {
                normal: Dir2::NEG_Y,
            },
            CollisionResponse::Reflect,
            SpinEffect::Clear,
        ));
    }
}
