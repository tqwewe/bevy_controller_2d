use bevy::prelude::*;
use bevy_controller_2d::impacted::CollisionShape;
use bevy_controller_2d::{
    CharacterController, CharacterControllerBundle, CharacterControllerPlugin,
};
use bevy_inspector_egui::{Inspectable, InspectorPlugin};

#[derive(Inspectable, Clone, Copy)]
struct Config {
    #[inspectable(label = "Move speed", min = 0.0, max = 1000.0)]
    move_speed: f32,
    #[inspectable(label = "Jump height", min = 0.0, max = 10.0)]
    jump_height: f32,
    #[inspectable(label = "Time to jump apex", min = 0.0, max = 2.0)]
    time_to_jump_apex: f32,
    #[inspectable(label = "Acceleration time (grounded)", min = 0.0, max = 2.0)]
    acceleration_time_grounded: f32,
    #[inspectable(label = "Acceleration time (airborne)", min = 0.0, max = 2.0)]
    acceleration_time_airborne: f32,
    #[inspectable(label = "Gravity up multiplier", min = 0.0, max = 5.0)]
    gravity_up_multiplier: f32,
    #[inspectable(label = "Gravity down multiplier", min = 0.0, max = 5.0)]
    gravity_down_multiplier: f32,
    #[inspectable(label = "Coyote time", min = 0.0, max = 1.0)]
    coyote_time: f32,
    #[inspectable(label = "Skin width", min = 0.0, max = 5.0)]
    skin_width: f32,
    #[inspectable(label = "Horizontal ray count", min = 0, max = 20)]
    horizontal_ray_count: u8,
    #[inspectable(label = "Vertical ray count", min = 0, max = 20)]
    vertical_ray_count: u8,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            move_speed: 400.0,
            jump_height: 2.0,
            time_to_jump_apex: 0.4,
            acceleration_time_grounded: 0.1,
            acceleration_time_airborne: 0.2,
            gravity_up_multiplier: 1.0,
            gravity_down_multiplier: 1.5,
            coyote_time: 0.1,
            skin_width: 1.0,
            horizontal_ray_count: 8,
            vertical_ray_count: 4,
        }
    }
}

impl From<Config> for CharacterController {
    fn from(config: Config) -> Self {
        CharacterController {
            move_speed: config.move_speed,
            jump_height: config.jump_height,
            time_to_jump_apex: config.time_to_jump_apex,
            acceleration_time_grounded: config.acceleration_time_grounded,
            acceleration_time_airborne: config.acceleration_time_airborne,
            gravity_up_multiplier: config.gravity_up_multiplier,
            gravity_down_multiplier: config.gravity_down_multiplier,
            coyote_time: config.coyote_time,
            skin_width: config.skin_width,
            horizontal_ray_count: config.horizontal_ray_count,
            vertical_ray_count: config.vertical_ray_count,
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(InspectorPlugin::<Config>::new())
        .add_plugin(CharacterControllerPlugin)
        .add_system(inspector_update)
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_world)
        .add_startup_system(spawn_player)
        .run();
}

fn inspector_update(config: Res<Config>, mut controllers: Query<&mut CharacterController>) {
    if config.is_changed() {
        for mut controller in controllers.iter_mut() {
            *controller = (*config).into();
        }
    }
}

fn setup_camera(mut commands: Commands) {
    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    camera_bundle.transform.translation.y = 250.0;
    commands.spawn_bundle(camera_bundle);
}

fn spawn_player(config: Res<Config>, mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(25.0, 50.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 30.0, 1.0),
            ..Default::default()
        })
        .insert(CollisionShape::new_rectangle(25.0, 50.0))
        .insert_bundle(CharacterControllerBundle::new((*config).into()))
        .insert(Name::new("player"));
}

fn spawn_world(mut commands: Commands) {
    // Floor
    let w = 1000.0;
    let h = 50.0;
    let x = 0.0;
    let y = -h / 2.0;
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::MAROON,
                custom_size: Some(Vec2::new(w, h)),
                ..Default::default()
            },
            transform: Transform::from_xyz(x, y, 0.0),
            ..Default::default()
        })
        .insert(CollisionShape::new_rectangle(w, h));

    // Left wall
    let w = 50.0;
    let h = 500.0;
    let x = -500.0 + w / 2.0;
    let y = h / 2.0;
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::MAROON,
                custom_size: Some(Vec2::new(w, h)),
                ..Default::default()
            },
            transform: Transform::from_xyz(x, y, 0.0),
            ..Default::default()
        })
        .insert(CollisionShape::new_rectangle(w, h));

    // Right wall
    let w = 50.0;
    let h = 500.0;
    let x = 500.0 + w / -2.0;
    let y = h / 2.0;
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::MAROON,
                custom_size: Some(Vec2::new(w, h)),
                ..Default::default()
            },
            transform: Transform::from_xyz(x, y, 0.0),
            ..Default::default()
        })
        .insert(CollisionShape::new_rectangle(w, h));

    // Middle wall
    let w = 100.0;
    let h = 500.0;
    let x = 0.0;
    let y = h / 2.0 + 75.0;
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::MAROON,
                custom_size: Some(Vec2::new(w, h)),
                ..Default::default()
            },
            transform: Transform::from_xyz(x, y, 0.0),
            ..Default::default()
        })
        .insert(CollisionShape::new_rectangle(w, h));

    // Left ledge 1
    let w = 100.0;
    let h = 10.0;
    let x = -500.0 + 50.0 + 50.0;
    let y = 100.0;
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::ORANGE,
                custom_size: Some(Vec2::new(w, h)),
                ..Default::default()
            },
            transform: Transform::from_xyz(x, y, 0.0),
            ..Default::default()
        })
        .insert(CollisionShape::new_rectangle(w, h));

    // Left ledge 2
    let w = 100.0;
    let h = 10.0;
    let x = -100.0;
    let y = 200.0;
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::ORANGE,
                custom_size: Some(Vec2::new(w, h)),
                ..Default::default()
            },
            transform: Transform::from_xyz(x, y, 0.0),
            ..Default::default()
        })
        .insert(CollisionShape::new_rectangle(w, h));

    // Left ledge 3
    let w = 250.0;
    let h = 10.0;
    let x = -325.0;
    let y = 350.0;
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::ORANGE,
                custom_size: Some(Vec2::new(w, h)),
                ..Default::default()
            },
            transform: Transform::from_xyz(x, y, 0.0),
            ..Default::default()
        })
        .insert(CollisionShape::new_rectangle(w, h));

    for i in 1..6 {
        // Right ledge
        let w = 200.0;
        let h = 10.0;
        let x = (50.0 + w / 2.0) + (w * (i % 2) as f32);
        let y = (100.0 * i as f32) - 20.0;
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::ORANGE,
                    custom_size: Some(Vec2::new(w, h)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(x, y, 0.0),
                ..Default::default()
            })
            .insert(CollisionShape::new_rectangle(w, h));
    }
}
