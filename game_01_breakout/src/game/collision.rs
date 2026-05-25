use std::f32::consts::{FRAC_PI_6, PI};

use bevy::{
    math::{
        FloatOrd,
        bounding::{Aabb2d, RayCast2d},
    },
    prelude::*,
};

#[derive(Component)]
pub enum Collider {
    Aabb { half_size: Vec2 },   // e.g. brick, paddle
    HalfPlane { normal: Dir2 }, // e.g. wall
}

#[derive(Component, Clone, Copy)]
pub enum CollisionResponse {
    Reflect,
    ReflectOrPierce,
    Deflect,
}

#[derive(Component, Clone, Copy)]
pub enum SpinEffect {
    Clear,
    Impart(f32), // TODO: f32 is ...? Describe what f32 stays for!
}

#[derive(Message)]
pub struct Collision {
    pub ball: Entity,
    pub hit: Entity,
    pub point: Vec2,
    pub normal: Dir2,
}

pub struct Contact {
    pub entity: Entity,
    pub distance: f32,
    pub contact_point: Vec2,
    pub contact_surface_normal: Dir2,
    pub hit_body_center: Vec2,
}

pub fn first_contact(
    ray: Ray2d,
    max_distance: f32,
    colliders: &Query<
        (
            Entity,
            &Transform,
            &Collider,
            &CollisionResponse,
            Option<&SpinEffect>,
        ),
        // Without<Ball>,
    >,
) -> Option<(Contact, CollisionResponse, Option<SpinEffect>)> {
    let ray_cast = RayCast2d::from_ray(ray, max_distance);

    colliders
        .iter()
        .filter_map(|(entity, transform, collider, response, spin)| {
            let center = transform.translation.xy();

            let (distance, normal) = match collider {
                Collider::HalfPlane { normal } => {
                    // walls: intersect the infinite plane through `center`
                    let distance = ray.intersect_plane(center, Plane2d::new(normal.as_vec2()))?;
                    (distance, *normal)
                }
                Collider::Aabb { half_size } => {
                    let aabb = Aabb2d::new(center, *half_size);
                    let distance = ray_cast.aabb_intersection_at(&aabb)?;
                    // derive the face normal from where on the box we landed:
                    // at the entry point one axis sits on ±half_size, so its
                    // normalised magnitude wins.
                    let point = ray.origin + ray.direction * distance;
                    let local = (point - center) / *half_size;
                    let normal = if local.x.abs() >= local.y.abs() {
                        Dir2::new_unchecked(Vec2::new(local.x.signum(), 0.0))
                    } else {
                        Dir2::new_unchecked(Vec2::new(0.0, local.y.signum()))
                    };
                    (distance, normal)
                }
            };

            // intersect_plane only returns hits ahead of the ray, but it's
            // unbounded, so the upper bound still has to be enforced here.
            (distance <= max_distance).then(|| {
                let contact = Contact {
                    entity,
                    distance,
                    contact_point: ray.origin + ray.direction * distance,
                    contact_surface_normal: normal,
                    hit_body_center: center,
                };

                (contact, *response, spin.copied())
            })
        })
        .min_by_key(|(contact, _, _)| FloatOrd(contact.distance))
}

pub fn deflect(ball_pos: Vec2, paddle_center: Vec2, speed: f32) -> Vec2 {
    let offset = ball_pos - paddle_center;
    let linear = offset.to_angle().clamp(0.0, PI) / PI;
    let angle = FRAC_PI_6.lerp(PI - FRAC_PI_6, linear);
    Vec2::from_angle(angle) * speed
}
