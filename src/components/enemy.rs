use super::collider::*;
use super::player::Player;
use super::unit::*;
use crate::GameState;
use crate::MyAssets;
use bevy::prelude::*;
use bevy_health_bar3d::prelude::*;
use bevy_sprite3d::Sprite3d;
use bevy_sprite3d::Sprite3dParams;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Spawning), spawn_enemy)
            .add_systems(
                Update,
                (
                    update_enemy.run_if(in_state(GameState::Ready)),
                    deal_damage.run_if(in_state(GameState::Ready)),
                ),
            );
    }
}
#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct MeleeRange(pub f32);

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
        Sprite3d {
            image: assets.player.clone(),
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
    mut enemies: Query<(&mut Transform, &Enemy, &Speed, &Unit), Without<Camera>>,
    colliders: Query<(&Transform, &Collider), (Without<Unit>, Without<Camera>)>,
    players: Query<(&mut Transform, &Player), (Without<Enemy>, Without<Camera>, Without<Collider>)>,
    time: Res<Time>,
) {
    let dtime = time.delta_seconds();
    let player = players.single();
    for (mut transform, _, speed, unit) in &mut enemies {
        let direction = player.0.translation - transform.translation;
        if direction.length() <= 1.0 {
            continue;
        }
        let direction = direction.normalize_or_zero();
        unit.move_and_slide(&mut transform, direction, speed, &colliders, dtime);
   
    }
}

fn deal_damage(
    mut enemies: Query<(&mut Transform, &Enemy, &MeleeRange)>,
    mut players: Query<(&mut Transform, &Player, &mut Health), Without<Enemy>>,
) {
    let mut player = players.single_mut();
    for (transform, _, range) in &mut enemies {
        if player.0.translation.distance(transform.translation) <= range.0 {
            if player.2.current > 10 {
                player.2.current -= 10;
            } else {
                player.2.current = 0;
            }
            // info!("Player hit")
        }
    }
}
