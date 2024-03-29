use bevy::prelude::*;
use bevy_health_bar3d::configuration::{BarBundle, BarOffset, BarWidth};
use bevy_sprite3d::AtlasSprite3d;
use bevy_sprite3d::AtlasSprite3dComponent;
use bevy_sprite3d::Sprite3dParams;
use rand::RngCore;

use crate::assets::MyAssets;
use crate::entities::collider::Collider;
use crate::entities::enemy::components::Enemy;
use crate::entities::unit::*;
use crate::world::CHUNK_RADIUS;
use crate::world::components::TileType;
use crate::world::resources::WorldData;
use crate::world::systems::get_chunk_pos;
use crate::world::CHUNK_SIDE;

use super::components::*;
use super::resources::*;
use super::CAMERA_OFFSET;

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
            &Damage,
        ),
        (Without<Camera>, Without<Enemy>),
    >,
    mut camera: Query<(&Camera, &mut Transform)>,
    mut settings: ResMut<PlayerSettings>,
    colliders: Query<(&Transform, &Collider), (Without<Unit>, Without<Camera>)>,
    mut enemies: Query<
        (&Enemy, &mut Health, &mut Transform),
        (Without<Player>, Without<Collider>, Without<Camera>),
    >,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut commands: Commands,
    world: Res<WorldData>,
    assets: Res<MyAssets>,
    mut sprite_params: Sprite3dParams,
) {
    let dtime = time.delta_seconds();
    let (mut transform, _, speed, unit, mut state, mut state_timer, damage) = players.single_mut();

    let direction = get_direction_vector(&input, &mut transform);

    // detect changes in X-axis movement
    update_movement_state(&mut state, direction);

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

        attack_enemy(enemies, &transform, damage);
    }

    if input.pressed(KeyCode::F) {
        // already charging
        if matches!(*state, AnimationState::FishingCharging) {
            state_timer.0.tick(time.delta());
        } else {
            // start charging
            *state_timer = StateTimer(Timer::from_seconds(1.0, TimerMode::Once));
            *state = AnimationState::FishingCharging;
        }
    } else {
        if matches!(*state, AnimationState::FishingCharging) {
            info!("Charged for {}", state_timer.0.elapsed_secs());
            

            let fishing_translation =
                transform.translation + transform.left() * state_timer.0.elapsed_secs() * 5.;
            let chunk_pos = get_chunk_pos(fishing_translation.xz());

            match world.chunks.get(&chunk_pos) {
                Some(chunk) => {
                    let x_inside = fishing_translation.x / 2. - (chunk_pos.0 * CHUNK_SIDE - CHUNK_RADIUS) as f32 + 0.5;
                    let z_inside = fishing_translation.z / 2. - (chunk_pos.1 * CHUNK_SIDE - CHUNK_RADIUS) as f32 + 0.5;

                    let mut i = (x_inside).floor() as usize;
                    let mut j = (z_inside).floor() as usize;
                    if j > 5 {j = 4;}
                    if i > 5 {i = 4;}
                    if let TileType::Water = chunk.tiles[i][j]
                    {
                        *state = AnimationState::Fishing;
                        commands.spawn((
                            FishingFloat(Timer::from_seconds(0.3, TimerMode::Once)),
                            AtlasSprite3d {
                                atlas: assets.float.clone(),
                                pixels_per_metre: 16.0,
                                index: 0 as usize,
                                unlit: true,
                                transform: Transform {
                                    translation: Vec3 {
                                        y: 0.5,
                                        ..fishing_translation
                                    },
                                    ..*transform
                                },
                                ..default()
                            }
                            .bundle(&mut sprite_params),
                        ));
                    } else {
                        *state = AnimationState::Idle;
                    }
                }
                None => *state = AnimationState::Idle,
            }

            
        }
    }


    let direction = direction.normalize_or_zero();

    unit.move_and_slide(&mut transform, direction, speed, &colliders, dtime);

    // move camera on top of player
    if settings.camera_locked {
        for (_, mut camera_transform) in &mut camera {
            camera_transform.translation = transform.translation + CAMERA_OFFSET;
        }
    }
}

fn attack_enemy(
    mut enemies: Query<
        (&Enemy, &mut Health, &mut Transform),
        (Without<Player>, Without<Collider>, Without<Camera>),
    >,
    transform: &Mut<Transform>,
    damage: &Damage,
) {
    for (_, mut enemy_health, mut enemy_transform) in enemies.iter_mut() {
        let player_transform = &(**transform);

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

fn get_direction_vector(input: &Input<KeyCode>, transform: &mut Transform) -> Vec3 {
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
    direction
}

fn update_movement_state(state: &mut AnimationState, direction: Vec3) {
    if matches!(*state, AnimationState::Moving) || matches!(*state, AnimationState::Idle) {
        if direction == Vec3::ZERO {
            *state = AnimationState::Idle;
        } else {
            *state = AnimationState::Moving;
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
        AnimationState::FishingCharging => atlas.index = 0,
        AnimationState::Fishing => atlas.index = 1,
    };
}

pub fn level_up(
    // operate on anything that has Xp and Health
    mut query: Query<(&mut Xp, &mut Health, &mut Speed, &mut Damage)>,
) {
    for (mut xp, mut health, mut speed, mut damage) in query.iter_mut() {
        if xp.0 >= 1000 {
            xp.0 -= 1000;
            health.max += 25;
            health.current = health.max;
            damage.0 += 20;
            speed.0 += 10.0;
        }
    }
}

fn knockback_enemy(enemy_transform: &mut Transform, player_transform: &Transform) {
    let direction = enemy_transform.translation - player_transform.translation;
    let direction = direction.normalize_or_zero();
    enemy_transform.translation += direction
}

pub fn update_slash(
    mut commands: Commands,
    mut slashes: Query<(Entity, &mut Slash, &mut AtlasSprite3dComponent)>,
    time: Res<Time>,
) {
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


pub fn catch_fish(
    mut players: Query<
        (
            &Player,
            &mut AnimationState,
            &mut Xp
        ),
        Without<Camera>,
    >,
    mut floats: Query<(&FishingFloat, &mut AtlasSprite3dComponent), Without<Player>>,
    input: Res<Input<KeyCode>>,
) {
    let (_, mut state, mut xp) = players.single_mut();
    if let Ok((_, mut atlas)) = floats.get_single_mut() {
        if input.pressed(KeyCode::Space) {
            if matches!(*state, AnimationState::Fishing) {

                if atlas.index == 2 || atlas.index == 3 {
                    *state = AnimationState::Idle;
                    xp.0 += 500;
                    info!("Caught FISH!!!!!!");
                }                 
                atlas.index = 4;
    
                *state = AnimationState::Idle;
            }
        }
    }
}


pub fn update_fishing_float(
    mut commands: Commands,
    mut floats: Query<(Entity, &mut FishingFloat, &mut AtlasSprite3dComponent)>,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();
    for (entity, mut float, mut atlas) in &mut floats {
        float.0.tick(time.delta());
        if float.0.finished() {
            if atlas.index == 4 {
                commands.entity(entity).despawn();
            } else {
                float.0.reset();
                if atlas.index == 1 {
                    if rng.next_u32() % 10 == 0 {
                        atlas.index = 2;
                    } else {
                        atlas.index = 0;
                    }
                } else {
                    atlas.index += 1;
                    atlas.index %= 4;
                }
            }
        }
    }
}
