use std::fmt::Display;

use crate::*;

use super::collider::*;
use super::unit::*;
use bevy::prelude::*;
use bevy_sprite3d::Sprite3d;
use bevy_sprite3d::Sprite3dParams;
pub struct PlayerPlugin;

#[derive(Resource)]
pub struct PlayerSettings
{
    camera_locked: bool,
}

impl Plugin for PlayerPlugin
{
    fn build(&self, app: &mut App)
    {
        app.insert_resource(PlayerSettings {
            camera_locked: true,
        });
        app.add_systems(OnEnter(GameState::Spawning), spawn_player);
        app.add_systems(Update, update_player.run_if(in_state(GameState::Ready)));
        app.add_systems(
            Update,
            (update_player_sprite, swipe_attack).run_if(in_state(GameState::Ready)),
        );
        //app.add_systems(Startup, spawn_player)
        //    .add_systems(Update, (update_player, level_up));
    }
}
#[derive(Component)]
pub struct Player;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Component)]
pub struct AttackTimer(Timer);

#[derive(Component)]
pub enum AnimationState
{
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
)
{
    commands.spawn((
        Player,
        Health {
            current: 100,
            max: 125,
        },
        AnimationState::Idle,
        AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
        AttackTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
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
            &mut AttackTimer,
        ),
        Without<Camera>,
    >,
    mut camera: Query<(&Camera, &mut Transform)>,
    mut settings: ResMut<PlayerSettings>,
    colliders: Query<(&Transform, &Collider), (Without<Unit>, Without<Camera>)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
)
{
    let dtime = time.delta_seconds();
    for (mut transform, _, speed, unit, mut state, attack_timer) in &mut players
    {
        let mut direction = Vec3::ZERO;
        if input.pressed(KeyCode::W)
        {
            direction.z -= 1.;
            // transform.rotate(Quat::from_rotation_y(2. * std::f32::consts::PI * dtime));
        }
        if input.pressed(KeyCode::S)
        {
            direction.z += 1.;
        }
        if input.pressed(KeyCode::D)
        {
            direction.x += 1.;
            transform.rotation = Quat::from_xyzw(0., 1., 0., 0.);
        }
        if input.pressed(KeyCode::A)
        {
            direction.x -= 1.;
            transform.rotation = Quat::from_xyzw(0., 0., 0., 1.);
        }
        if input.just_pressed(KeyCode::Y)
        {
            settings.camera_locked = !settings.camera_locked;
        }

        // detect changes in X-axis movement
        if matches!(*state, AnimationState::Attacking)
        {
            continue;
        }
        if direction == Vec3::ZERO
        {
            *state = AnimationState::Idle;
        }
        else
        {
            *state = AnimationState::Moving;
        }
        let direction = direction.normalize_or_zero();

        if input.just_pressed(KeyCode::E) && attack_timer.0.elapsed_secs() == 0.0
        {
            info!("Test");
            *state = AnimationState::Attacking;
            // do attacking
        }
        else
        {
            unit.move_and_slide(&mut transform, direction, speed, &colliders, dtime);
        }
        // move camera on top of player
        if settings.camera_locked || input.pressed(KeyCode::Space)
        {
            for (_, mut camera_transform) in &mut camera
            {
                camera_transform.translation = transform.translation + CAMERA_OFFSET;
            }
        }
    }
}

pub fn update_player_sprite(
    mut players: Query<(
        &Player,
        &mut AnimationState,
        &mut AtlasSprite3dComponent,
        &mut AnimationTimer,
        &mut AttackTimer,
    )>,
    time: Res<Time>,
)
{
    for (_, mut state, mut atlas, mut timer, mut attack_timer) in &mut players
    {
        timer.tick(time.delta());
        if !timer.just_finished()
        {
            continue;
        }
        match *state
        {
            AnimationState::Idle => atlas.index = (atlas.index + 1) % 2,
            AnimationState::Moving =>
            {
                atlas.index = (atlas.index + 1) % 4 + 4;
            }
            AnimationState::Attacking =>
            {
                if atlas.index == 2
                {
                    atlas.index = 3;
                }
                else
                {
                    atlas.index = 2;
                }
                attack_timer.0.tick(time.delta());
                if timer.just_finished()
                {
                    *state = AnimationState::Idle;
                    attack_timer.0.reset();
                }
            }
        }
    }
}

fn level_up(
    // operate on anything that has Xp and Health
    mut query: Query<(&mut Xp, &mut Health, &mut Speed)>,
)
{
    for (mut xp, mut health, mut speed) in query.iter_mut()
    {
        if xp.0 >= 1000
        {
            xp.0 -= 1000;
            health.max += 25;
            health.current = health.max;
            speed.0 += 10.0;
        }
    }
}

fn swipe_attack(
    player: Query<(&Player, &Transform, &Damage)>,
    mut enemies: Query<(&Enemy, &mut Health, &mut Transform), Without<Player>>,
    input: Res<Input<KeyCode>>,
)
{
    let player = player.single();
    if input.just_pressed(KeyCode::Space)
    {
        for (_, mut enemy_health, mut enemy_transform) in enemies.iter_mut()
        {
            let player_transform = player.1;

            let player_direction = player_transform.rotation.mul_vec3(Vec3::new(-1., 0., 0.));
            info!("Player direction: {}", player_direction);

            let distance_to_enemy = enemy_transform.translation - player_transform.translation;
            info!("Distance to enemey: {}", distance_to_enemy);

            let angle = player_direction.dot(distance_to_enemy.normalize_or_zero());
            info!("Angle: {}", angle);

            if angle > 0.5 && distance_to_enemy.length() < 3.0
            {
                let player_damage = player.2 .0;
                info!("Damage!");
                if enemy_health.current < player_damage
                {
                    info!("Enemy dead!");
                    enemy_health.current = 0;
                }
                else
                {
                    enemy_health.current -= player_damage;
                }
                knockback_enemy(&mut enemy_transform, player_transform);
            }
        }
    }
}

fn knockback_enemy(enemy_transform: &mut Transform, player_transform: &Transform)
{
    let direction = enemy_transform.translation - player_transform.translation;
    let direction = direction.normalize_or_zero();
    enemy_transform.translation += direction
}
