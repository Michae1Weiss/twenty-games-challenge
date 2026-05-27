use std::f32::consts::{FRAC_PI_6, PI};

use bevy::{
    math::{
        FloatOrd,
        bounding::{Aabb2d, RayCast2d},
    },
    prelude::*,
};

use crate::game::ball::Ball;

pub(super) fn plugin(app: &mut App) {
    app.add_message::<Collision>();
}

/// Nudge the next ray off the surface so it doesn't re-detect the surface it
/// just bounced from at distance 0.
const SURFACE_EPS: f32 = 0.5;

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
    Impart(f32), // radian/sec
}

#[allow(dead_code)]
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

pub struct TrajectoryParams {
    pub max_length: f32,
    pub max_bounces: u32,
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
        Without<Ball>,
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

/// Walks the ball's path through reflections, writing the polyline into `out`.
/// Returns the entity of the FIRST surface hit, so the caller can decide
/// whether to draw (e.g. only when the first hit is the paddle).
///
/// Reuses the real physics: `first_contact` for geometry, `reflect_velocity`
/// for the bounce. A brick (`BounceOrPierce`) terminates the line — that's the
/// target. Walls and the paddle continue it. The total length is capped, so it
/// can never run away no matter how many reflections happen.
pub fn predict_trajectory(
    start: Vec2,
    mut direction: Dir2,
    speed: f32,
    colliders: &Query<
        (
            Entity,
            &Transform,
            &Collider,
            &CollisionResponse,
            Option<&SpinEffect>,
        ),
        Without<Ball>,
    >,
    params: &TrajectoryParams,
    out: &mut Vec<Vec2>,
) -> Option<Entity> {
    out.clear();
    out.push(start);

    let mut position = start;
    let mut remaining = params.max_length;
    let mut first_hit = None;

    for _ in 0..params.max_bounces {
        let ray = Ray2d::new(position, direction);
        let Some((contact, response, _)) = first_contact(ray, remaining, colliders) else {
            // No hit within budget: run the final segment straight to the cap.
            out.push(position + *direction * remaining);
            return first_hit;
        };

        first_hit.get_or_insert(contact.entity);
        out.push(contact.contact_point);
        remaining -= contact.distance;

        match response {
            CollisionResponse::ReflectOrPierce => return first_hit, // hit a brick → done
            CollisionResponse::Reflect | CollisionResponse::Deflect => {
                let outgoing = reflect_velocity(*direction * speed, response, &contact);
                let Ok(next) = Dir2::new(outgoing) else {
                    return first_hit;
                };
                direction = next;
                position = contact.contact_point + *next * SURFACE_EPS;
            }
        }

        if remaining <= 0.0 {
            return first_hit;
        }
    }

    first_hit
}

// TODO: ball still uses it's own "how a surface redirects the ball"
// the single source of truth for "how a surface redirects the ball"
fn reflect_velocity(incoming: Vec2, response: CollisionResponse, contact: &Contact) -> Vec2 {
    match response {
        CollisionResponse::Reflect | CollisionResponse::ReflectOrPierce => {
            incoming.reflect(contact.contact_surface_normal.as_vec2())
        }
        CollisionResponse::Deflect => deflect(
            contact.contact_point,
            contact.hit_body_center,
            incoming.length(),
        ),
    }
}
