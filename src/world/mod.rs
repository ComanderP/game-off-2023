use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::GameState;

pub mod components;
mod resources;
mod systems;

use self::resources::*;
use self::systems::*;

pub const CHUNK_RADIUS: i32 = 2;
pub const CHUNK_SIDE: i32 = CHUNK_RADIUS * 2 + 1;

pub struct WorldPlugin;

impl Plugin for WorldPlugin
{
    fn build(&self, app: &mut App)
    {
        app.insert_resource(WorldData {
            chunks: HashMap::default(),
        })
        .add_systems(OnEnter(GameState::Ready), spawn_tiles_around_player)
        .add_systems(Update, update_tiles.run_if(in_state(GameState::Ready)))
        .add_systems(Update, deload_chunks.run_if(in_state(GameState::Ready)));
    }
}
