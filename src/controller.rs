use bevy::prelude::*;
#[cfg(feature = "debug")]
use bevy_prototype_lyon::prelude::*;

use crate::{collisions::PlayerVelocity, ControllerLabel};

pub(crate) struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "debug")]
        app.add_plugin(ShapePlugin);
        app.register_type::<CharacterController>().add_system(
            move_player
                .label(ControllerLabel::Move)
                .after(ControllerLabel::Collisions),
        );
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct CharacterController {
    /// Movement speed _(default: `400`)_
    pub move_speed: f32,
    /// Jump height _(default: `2.0`)_
    pub jump_height: f32,
    /// Time in seconds to reach jump apex (top of jump) _(default: `0.4`)_
    pub time_to_jump_apex: f32,
    /// Time in seconds to accelerate to move speed when grounded _(default: `0.1`)_
    pub acceleration_time_grounded: f32,
    /// Time in seconds to accelerate to move speed when airborne _(default: `0.2`)_
    pub acceleration_time_airborne: f32,
    /// Gravity multiplier when jumpin up _(default: `1.0`)_
    pub gravity_up_multiplier: f32,
    /// Gravity multiplier when falling down _(default: `1.5`)_
    pub gravity_down_multiplier: f32,
    /// Time in seconds after leaving a platform to allow for a jump _(default: `0.08`)_
    pub coyote_time: f32,
    /// Ray casting inset _(default: `1.0`)_
    pub skin_width: f32,
    /// Horizontal ray count _(default: `6`)_
    pub horizontal_ray_count: u8,
    /// Vertical ray count _(default: `4`)_
    pub vertical_ray_count: u8,
}

impl Default for CharacterController {
    fn default() -> Self {
        CharacterController {
            move_speed: 400.0,
            jump_height: 2.0,
            time_to_jump_apex: 0.4,
            acceleration_time_grounded: 0.1,
            acceleration_time_airborne: 0.2,
            gravity_up_multiplier: 1.0,
            gravity_down_multiplier: 1.5,
            coyote_time: 0.08,
            skin_width: 1.0,
            horizontal_ray_count: 6,
            vertical_ray_count: 4,
        }
    }
}

fn move_player(mut query: Query<(&PlayerVelocity, &mut Transform)>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.0.extend(0.0);
    }
}
