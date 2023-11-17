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
        app.add_systems(Update, update_slash.run_if(in_state(GameState::Ready)));
        app.add_systems(Update, update_player_sprite.run_if(in_state(GameState::Ready)));
        //app.add_systems(Startup, spawn_player)
        //    .add_systems(Update, (update_player, level_up));
    }
}
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Slash(Timer);

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Component)]
pub struct StateTimer(Timer);

#[derive(Component)]
pub enum AnimationState {
    Idle,
    Moving,
    Attacking,
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
        AnimationState::Idle,
        AnimationTimer(Timer::from_seconds(0.15, TimerMode::Repeating)),
        StateTimer(Timer::from_seconds(0.3, TimerMode::Once)),
        Xp(0),
        Speed(3.5),
        Damage(10),
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
            offset: BarOffset::new(1.),
            ..default()
        },
    ));
}

pub fn update_player(
    mut players: Query<
        (
            &mut Transform,
            &Player,
            &Speed,
            &Unit,
            &mut AnimationState,
            &mut StateTimer,
            &Damage
        ),
        (Without<Camera>, Without<Enemy>),
    >,
    mut camera: Query<(&Camera, &mut Transform)>,
    mut settings: ResMut<PlayerSettings>,
    colliders: Query<(&Transform, &Collider), (Without<Unit>, Without<Camera>)>,
    mut enemies: Query<(&Enemy, &mut Health, &mut Transform), (Without<Player>, Without<Collider>, Without<Camera>)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut commands: Commands,
    assets: Res<MyAssets>,
    mut sprite_params: Sprite3dParams,
) {
    let dtime = time.delta_seconds();
    let (mut transform, _, speed, unit, mut state, mut state_timer, damage) = players.single_mut();
    let mut direction = Vec3::ZERO;
    if input.pressed(KeyCode::W) {
        direction.z -= 1.;
    }
    if input.pressed(KeyCode::S) {
        direction.z += 1.;
    }
    if input.pressed(KeyCode::D) {
        direction.x += 1.;
        transform.rotation = Quat::from_xyzw(0., 1., 0., 0.);
    }
    if input.pressed(KeyCode::A) {
        direction.x -= 1.;
        transform.rotation = Quat::from_xyzw(0., 0., 0., 1.);
    }
    if input.just_pressed(KeyCode::Y) {
        settings.camera_locked = !settings.camera_locked;
    }

    // detect changes in X-axis movement
    if !matches!(*state, AnimationState::Attacking) {
        if direction == Vec3::ZERO {
            *state = AnimationState::Idle;
        } else {
            *state = AnimationState::Moving;
        }
    }

    // if pressed attack key and not currently attacking
    if input.just_pressed(KeyCode::E) && !matches!(*state, AnimationState::Attacking) {
        *state = AnimationState::Attacking;
        *state_timer = StateTimer(Timer::from_seconds(0.3, TimerMode::Once));
        commands.spawn((
            Slash(Timer::from_seconds(0.3, TimerMode::Once)),
            Unit {
                size: Vec2::new(0.5, 0.5),
            },
            AtlasSprite3d {
                atlas: assets.slash.clone(),
                pixels_per_metre: 16.0,
                index: 0 as usize,
                unlit: true,
                transform: Transform {
                    translation: transform.translation + transform.left() * 1.5,
                    ..*transform
                },
                ..default()
            }
            .bundle(&mut sprite_params),
        ));
        // do attacking stuff

        for (_, mut enemy_health, mut enemy_transform) in enemies.iter_mut() {
            let player_transform = &(*transform);

            let player_direction = player_transform.rotation.mul_vec3(Vec3::new(-1., 0., 0.));
            info!("Player direction: {}", player_direction);

            let distance_to_enemy = enemy_transform.translation - player_transform.translation;
            info!("Distance to enemey: {}", distance_to_enemy);

            let angle = player_direction.dot(distance_to_enemy.normalize_or_zero());
            info!("Angle: {}", angle);

            if angle > 0.5 && distance_to_enemy.length() < 3.0 {
                let player_damage = damage.0;
                info!("Damage!");
                if enemy_health.current < player_damage {
                    info!("Enemy dead!");
                    enemy_health.current = 0;
                } else {
                    enemy_health.current -= player_damage;
                }
                knockback_enemy(&mut enemy_transform, player_transform);
            }
        }

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

pub fn update_player_sprite(
    mut players: Query<
        (
            &mut AnimationState,
            &mut StateTimer,
            &mut AtlasSprite3dComponent,
            &mut AnimationTimer,
        ),
        With<Player>,
    >,
    time: Res<Time>,
) {
    let (mut state, mut state_timer, mut atlas, mut timer) = players.single_mut();
    timer.tick(time.delta());

    match *state {
        AnimationState::Idle => {
            if timer.just_finished() {
                atlas.index = (atlas.index + 1) % 2;
            }
        }
        AnimationState::Moving => {
            if timer.just_finished() {
                atlas.index = (atlas.index + 1) % 4 + 4;
            }
        }
        AnimationState::Attacking => {
            state_timer.0.tick(time.delta());
            if timer.0.just_finished() {
                atlas.index = if atlas.index == 2 { 3 } else { 2 };
            }
            if state_timer.0.finished() {
                *state = AnimationState::Idle;
            }
        }
    };
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


fn knockback_enemy(enemy_transform: &mut Transform, player_transform: &Transform) {
    let direction = enemy_transform.translation - player_transform.translation;
    let direction = direction.normalize_or_zero();
    enemy_transform.translation += direction
}

fn update_slash(mut commands: Commands,mut slashes: Query<(Entity, &mut Slash, &mut AtlasSprite3dComponent)>, time: Res<Time>) {

    for (entity, mut slash, mut atlas) in &mut slashes {
        slash.0.tick(time.delta());
        if slash.0.finished() {
            if atlas.index == 1 {
                commands.entity(entity).despawn();
            } else {
                slash.0.reset();
                atlas.index += 1;
            }
        }
    }
}
