use std::f32::consts::FRAC_PI_2;

use crate::*;

use super::collider::*;
use bevy::{prelude::{shape::Quad, *}, ecs::system::EntityCommands};
use bevy_sprite3d::*;
use rand::Rng;

pub struct WorldPlugin;

#[derive(Component)]
pub enum TileType {
    Grass,
    Water,
}

const CHUNK_RADIUS: i32 = 32;
const CHUNK_SIDE: i32 = CHUNK_RADIUS * 2 + 1;

struct ChunkData {
    tiles : [[i32; CHUNK_SIDE as usize]; CHUNK_SIDE as usize],
}

#[derive (Resource)]
struct WorldData {
    chunks : bevy::utils::HashMap<(i32, i32), ChunkData>,
}

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Ready), spawn_tiles)
            .add_systems(Update, update_tiles);
    }
}

pub fn spawn_tiles(
    mut commands: Commands,
    assets: Res<MyAssets>,
    mut sprite_params: Sprite3dParams,
) {
    for i in -CHUNK_RADIUS..=CHUNK_RADIUS {
        for j in -CHUNK_RADIUS..=CHUNK_RADIUS {
            let mut rng = rand::thread_rng();

            if rng.gen::<i32>() % 10 != 0 {
                let mut path = "grass_var1.png";
                if rng.gen::<i32>() % 20 == 0 {
                    path = "grass_var2.png";
                }

                if rng.gen::<i32>() % 25 == 0 {
                    commands.spawn((
                        Collider {
                            size: Vec2::new(1.8, 0.5),
                            active: true,
                        },
                        Sprite3d {
                            image: assets.rock.clone(),
                            pixels_per_metre: 16.0,
                            unlit: true,
                            transform: Transform::from_xyz(
                                (i as f32) * 2.,
                                1.,
                                (j as f32) * 2. + 1.,
                            ),
                            ..default()
                        }
                        .bundle(&mut sprite_params),
                    ));
                }
                commands.spawn((
                    TileType::Grass,
                    Sprite3d {
                        image: assets.grass.clone(),
                        pixels_per_metre: 16.0,
                        unlit: true,
                        transform: Transform::from_xyz((i as f32) * 2., 0.001, (j as f32) * 2.)
                            .with_rotation(Quat::from_rotation_x(-FRAC_PI_2)),
                        ..default()
                    }
                    .bundle(&mut sprite_params),
                ));
            } else {
                commands.spawn((
                    TileType::Water,
                    Collider {
                        size: Vec2::new(2., 2.),
                        active: true,
                    },
                    Sprite3d {
                        image: assets.water.clone(),
                        pixels_per_metre: 16.0,
                        unlit: true,
                        transform: Transform::from_xyz((i as f32) * 2., 0.001, (j as f32) * 2.)
                            .with_rotation(Quat::from_rotation_x(-FRAC_PI_2)),
                        ..default()
                    }
                    .bundle(&mut sprite_params),
                ));
            }
        }
    }
}

pub fn update_tiles(commands: Commands) {}
