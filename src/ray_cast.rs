use bevy::{
    ecs::query::{FilterFetch, WorldQuery},
    prelude::*,
};
use impacted::CollisionShape;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    origin: Vec2,
    direction: Vec2,
    length: Option<f32>,
}

impl Ray {
    pub fn new(origin: Vec2, direction: Vec2) -> Self {
        Ray {
            origin,
            direction,
            length: None,
        }
    }

    pub fn with_length(mut self, length: f32) -> Self {
        self.length = Some(length);
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RayHit {
    pub position: Vec2,
    pub distance: f32,
}

pub trait RayCast {
    fn ray_cast(&self, ray: Ray) -> Option<RayHit>;
}

pub struct Edge(pub Vec2, pub Vec2);

impl RayCast for Edge {
    fn ray_cast(&self, ray: Ray) -> Option<RayHit> {
        let x1 = self.0.x;
        let y1 = self.0.y;
        let x2 = self.1.x;
        let y2 = self.1.y;

        let x3 = ray.origin.x;
        let y3 = ray.origin.y;
        let x4 = ray.origin.x + ray.direction.x;
        let y4 = ray.origin.y + ray.direction.y;

        let den = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
        if den == 0.0 {
            return None;
        }

        let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / den;
        let u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / den;

        if t > 0.0 && t < 1.0 && u > 0.0 {
            let point = Vec2::new(x1 + t * (x2 - x1), y1 + t * (y2 - y1));
            let distance = point.distance(ray.origin);
            match ray.length {
                Some(length) if length >= distance => Some(RayHit {
                    position: point,
                    distance,
                }),
                None => Some(RayHit {
                    position: point,
                    distance,
                }),
                _ => None,
            }
        } else {
            None
        }
    }
}

impl<'w, 's, 'q1, 'q2, F: WorldQuery> RayCast
    for Query<'w, 's, (&'q1 CollisionShape, &'q2 Transform), F>
where
    F: WorldQuery,
    F::Fetch: FilterFetch,
{
    fn ray_cast(&self, ray: Ray) -> Option<RayHit> {
        self.iter()
            .fold(None, |acc: Option<RayHit>, (collider, transform)| {
                let bounds = collider.bounds();
                let top_left = Vec2::new(
                    bounds.min().x + transform.translation.x,
                    bounds.max().y + transform.translation.y,
                );
                let top_right = Vec2::new(
                    bounds.max().x + transform.translation.x,
                    bounds.max().y + transform.translation.y,
                );
                let bottom_left = Vec2::new(
                    bounds.min().x + transform.translation.x,
                    bounds.min().y + transform.translation.y,
                );
                let bottom_right = Vec2::new(
                    bounds.max().x + transform.translation.x,
                    bounds.min().y + transform.translation.y,
                );

                let edges = [
                    Edge(top_left, top_right),
                    Edge(top_right, bottom_right),
                    Edge(bottom_right, bottom_left),
                    Edge(bottom_left, top_left),
                ];

                let hit = edges.into_iter().fold(None, |acc: Option<RayHit>, edge| {
                    let hit = edge.ray_cast(ray);

                    match hit {
                        Some(hit) => match acc {
                            Some(previous_hit) => {
                                if hit.distance < previous_hit.distance {
                                    Some(hit)
                                } else {
                                    acc
                                }
                            }
                            None => Some(hit),
                        },
                        None => acc,
                    }
                });

                match hit {
                    Some(hit) => match acc {
                        Some(previous_hit) => {
                            if hit.distance < previous_hit.distance {
                                Some(hit)
                            } else {
                                acc
                            }
                        }
                        None => Some(hit),
                    },
                    None => acc,
                }
            })
    }
}
