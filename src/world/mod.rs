use bevy::prelude::*;
use bevy::utils::HashMap;
use noise::{Fbm, Perlin};
use noise::utils::{NoiseMapBuilder, PlaneMapBuilder};
use crate::GameState;

pub mod components;
mod resources;
mod systems;

use self::resources::*;
use self::systems::*;

pub const CHUNK_RADIUS: i32 = 2;
pub const CHUNK_SIDE: i32 = CHUNK_RADIUS * 2 + 1;
pub const MAP_SIDE: usize = 1000;

pub struct WorldPlugin;

impl Plugin for WorldPlugin
{
    fn build(&self, app: &mut App)
    {
        let fbm = Fbm::<Perlin>::new(0);
        app.insert_resource(WorldData {
            chunks: HashMap::default(),
            noise_map: PlaneMapBuilder::<_, 2>::new(&fbm)
            .set_is_seamless(true)
            .set_size(MAP_SIDE, MAP_SIDE)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build()

        })
        .add_systems(OnEnter(GameState::Ready), spawn_tiles_around_player)
        .add_systems(Update, update_tiles.run_if(in_state(GameState::Ready)))
        .add_systems(Update, deload_chunks.run_if(in_state(GameState::Ready)));
    }
}
