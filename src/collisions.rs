use bevy::{core::Stopwatch, prelude::*};
#[cfg(feature = "debug")]
use bevy_prototype_lyon::prelude::*;
use impacted::CollisionShape;

use crate::{
    ray_cast::{Ray, RayCast},
    CharacterController, ControllerLabel,
};

pub(crate) struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PlayerVelocity>()
            .register_type::<RaySpacing>()
            .register_type::<CollisionInfo>()
            .register_type::<RaycastOrigins>()
            .register_type::<JumpCount>()
            .register_type::<CoyoteStopwatch>()
            .add_event::<CollisionEvent>()
            .add_system(calculate_ray_spacing.label(ControllerLabel::Calculate))
            .add_system(update_ray_origins.label(ControllerLabel::Calculate))
            .add_system(
                horizontal_collisions
                    .label(ControllerLabel::Collisions)
                    .after(ControllerLabel::Calculate),
            )
            .add_system(
                vertical_collisions
                    .label(ControllerLabel::Collisions)
                    .after(ControllerLabel::Calculate),
            )
            .add_system(reset_jumps.after(ControllerLabel::Collisions))
            .add_system(coyote_time.after(ControllerLabel::Collisions));
    }
}

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct PlayerVelocity(pub Vec2);

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct RaySpacing {
    horizontal: f32,
    vertical: f32,
}

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct CollisionInfo {
    pub above: bool,
    pub below: bool,
    pub left: bool,
    pub right: bool,
}

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct RaycastOrigins {
    top_left: Vec2,
    top_right: Vec2,
    bottom_left: Vec2,
    bottom_right: Vec2,
}

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct JumpCount(pub u32);

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct CoyoteStopwatch(pub(crate) Stopwatch);

#[derive(Component)]
struct DebugShapeHorizontal;

#[derive(Component)]
struct DebugShapeVertical;

#[derive(Clone, Copy, Debug)]
pub struct CollisionEvent {
    entity: Entity,
    side: CollisionSide,
}

#[derive(Clone, Copy, Debug)]
pub enum CollisionSide {
    Below(bool),
    Above(bool),
    Left(bool),
    Right(bool),
}

fn calculate_ray_spacing(
    mut query: Query<
        (&mut RaySpacing, &CharacterController, &CollisionShape),
        Or<(Changed<CharacterController>, Changed<CollisionShape>)>,
    >,
) {
    for (mut ray_spacing, controller, collider) in query.iter_mut() {
        let bounds = collider.bounds().expand(Vec2::new(
            controller.skin_width * -2.0,
            controller.skin_width * -2.0,
        ));

        ray_spacing.horizontal = bounds.size().y / (controller.horizontal_ray_count - 1) as f32;
        ray_spacing.vertical = bounds.size().x / (controller.vertical_ray_count - 1) as f32;
    }
}

fn update_ray_origins(
    mut query: Query<
        (
            &mut RaycastOrigins,
            &CharacterController,
            &Transform,
            &CollisionShape,
        ),
        Or<(
            Changed<CharacterController>,
            Changed<Transform>,
            Changed<CollisionShape>,
        )>,
    >,
) {
    for (mut ray_origins, controller, transform, collider) in query.iter_mut() {
        let bounds = collider.bounds().expand(Vec2::new(
            controller.skin_width * -2.0,
            controller.skin_width * -2.0,
        ));
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

        ray_origins.top_left = top_left;
        ray_origins.top_right = top_right;
        ray_origins.bottom_left = bottom_left;
        ray_origins.bottom_right = bottom_right;
    }
}

fn horizontal_collisions(
    mut commands: Commands,
    mut controllers: Query<(
        Entity,
        &mut PlayerVelocity,
        &mut CollisionInfo,
        &CharacterController,
        &RaycastOrigins,
        &RaySpacing,
    )>,
    colliders: Query<(&CollisionShape, &Transform), Without<CharacterController>>,
    mut collision_events: EventWriter<CollisionEvent>,
    shapes: Query<Entity, With<DebugShapeHorizontal>>,
) {
    for shape in shapes.iter() {
        commands.entity(shape).despawn();
    }

    for (entity, mut velocity, mut collisions, controller, raycast_origins, ray_spacing) in
        controllers.iter_mut()
    {
        if velocity.0.x == 0.0 {
            continue;
        }

        let mut left = false;
        let mut right = false;

        let direction_x = velocity.0.x.signum();
        let mut ray_length = velocity.0.x.abs() + controller.skin_width;
        for i in 0..controller.horizontal_ray_count {
            let mut ray_origin = if direction_x == -1.0 {
                raycast_origins.bottom_left
            } else {
                raycast_origins.bottom_right
            };
            ray_origin += Vec2::Y * (ray_spacing.horizontal * i as f32);
            let ray_direction = Vec2::X * direction_x;

            let hit =
                colliders.ray_cast(Ray::new(ray_origin, ray_direction).with_length(ray_length));

            // Ray cast line
            #[cfg(feature = "debug")]
            commands
                .spawn_bundle(GeometryBuilder::build_as(
                    &shapes::Line(
                        ray_origin + velocity.0,
                        (ray_origin + (ray_direction * ray_length)) + velocity.0,
                    ),
                    DrawMode::Stroke(StrokeMode {
                        color: Color::YELLOW,
                        options: StrokeOptions::default().with_line_width(1.0),
                    }),
                    Transform::from_xyz(0.0, 0.0, 10.0),
                ))
                .insert(DebugShapeHorizontal);

            if let Some(hit) = hit {
                // Point
                #[cfg(feature = "debug")]
                commands
                    .spawn_bundle(GeometryBuilder::build_as(
                        &shapes::Circle {
                            center: hit.position,
                            radius: 1.0,
                        },
                        DrawMode::Stroke(StrokeMode {
                            color: Color::GRAY,
                            options: StrokeOptions::default().with_line_width(1.0),
                        }),
                        Transform::from_xyz(0.0, 0.0, 10.0),
                    ))
                    .insert(DebugShapeHorizontal);

                velocity.0.x = (hit.distance - controller.skin_width) * direction_x;
                ray_length = hit.distance;

                if direction_x == -1.0 {
                    left = true;
                }
                if direction_x == 1.0 {
                    right = true;
                }
            }
        }

        if collisions.left != left {
            collisions.left = left;
            collision_events.send(CollisionEvent {
                entity,
                side: CollisionSide::Left(left),
            });
        }
        if collisions.right != right {
            collisions.right = right;
            collision_events.send(CollisionEvent {
                entity,
                side: CollisionSide::Right(right),
            });
        }
    }
}

