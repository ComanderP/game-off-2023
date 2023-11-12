use crate::*;

use super::collider::*;
use super::unit::*;
use bevy::prelude::*;
use bevy_sprite3d::Sprite3d;
use bevy_sprite3d::Sprite3dParams;
use rand::Rng;
pub struct ShopPlugin;

impl Plugin for ShopPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Spawning), spawn_merchant);
        app.add_systems(Update, update_merchant.run_if(in_state(GameState::Ready)));
    }
}
#[derive(Component)]
pub struct Merchant;

#[derive(Component)]
pub struct Cart;

pub fn spawn_merchant(
    mut commands: Commands,
    assets: Res<MyAssets>,
    mut sprite_params: Sprite3dParams,
) {
    let mut rng = rand::thread_rng();
    let cx = rng.gen::<i32>() % 10 - 5;
    let cy = rng.gen::<i32>() % 10 - 5;

    commands.spawn((
        Merchant,
        Health {
            current: 100,
            max: 125,
        },
        Speed(3.5),
        Unit {
            size: Vec2::new(0.5, 0.5),
        },
        Sprite3d {
            // atlas sheets crash for some reason ??
            image: assets.player.clone(),
            pixels_per_metre: 16.0,
            unlit: true,
            transform: Transform::from_translation(Vec3::new(cx as f32, 1., cy as f32)),
            ..default()
        }
        .bundle(&mut sprite_params),
        BarBundle::<Health> {
            width: BarWidth::new(1.),
            offset: BarOffset::new(2.),
            ..default()
        },
    ));

    commands.spawn((
        Cart,
        Collider {
            size: Vec2::new(3.8, 0.8),
            active: true,
        },
        Sprite3d {
            image: assets.cart.clone(),
            pixels_per_metre: 16.0,
            unlit: true,
            transform: Transform::from_translation(Vec3::new(
                cx as f32 + 2.0,
                1.2,
                cy as f32 - 2.0,
            )),
            ..default()
        }
        .bundle(&mut sprite_params),
    ));
}

pub fn update_merchant(
    mut merchants: Query<(&mut Transform, &Merchant, &Speed, &Unit), Without<Camera>>,
    colliders: Query<(&Transform, &Collider), (Without<Unit>, Without<Camera>)>,
    time: Res<Time>,
) {
    let dtime = time.delta_seconds();
    for (mut transform, _, speed, unit) in &mut merchants {
        let mut rng = rand::thread_rng();
        // let direction = Vec3::new(rng.gen::<i32>() as f32, 0., rng.gen::<i32>() as f32);
        // let direction = direction.normalize_or_zero();

        let direction = Vec3::ZERO;
        unit.move_and_slide(&mut transform, direction, speed, &colliders, dtime);
    }
}
