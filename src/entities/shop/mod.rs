use bevy::prelude::*;

use crate::GameState;

pub mod components;
mod resources;
mod systems;

use self::resources::*;
use self::systems::*;

pub struct ShopPlugin;

impl Plugin for ShopPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_systems(OnEnter(GameState::Spawning), spawn_merchant);
        app.add_systems(Update, update_merchant.run_if(in_state(GameState::Ready)));
    }
}