fn vertical_collisions(
    mut commands: Commands,
    mut controllers: Query<(
        Entity,
        &mut PlayerVelocity,
        &mut CollisionInfo,
        &CharacterController,
        &RaycastOrigins,
        &RaySpacing,
    )>,
    colliders: Query<(&CollisionShape, &Transform), Without<CharacterController>>,
    mut collision_events: EventWriter<CollisionEvent>,
    shapes: Query<Entity, With<DebugShapeVertical>>,
) {
    for shape in shapes.iter() {
        commands.entity(shape).despawn();
    }

    for (entity, mut velocity, mut collisions, controller, raycast_origins, ray_spacing) in
        controllers.iter_mut()
    {
        let mut below = false;
        let mut above = false;

        let direction_y = velocity.0.y.signum();
        let mut ray_length = velocity.0.y.abs() + controller.skin_width;
        for i in 0..controller.vertical_ray_count {
            let mut ray_origin = if direction_y == -1.0 {
                raycast_origins.bottom_left
            } else {
                raycast_origins.top_left
            };
            ray_origin += Vec2::X * (ray_spacing.vertical * i as f32 + velocity.0.x);
            let ray_direction = Vec2::Y * direction_y;

            let hit =
                colliders.ray_cast(Ray::new(ray_origin, ray_direction).with_length(ray_length));

            // Ray cast line
            #[cfg(feature = "debug")]
            commands
                .spawn_bundle(GeometryBuilder::build_as(
                    &shapes::Line(
                        ray_origin + velocity.0,
                        (ray_origin + (ray_direction * ray_length)) + velocity.0,
                    ),
                    DrawMode::Stroke(StrokeMode {
                        color: Color::YELLOW,
                        options: StrokeOptions::default().with_line_width(1.0),
                    }),
                    Transform::from_xyz(0.0, 0.0, 10.0),
                ))
                .insert(DebugShapeVertical);

            if let Some(hit) = hit {
                // Point
                #[cfg(feature = "debug")]
                commands
                    .spawn_bundle(GeometryBuilder::build_as(
                        &shapes::Circle {
                            center: hit.position,
                            radius: 1.0,
                        },
                        DrawMode::Stroke(StrokeMode {
                            color: Color::GRAY,
                            options: StrokeOptions::default().with_line_width(1.0),
                        }),
                        Transform::from_xyz(0.0, 0.0, 100.0),
                    ))
                    .insert(DebugShapeVertical);

                velocity.0.y = (hit.distance - controller.skin_width) * direction_y;
                ray_length = hit.distance;

                if direction_y == -1.0 {
                    below = true;
                }
                if direction_y == 1.0 {
                    above = true;
                }
            }
        }

        if collisions.below != below {
            collisions.below = below;
            collision_events.send(CollisionEvent {
                entity,
                side: CollisionSide::Below(below),
            });
        }
        if collisions.above != above {
            collisions.above = above;
            collision_events.send(CollisionEvent {
                entity,
                side: CollisionSide::Above(above),
            });
        }
    }
}

fn reset_jumps(
    mut collision_events: EventReader<CollisionEvent>,
    mut query: Query<&mut JumpCount>,
) {
    for CollisionEvent { entity, side } in collision_events.iter() {
        match side {
            CollisionSide::Below(below) if *below => {
                if let Ok(mut jump_count) = query.get_component_mut::<JumpCount>(*entity) {
                    jump_count.0 = 0;
                }
            }
            _ => {}
        }
    }
}

fn coyote_time(
    time: Res<Time>,
    mut collision_events: EventReader<CollisionEvent>,
    mut query: Query<(&mut CoyoteStopwatch, &JumpCount)>,
) {
    for CollisionEvent { entity, side } in collision_events.iter() {
        if let CollisionSide::Below(below) = side {
            let jump_count = query
                .get_component::<JumpCount>(*entity)
                .map(|JumpCount(count)| *count)
                .unwrap_or(0);

            if let Ok(mut coyote_stopwatch) = query.get_component_mut::<CoyoteStopwatch>(*entity) {
                if *below {
                    coyote_stopwatch.0.pause();
                    coyote_stopwatch.0.reset();
                } else if jump_count == 0 {
                    coyote_stopwatch.0.unpause();
                }
            }
        }
    }

    for (mut coyote_stopwatch, _) in query.iter_mut() {
        coyote_stopwatch.0.tick(time.delta());
    }
}
