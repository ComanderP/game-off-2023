use bevy::prelude::*;

use crate::GameState;

pub mod components;
mod resources;
mod systems;

use self::resources::*;
use self::systems::*;

pub const CHUNK_RADIUS: i32 = 32;
pub const CHUNK_SIDE: i32 = CHUNK_RADIUS * 2 + 1;

pub struct WorldPlugin;

impl Plugin for WorldPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_systems(OnEnter(GameState::Ready), spawn_tiles)
            .add_systems(Update, update_tiles);
    }
}
