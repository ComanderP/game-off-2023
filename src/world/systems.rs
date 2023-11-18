use crate::assets::MyAssets;
use crate::entities::collider::Collider;
use crate::entities::player::components::Player;
use std::f32::consts::FRAC_PI_2;

use super::components::*;
use super::resources::*;
use super::CHUNK_RADIUS;

use bevy::prelude::*;
use bevy_sprite3d::*;
use rand::distributions::{Bernoulli, Distribution};
use rand::rngs::ThreadRng;

#[derive(Bundle)]
struct TileBundle
{
    tile_type: TileType,
    sprite: Sprite3dBundle,
}

#[derive(Bundle)]
struct SolidObjectBundle
{
    collider: Collider,
    sprite: Sprite3dBundle,
}

/// Spawn the tiles for the world
/// TODO: Make this spawn chunks instead of all tiles
pub fn spawn_tiles(commands: Commands, assets: Res<MyAssets>, sprite_params: Sprite3dParams)
{
    let mut rng = rand::thread_rng();

    let mut common = (commands, assets, sprite_params);

    for i in -CHUNK_RADIUS..=CHUNK_RADIUS
    {
        for j in -CHUNK_RADIUS..=CHUNK_RADIUS
        {
            // Spawn a grass tile
            if probability(0.9, &mut rng)
            {
                // Spawn a rock
                if probability(0.04, &mut rng)
                {
                    spawn_rock(i, j, &mut common);
                }
                spawn_tile(i, j, TileType::Grass, &mut common);
            }
            // Spawn a water tile
            else
            {
                spawn_tile(i, j, TileType::Water, &mut common);
            }
        }
    }
}

/// Spawns a rock at the given position.
fn spawn_rock(i: i32, j: i32, common: &mut (Commands, Res<MyAssets>, Sprite3dParams))
{
    common.0.spawn(SolidObjectBundle {
        collider: Collider {
            size: Vec2::new(1.8, 0.5),
            active: true,
        },
        sprite: Sprite3d {
            image: common.1.rock.clone(),
            pixels_per_metre: 16.0,
            unlit: true,
            transform: Transform::from_xyz((i as f32) * 2., 1., (j as f32) * 2. + 1.),
            ..default()
        }
        .bundle(&mut common.2),
    });
}

/// Spawns a tile of the given type at the given position.
fn spawn_tile(
    i: i32,
    j: i32,
    tile_type: TileType,
    common: &mut (Commands, Res<MyAssets>, Sprite3dParams),
)
{
    // Spawn the tile
    let mut entity = common.0.spawn(TileBundle {
        tile_type,
        sprite: Sprite3d {
            image: get_tile_image(&common.1, tile_type),
            pixels_per_metre: 16.0,
            unlit: true,
            transform: Transform::from_xyz((i as f32) * 2., 0.001, (j as f32) * 2.)
                .with_rotation(Quat::from_rotation_x(-FRAC_PI_2)),
            ..default()
        }
        .bundle(&mut common.2),
    });
    // Add a collider to the tile if it is water
    match tile_type
    {
        TileType::Water =>
        {
            entity.insert(Collider {
                size: Vec2::new(2., 2.),
                active: true,
            });
        }
        _ => (),
    };
}

/// Returns the image for the given tile type.
fn get_tile_image(assets: &MyAssets, tile_type: TileType) -> Handle<Image>
{
    match tile_type
    {
        TileType::Grass => assets.grass.clone(),
        TileType::Water => assets.water.clone(),
    }
}

/// Returns true with a probability of `odds`.
/// # Example
/// ```no_run
/// let mut rng = rand::thread_rng();
/// // 50% chance of returning true
/// let result = probability(0.5, &mut rng);
/// ```
fn probability(odds: f64, rng: &mut ThreadRng) -> bool
{
    Bernoulli::new(odds).unwrap().sample(rng)
}

pub fn spawn_tiles_around_player(
    commands: Commands,
    assets: Res<MyAssets>,
    sprite_params: Sprite3dParams,
    player: Query<&Transform, With<Player>>,
)
{
    let player_pos = player.single().translation;
    let player_pos = Vec2::new(player_pos.x, player_pos.z);

    // info!("Player position: {:?}", player_pos);
}

pub fn update_tiles(commands: Commands) {}
