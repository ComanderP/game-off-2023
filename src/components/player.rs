

use std::fmt::Display;

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
        app.add_systems(Update, update_player_sprite.run_if(in_state(GameState::Ready)));
        //app.add_systems(Startup, spawn_player)
        //    .add_systems(Update, (update_player, level_up));
    }
}
#[derive(Component)]
pub struct Player;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Component)]
pub enum AnimationState {
    IdleLeft,
    IdleRight,
    MovingLeft,
    MovingRight,
}

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
        AnimationState::IdleLeft,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Xp(0),
        Speed(3.5),
        Unit {
            size: Vec2::new(0.5, 0.5),
        },
        AtlasSprite3d {
            atlas: assets.player_moving.clone(),
            pixels_per_metre: 16.0,
            index: 0 as usize,
            unlit: true,
            transform: Transform::from_translation(Vec3::new(0., 1., 0.)),
            ..default()
        }
        .bundle(&mut sprite_params),
        BarBundle::<Health> {
            width: BarWidth::new(1.),
            offset: BarOffset::new(2.),
            ..default()
        },
    ));
}

pub fn update_player(
    mut players: Query<(&mut Transform, &Player, &Speed, &Unit, &mut AnimationState), Without<Camera>>,
    mut camera: Query<(&Camera, &mut Transform)>,
    mut settings: ResMut<PlayerSettings>,
    colliders: Query<(&Transform, &Collider), (Without<Unit>, Without<Camera>)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let dtime = time.delta_seconds();
    for (mut transform, _, speed, unit, mut state) in &mut players {
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

        // detect changes in X-axis movement
        if direction.x == 1. {
            *state = AnimationState::MovingRight;
        } else if direction.x == -1. {
            *state = AnimationState::MovingLeft;
        } else if direction == Vec3::ZERO {
            let next_state = match *state {
                AnimationState::MovingLeft => AnimationState::IdleLeft,
                AnimationState::MovingRight => AnimationState::IdleRight,
                AnimationState::IdleLeft => AnimationState::IdleLeft,
                AnimationState::IdleRight => AnimationState::IdleRight,
            };

            *state = next_state;
        } else {
            let next_state = match *state {
                AnimationState::MovingLeft => AnimationState::MovingLeft,
                AnimationState::MovingRight => AnimationState::MovingRight,
                AnimationState::IdleLeft => AnimationState::MovingLeft,
                AnimationState::IdleRight => AnimationState::MovingRight,
            };

            *state = next_state;
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

pub fn update_player_sprite(
    mut players: Query<(&Player, &mut AnimationState, &mut AtlasSprite3dComponent, &mut AnimationTimer)>,
    time: Res<Time>
) {
    for (_, state, mut atlas, mut timer) in &mut players {
        timer.tick(time.delta());
        if !timer.just_finished() {
            continue;
        }
        match *state {
            AnimationState::IdleLeft => atlas.index = 0,
            AnimationState::IdleRight => atlas.index = 1,
            AnimationState::MovingLeft => {
                atlas.index = (atlas.index + 1) % 4 + 2;
            }
            AnimationState::MovingRight => {
                atlas.index = (atlas.index + 1) % 4 + 6;
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
