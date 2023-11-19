use std::f32::consts::E;

use bevy::prelude::*;
use bevy_health_bar3d::prelude::*;
use bevy_sprite3d::Sprite3d;
use bevy_sprite3d::Sprite3dParams;

use crate::entities::collider::Collider;
use crate::entities::player::components::Player;
use crate::MyAssets;

use super::components::*;
use super::resources::*;

use crate::entities::unit::*;

pub fn spawn_enemy(
    mut commands: Commands,
    assets: Res<MyAssets>,
    mut sprite_params: Sprite3dParams,
) {
    commands.spawn((
        Enemy,
        Health {
            current: 100,
            max: 125,
        },
        Speed(3.),
        Unit {
            size: Vec2::new(0.5, 0.5),
        },
        MeleeRange(2.0),
        Damage(10),
        Cooldown(Timer::from_seconds(0.5, TimerMode::Repeating)),
        Sprite3d {
            image: assets.fish_man.clone(),
            pixels_per_metre: 16.0,
            unlit: true,
            transform: Transform::from_xyz(20., 1., 20.),
            ..Default::default()
        }
        .bundle(&mut sprite_params),
        BarBundle::<Health> {
            width: BarWidth::new(1.),
            offset: BarOffset::new(1.),
            ..default()
        },
    ));
}

pub fn update_enemy(
    mut commands: Commands,
    mut enemies: Query<(Entity, &mut Transform, &Enemy, &Speed, &Unit, &Health)>,
    colliders: Query<(&Transform, &Collider), (Without<Unit>, Without<Camera>)>,
    players: Query<(&mut Transform, &Player), (Without<Enemy>, Without<Camera>, Without<Collider>)>,
    time: Res<Time>,
) {
    let dtime = time.delta_seconds();
    let player = players.single();
    for (entity, mut transform, _, speed, unit, health) in &mut enemies {
        if health.current == 0 {
            commands.entity(entity).remove::<Health>();
            // commands.entity(entity).despawn();
            continue;
        }

        let direction = player.0.translation - transform.translation;
        if direction.length() <= 1.0 {
            continue;
        }
        let direction = direction.normalize_or_zero();
        unit.move_and_slide(&mut transform, direction, speed, &colliders, dtime);
    }
}

pub fn kill_enemies(    mut commands: Commands,mut enemies: Query<(Entity, &Enemy), Without<Health>>) {
    for (entity, _) in &mut enemies {
        commands.entity(entity).despawn();
    }
}

pub fn deal_damage(
    mut enemies: Query<(&mut Transform, &Enemy, &MeleeRange, &Damage, &mut Cooldown)>,
    mut players: Query<(&mut Transform, &Player, &mut Health), Without<Enemy>>,
    time: Res<Time>,
) {
    let mut player = players.single_mut();
    for (transform, _, range, damage, mut cooldown) in &mut enemies {
        if player.0.translation.distance(transform.translation) <= range.0 {
            cooldown.0.tick(time.delta());
            if !cooldown.0.just_finished() {
                continue;
            }
            if player.2.current > damage.0 {
                player.2.current -= damage.0;
            } else {
                player.2.current = 0;
            }
            info!("Player hit")
        }
    }
}
