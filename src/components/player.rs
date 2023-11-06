use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_update);
    }
}
#[derive(Component)]
pub struct Player;

pub fn spawn_player(
    // needed for creating/removing data in the ECS World
    mut commands: Commands,
    // needed for loading assets
    asset_server: Res<AssetServer>,
) {
    // create a new entity with whatever components we want
    commands.spawn((
        // give it a marker
        Player,
        // give it health and xp
        Health {
            current: 100,
            max: 125,
        },
        Xp(0),
        Speed(100.),
        // give it a 2D sprite to render on-screen
        // (Bevy's SpriteBundle lets us add everything necessary)
        SpriteBundle {
            texture: asset_server.load("man.png"),
            transform: Transform::from_xyz(25.0, 50.0, 0.0),
            // use the default values for all other components in the bundle
            ..Default::default()
        },
    ));
}

pub fn player_update(
    mut players: Query<(&mut Transform, &Player, &Speed)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player, speed) in &mut players {
        let speed = speed.0 * time.delta_seconds();
        let mut direction = Vec2::ZERO;
        if input.pressed(KeyCode::W) {
            direction.y += 1.;
        }
        if input.pressed(KeyCode::S) {
            direction.y -= 1.;
        }
        if input.pressed(KeyCode::D) {
            direction.x += 1.;
        }
        if input.pressed(KeyCode::A) {
            direction.x -= 1.;
        }
        let direction = direction.normalize_or_zero();
        transform.translation.x += (speed * direction).x;
        transform.translation.y += (speed * direction).y;
    }
}

#[derive(Component)]
struct Xp(u32);

#[derive(Component)]
pub struct Speed(f32);

#[derive(Component)]
struct Health {
    current: u32,
    max: u32,
}

fn level_up(
    // operate on anything that has Xp and Health
    mut query: Query<(&mut Xp, &mut Health)>,
) {
    for (mut xp, mut health) in query.iter_mut() {
        if xp.0 > 1000 {
            xp.0 -= 1000;
            health.max += 25;
            health.current = health.max;
        }
    }
}
