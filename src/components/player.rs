use crate::*;

use super::collider::*;
use super::unit::*;
use bevy::prelude::*;
use bevy_sprite3d::Sprite3d;
use bevy_sprite3d::Sprite3dParams;
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
        app.add_systems(OnEnter(GameState::Spawning), spawn_player);
        app.add_systems(Update, update_player.run_if(in_state(GameState::Ready)));
        //app.add_systems(Startup, spawn_player)
        //    .add_systems(Update, (update_player, level_up));
    }
}
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Xp(pub u32);

pub fn spawn_player(
    mut commands: Commands,
    assets: Res<MyAssets>,
    mut sprite_params: Sprite3dParams,
) {
    commands.spawn((
        Player,
        Health {
            current: 100,
            max: 125,
        },
        Xp(0),
        Speed(3.5),
        Unit {
            size: Vec2::new(0.5, 0.5),
        },
        Sprite3d {
            image: assets.player.clone(),
            pixels_per_metre: 16.0,
            unlit: true,
            transform: Transform::from_translation(Vec3::new(0., 1., 0.)),
            ..default()
        }
        .bundle(&mut sprite_params),
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
        let mut direction = Vec3::ZERO;
        if input.pressed(KeyCode::W) {
            direction.z -= 1.;
        }
        if input.pressed(KeyCode::S) {
            direction.z += 1.;
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
                camera_transform.translation = transform.translation + CAMERA_OFFSET;
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
