use super::collider::*;
use super::unit::*;
use bevy::prelude::*;
pub struct PlayerPlugin;

#[derive(Resource)]
pub struct PlayerSettings {
    camera_locked: bool,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerSettings {
            camera_locked: true,
        });
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (update_player, level_up));
    }
}
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Xp(pub u32);

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player,
        Health {
            current: 100,
            max: 125,
        },
        Xp(0),
        Speed(100.),
        Unit {
            size: Vec2::new(16., 30.),
        },
        SpriteBundle {
            texture: asset_server.load("man_transp.png"),
            transform: Transform::from_xyz(0., 0., 0.),
            ..Default::default()
        },
    ));
}

pub fn update_player(
    mut players: Query<(&mut Transform, &Player, &Speed, &Unit), Without<Camera>>,
    mut camera: Query<(&Camera, &mut Transform)>,
    mut settings: ResMut<PlayerSettings>,
    colliders: Query<(&Transform, &Collider), (Without<Unit>, Without<Camera>)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let dtime = time.delta_seconds();
    for (mut transform, _, speed, unit) in &mut players {
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
        if input.just_pressed(KeyCode::Y) {
            settings.camera_locked = !settings.camera_locked;
        }

        let direction = direction.normalize_or_zero();

        unit.move_and_slide(&mut transform, direction, speed, &colliders, dtime);

        // move camera on top of player
        if settings.camera_locked || input.pressed(KeyCode::Space) {
            for (_, mut camera_transform) in &mut camera {
                camera_transform.translation = transform.translation;
            }
        }
    }
}

fn level_up(
    // operate on anything that has Xp and Health
    mut query: Query<(&mut Xp, &mut Health, &mut Speed)>,
) {
    for (mut xp, mut health, mut speed) in query.iter_mut() {
        if xp.0 >= 1000 {
            xp.0 -= 1000;
            health.max += 25;
            health.current = health.max;
            speed.0 += 10.0;
        }
    }
}
