use bevy::prelude::*;
use bevy_controller_2d::impacted::CollisionShape;
use bevy_controller_2d::{CharacterControllerBundle, CharacterControllerPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CharacterControllerPlugin)
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_world)
        .add_startup_system(spawn_player)
        .run();
}

fn setup_camera(mut commands: Commands) {
    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    camera_bundle.transform.translation.y = 250.0;
    commands.spawn_bundle(camera_bundle);
}

fn spawn_player(mut commands: Commands) {
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
        .insert_bundle(CharacterControllerBundle::default())
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

    // Lower left platform
    let w = 300.0;
    let h = 10.0;
    let x = -500.0 + 50.0 + w / 2.0;
    let y = 80.0;
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::GOLD,
                custom_size: Some(Vec2::new(w, h)),
                ..Default::default()
            },
            transform: Transform::from_xyz(x, y, 0.0),
            ..Default::default()
        })
        .insert(CollisionShape::new_rectangle(w, h));

    // Lower right platform
    let w = 300.0;
    let h = 10.0;
    let x = 500.0 - 50.0 - w / 2.0;
    let y = 80.0;
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::GOLD,
                custom_size: Some(Vec2::new(w, h)),
                ..Default::default()
            },
            transform: Transform::from_xyz(x, y, 0.0),
            ..Default::default()
        })
        .insert(CollisionShape::new_rectangle(w, h));

    // Upper platform
    let w = 200.0;
    let h = 10.0;
    let x = 0.0;
    let y = 180.0;
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::GOLD,
                custom_size: Some(Vec2::new(w, h)),
                ..Default::default()
            },
            transform: Transform::from_xyz(x, y, 0.0),
            ..Default::default()
        })
        .insert(CollisionShape::new_rectangle(w, h));

    // Upper platform box
    let w = 50.0;
    let h = 50.0;
    let x = 0.0;
    let y = 185.0 + h / 2.0;
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::DARK_GRAY,
                custom_size: Some(Vec2::new(w, h)),
                ..Default::default()
            },
            transform: Transform::from_xyz(x, y, 0.0),
            ..Default::default()
        })
        .insert(CollisionShape::new_rectangle(w, h));
}
