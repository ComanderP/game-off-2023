use bevy::prelude::*;
use bevy_sprite3d::*;
use rand::distributions::{Bernoulli, Distribution};
use rand::rngs::ThreadRng;

use crate::assets::MyAssets;
use crate::entities::collider::Collider;
use crate::entities::player::components::Player;
use std::f32::consts::FRAC_PI_2;

use super::{components::*, resources::*, CHUNK_RADIUS, CHUNK_SIDE};

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
pub fn spawn_tiles(mut commands: Commands, assets: Res<MyAssets>, mut sprite_params: Sprite3dParams)
{
    let mut rng = rand::thread_rng();
    let mut common = (&mut commands, assets.as_ref(), &mut sprite_params);

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
fn spawn_rock(i: i32, j: i32, common: &mut (&mut Commands, &MyAssets, &mut Sprite3dParams))
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
    common: &mut (&mut Commands, &MyAssets, &mut Sprite3dParams),
)
{
    // Spawn the tile
    let mut entity = common.0.spawn(TileBundle {
        tile_type,
        sprite: Sprite3d {
            image: get_tile_image(&common.1, tile_type),
            pixels_per_metre: 16.0,
            unlit: true,
            transform: Transform::from_xyz((i as f32) * 2., 0., (j as f32) * 2.)
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
            // entity.insert(Collider {
            //     size: Vec2::new(2., 2.),
            //     active: true,
            // });
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
    mut commands: Commands,
    assets: Res<MyAssets>,
    mut sprite3d_params: Sprite3dParams,
    player: Query<&Transform, With<Player>>,
    mut world_data: ResMut<WorldData>,
)
{
    // Add a chunk to the world around the player if it doesn't exist
    let player_pos = player.single().translation;
    let player_pos = Vec2::new(player_pos.x, player_pos.z);

    let chunk_coords = get_chunk_pos(player_pos);

    for i in -1..=1
    {
        for j in -1..=1
        {
            add_chunk(&mut world_data, chunk_coords, i, j);

            let (a, b) = (chunk_coords.0 + i, chunk_coords.1 + j);

            let chunk = world_data.chunks.get_mut(&(a, b));

            match chunk
            {
                Some(chunk) =>
                {
                    spawn_chunk(
                        &mut commands,
                        &assets,
                        &mut sprite3d_params,
                        chunk,
                        (a * CHUNK_SIDE) as i32,
                        (b * CHUNK_SIDE) as i32,
                    );
                    chunk.set_is_loaded(true);
                }
                None => (),
            }
        }
    }
}

fn add_chunk(world_data: &mut ResMut<'_, WorldData>, chunk_coords: (i32, i32), i: i32, j: i32)
{
    let chunk = world_data
        .chunks
        .get(&(chunk_coords.0 + i, chunk_coords.1 + j));

    match chunk
    {
        Some(_) => (),
        None =>
        {
            world_data
                .chunks
                .insert((chunk_coords.0 + i, chunk_coords.1 + j), Chunk::default());
        }
    }
}

fn get_chunk_pos(player_pos: Vec2) -> (i32, i32)
{
    let mut x = player_pos.x as i32;
    let mut y = player_pos.y as i32;

    if x > 0
    {
        x = (x + CHUNK_SIDE) / (CHUNK_SIDE * 2);
    }
    else
    {
        x = (x - CHUNK_SIDE) / (CHUNK_SIDE * 2);
    }
    if y > 0
    {
        y = (y + CHUNK_SIDE) / (CHUNK_SIDE * 2);
    }
    else
    {
        y = (y - CHUNK_SIDE) / (CHUNK_SIDE * 2);
    }

    let chunk_coords = (x, y);
    chunk_coords
}

/// Spawns a chunk at the given position.
/// The chunk is spawned at the given position, with the center of the chunk at the given position.
fn spawn_chunk(
    commands: &mut Commands,
    assets: &MyAssets,
    sprite3d_params: &mut Sprite3dParams,
    chunk: &Chunk,
    i: i32,
    j: i32,
)
{
    let start_x = i - CHUNK_SIDE / 2;
    let start_y = j - CHUNK_SIDE / 2;

    let mut bundle: Vec<TileBundle> = vec![];

    for (x, row) in chunk.tiles.iter().enumerate()
    {
        for (y, tile) in row.iter().enumerate()
        {
            bundle.push(TileBundle::new(
                *tile,
                start_x + x as i32,
                start_y + y as i32,
                assets,
                sprite3d_params,
            ));
        }
    }

    commands.spawn_batch(bundle);
}

impl TileBundle
{
    fn new(
        tile_type: TileType,
        i: i32,
        j: i32,
        assets: &MyAssets,
        mut sprite3d_params: &mut Sprite3dParams,
    ) -> Self
    {
        let t = TileBundle {
            tile_type,
            sprite: Sprite3d {
                image: get_tile_image(&assets, tile_type),
                pixels_per_metre: 16.0,
                unlit: true,
                transform: Transform::from_xyz((i as f32) * 2., 0., (j as f32) * 2.)
                    .with_rotation(Quat::from_rotation_x(-FRAC_PI_2)),
                ..default()
            }
            .bundle(&mut sprite3d_params),
        };
        // Add a collider to the tile if it is water
        match tile_type
        {
            TileType::Water =>
            {
                // entity.insert(Collider {
                //     size: Vec2::new(2., 2.),
                //     active: true,
                // });
            }
            _ => (),
        };
        return t;
    }
}

/// Updates the tiles in the based on the player's position.
/// Chunks are added and removed as needed.
/// If a chunk is too far away from the player, it is removed.
pub fn update_tiles(
    mut commands: Commands,
    assets: Res<MyAssets>,
    mut sprite3d_params: Sprite3dParams,
    player: Query<&Transform, With<Player>>,
    mut world_data: ResMut<WorldData>,
)
{
    // Add a chunk to the world around the player if it doesn't exist
    let player_pos = player.single().translation;
    let player_pos = Vec2::new(player_pos.x, player_pos.z);

    let chunk_coords = get_chunk_pos(player_pos);

    for i in -1..=1
    {
        for j in -1..=1
        {
            // Add new chunks to the world
            add_chunk(&mut world_data, chunk_coords, i, j);
            // Spawn the chunk if it not already loaded

            let (a, b) = (chunk_coords.0 + i, chunk_coords.1 + j);

            let chunk = world_data.chunks.get_mut(&(a, b));

            match chunk
            {
                Some(chunk) =>
                {
                    if !chunk.is_loaded()
                    {
                        load_chunk(&mut commands, &assets, &mut sprite3d_params, chunk, a, b);
                    }
                }
                None => (),
            }
        }
    }
}

pub fn deload_chunks(
    mut commands: Commands,
    player: Query<&Transform, With<Player>>,
    mut world_data: ResMut<WorldData>,
    mut entities: Query<(Entity, &Transform), With<TileType>>,
)
{
    let player_pos = player.single().translation;
    let player_pos = Vec2::new(player_pos.x, player_pos.z);

    let chunk_coords = get_chunk_pos(player_pos);

    // Remove chunks that are too far away from the player

    for (chunk_pos, chunk) in world_data.chunks.iter_mut()
    {
        if ((chunk_pos.0 - chunk_coords.0).abs() > 2 || (chunk_pos.1 - chunk_coords.1).abs() > 2 ) && chunk.is_loaded() {
            // Remove the chunk
            let tiles = get_chunk_tiles(*chunk_pos, &mut entities);
            for tile in tiles
            {

                commands.entity(tile).despawn();
            }
            chunk.set_is_loaded(false);
        }
    }
}

fn load_chunk(
    commands: &mut Commands<'_, '_>,
    assets: &Res<'_, MyAssets>,
    sprite3d_params: &mut Sprite3dParams<'_, '_>,
    chunk: &mut Chunk,
    a: i32,
    b: i32,
)
{
    spawn_chunk(
        commands,
        assets,
        sprite3d_params,
        chunk,
        (a * CHUNK_SIDE) as i32,
        (b * CHUNK_SIDE) as i32,
    );
    chunk.set_is_loaded(true);
}

fn get_chunk_tiles(
    chunk_pos: (i32, i32),
    mut entities: &mut Query<(Entity, &Transform), With<TileType>>,
) -> Vec<Entity>
{
    let mut tiles: Vec<Entity> = vec![];

    for (entity, transform) in entities.iter_mut()
    {
        let pos = Vec2::new(transform.translation.x, transform.translation.z);
        let chunk = get_chunk_pos(pos);

        if chunk == chunk_pos
        {
            tiles.push(entity);
        }
    }

    tiles
}
