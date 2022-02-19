# Bevy Controller 2D

Character controller plugin for Bevy.

### Configuration

- Movement speed _(default: `400`)_
- Jump height _(default: `2.0`)_
- Time in seconds to reach jump apex (top of jump) _(default: `0.4`)_
- Time in seconds to accelerate to move speed when grounded _(default: `0.1`)_
- Time in seconds to accelerate to move speed when airborne _(default: `0.2`)_
- Gravity multiplier when jumpin up _(default: `1.0`)_
- Gravity multiplier when falling down _(default: `1.5`)_
- Time in seconds after leaving a platform to allow for a jump _(default: `0.08`)_
- Ray casting inset _(default: `1.0`)_
- Horizontal ray count _(default: `6`)_
- Vertical ray count _(default: `4`)_

<p align="center">
  <img src="https://raw.githubusercontent.com/tqwewe/bevy_controller_2d/main/preview.gif" width="100%">
</p>

### Install

Add dependency to Cargo.toml

```toml
bevy_controller_2d = { git = "https://github.com/tqwewe/bevy_controller_2d", features = ["debug"] } # Add debug to view ray casts
```

Add `CharacterControllerPlugin` plugin

```rust
use bevy::prelude::*;
use bevy_controller_2d::impacted::CollisionShape;
use bevy_controller_2d::{CharacterControllerBundle, CharacterControllerPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CharacterControllerPlugin)
        .add_startup_system(setup_player)
        .add_startup_system(setup_ground)
        .run();
}
```

Spawn player with `CharacterControllerBundle` and `CollisionShape`

```rust
fn setup_player(mut commands: Commands) {
    let width = 25.0;
    let height = 50.0;

    commands
        // Sprite so we can see the player
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(width, height)),
                ..Default::default()
            },
            ..Default::default()
        })
        // Collision rectangle
        .insert(CollisionShape::new_rectangle(width, height))
        // Character controller bundle
        .insert_bundle(CharacterControllerBundle::default());
}
```

Spawn ground with `CollisionShape`

```rust
fn setup_ground(mut commands: Commands) {
    let width = 100.0;
    let height = 20.0;

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::DARK_GRAY,
                custom_size: Some(Vec2::new(ground_width, ground_height)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, -ground_height, 0.0),
            ..Default::default()
        })
        .insert(CollisionShape::new_rectangle(ground_width, ground_height));
}
```

### Examples

- `basic`

  ```bash
  cargo run --features debug --example basic
  ```

- `advanced`

  ```bash
  cargo run --features debug --example advanced
  ```

### Known issues

- Doesn't work with slopes
- Jump height seems to be affected with low frame rate

### Credits

Thanks to [Sebastian Lague](https://www.youtube.com/channel/UCmtyQOKKmrMVaKuRXz02jbQ), I was able to implement this following his [2D platformer tutorial series](https://www.youtube.com/playlist?list=PLFt_AvWsXl0f0hqURlhyIoAabKPgRsqjz).
