#![allow(clippy::type_complexity)]

use bevy::prelude::*;
use collisions::{
    CollisionInfo, CollisionsPlugin, CoyoteStopwatch, JumpCount, PlayerVelocity, RaySpacing,
    RaycastOrigins,
};
pub use impacted;
use input::VelocityXSmoothing;

pub use crate::controller::CharacterController;
use crate::controller::ControllerPlugin;
use crate::input::InputPlugin;

pub mod collisions;
pub mod controller;
mod input;
mod ray_cast;

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ControllerPlugin)
            .add_plugin(InputPlugin)
            .add_plugin(CollisionsPlugin);
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, SystemLabel)]
enum ControllerLabel {
    Calculate,
    Collisions,
    Move,
}

#[derive(Bundle, Default)]
pub struct CharacterControllerBundle {
    pub controller: CharacterController,
    pub jump_count: JumpCount,
    pub coyote_stopwatch: CoyoteStopwatch,
    pub velocity_x_smoothing: VelocityXSmoothing,
    pub velocity: PlayerVelocity,
    pub ray_spacing: RaySpacing,
    pub collisions: CollisionInfo,
    pub raycast_origins: RaycastOrigins,
}

impl CharacterControllerBundle {
    pub fn new(controller: CharacterController) -> Self {
        CharacterControllerBundle {
            controller,
            ..Default::default()
        }
    }
}

// taken from https://github.com/Unity-Technologies/UnityCsReference/blob/master/Runtime/Export/Math/Mathf.cs
fn smooth_damp(
    current: f32,
    mut target: f32,
    current_velocity: &mut f32,
    mut smooth_time: f32,
    max_speed: f32,
    delta_time: f32,
) -> f32 {
    smooth_time = smooth_time.max(0.0001);
    let omega = 2.0 / smooth_time;

    let x = omega * delta_time;
    let exp = 1.0 / (1.0 + x + 0.48 * x * x + 0.235 * x * x * x);
    let mut change = current - target;
    let original_to = target;

    // Clamp maximum speed
    let max_change = max_speed * smooth_time;
    change = change.clamp(-max_change, max_change);
    target = current - change;

    let temp = (*current_velocity + omega * change) * delta_time;
    *current_velocity = (*current_velocity - omega * temp) * exp;
    let mut output = target + (change + temp) * exp;

    // Prevent overshooting
    if (original_to - current > 0.0) == (output > original_to) {
        output = original_to;
        *current_velocity = (output - original_to) / delta_time;
    }

    output
}
