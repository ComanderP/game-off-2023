use bevy::prelude::*;

use crate::GameState;

pub mod components;
mod resources;
mod systems;

use self::resources::*;
use self::systems::*;

pub struct UIPlugin;

impl Plugin for UIPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_systems(OnEnter(GameState::Spawning), load_ui)
            .add_systems(Update, update_ui.run_if(in_state(GameState::Ready)));
    }
}