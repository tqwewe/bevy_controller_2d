use bevy::prelude::*;

use crate::{
    collisions::{CollisionInfo, CoyoteStopwatch, JumpCount, PlayerVelocity},
    smooth_damp, CharacterController, ControllerLabel,
};

pub(crate) struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<VelocityXSmoothing>()
            .add_system(player_input.label(ControllerLabel::Calculate));
    }
}

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct VelocityXSmoothing(f32);

fn player_input(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<(
        &CharacterController,
        &mut PlayerVelocity,
        &mut VelocityXSmoothing,
        &mut JumpCount,
        &CoyoteStopwatch,
        &CollisionInfo,
    )>,
) {
    for (
        controller,
        mut velocity,
        mut vel_x_smoothing,
        mut jump_count,
        coyote_stopwatch,
        collisions,
    ) in query.iter_mut()
    {
        let gravity = -(2.0 * controller.jump_height) / controller.time_to_jump_apex.powi(2);
        let jump_velocity = gravity.abs() * controller.time_to_jump_apex;

        // Reset Y velocity if touching above or below
        if collisions.below || collisions.above {
            velocity.0.y = 0.0;
        }

        let mut input_raw = Vec2::ZERO;

        // Horizontal movement
        if input.pressed(KeyCode::Left) && !input.pressed(KeyCode::Right) {
            input_raw.x = -1.0;
        } else if input.pressed(KeyCode::Right) && !input.pressed(KeyCode::Left) {
            input_raw.x = 1.0;
        }

        // Jumping
        if (input.just_pressed(KeyCode::Up) || input.just_pressed(KeyCode::Space))
            && (collisions.below || coyote_stopwatch.0.elapsed_secs() <= controller.coyote_time)
            && jump_count.0 == 0
        {
            jump_count.0 += 1;
            velocity.0.y = jump_velocity;
        }

        // Smooth x movement
        let target_velocity_x = input_raw.x * controller.move_speed * time.delta_seconds();
        let acceleration_time = if collisions.below {
            controller.acceleration_time_grounded
        } else {
            controller.acceleration_time_airborne
        };
        velocity.0.x = smooth_damp(
            velocity.0.x,
            target_velocity_x,
            &mut vel_x_smoothing.0,
            acceleration_time,
            f32::INFINITY,
            time.delta_seconds(),
        );

        // Apply gravity
        let gravity_multiplier = if velocity.0.y > 0.0 {
            // controller.
            controller.gravity_up_multiplier
        } else if velocity.0.y < 0.0 {
            controller.gravity_down_multiplier
        } else {
            1.0
        };
        velocity.0.y += gravity * gravity_multiplier * time.delta_seconds();
    }
}
