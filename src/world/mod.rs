use std::cmp::min;

use crate::GameState;
use bevy::prelude::*;
use bevy::utils::HashMap;
use noise::core::perlin::perlin_2d;
use noise::permutationtable::PermutationTable;
use noise::utils::{NoiseMapBuilder, PlaneMapBuilder};
use noise::{Abs, Curve, Cylinders, Fbm, NoiseFn, Perlin, ScalePoint};
use rand::RngCore;

pub mod components;
pub mod resources;
pub mod systems;

use self::resources::*;
use self::systems::*;

pub const CHUNK_RADIUS: i32 = 2;
pub const CHUNK_SIDE: i32 = CHUNK_RADIUS * 2 + 1;
pub const MAP_SIDE: usize = 1000;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        let mut rng = rand::thread_rng();
        let perlin = Perlin::new(rng.next_u32());
        let hasher = PermutationTable::new(0);

        app.insert_resource(WorldData {
            chunks: HashMap::default(),
            // make it so circle around player, yes
            noise_map: PlaneMapBuilder::<_, 2>::new_fn(
                |point, _hasher| {
                    (-((point[0] + 50.).abs().min((point[0] - 50.).abs()).powi(2)
                        + (point[1] + 50.).abs().min((point[1] - 50.).abs()).powi(2)
                        - 1.))
                        .max(perlin.get(point))
                },
                &hasher,
            )
            .set_is_seamless(false)
            .set_size(MAP_SIDE, MAP_SIDE)
            .set_x_bounds(-50.0, 50.0)
            .set_y_bounds(-50.0, 50.0)
            .build(),
        })
        .add_systems(OnEnter(GameState::Ready), spawn_tiles_around_player)
        .add_systems(Update, update_tiles.run_if(in_state(GameState::Ready)))
        .add_systems(Update, deload_chunks.run_if(in_state(GameState::Ready)));
    }
}
