use bevy::prelude::*;

use crate::GameState;

pub mod components;
mod resources;
mod systems;

use self::resources::*;
use self::systems::*;
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_systems(OnEnter(GameState::Spawning), spawn_enemy)
            .add_systems(
                Update,
                (
                    update_enemy.run_if(in_state(GameState::Ready)),
                    deal_damage.run_if(in_state(GameState::Ready)),
                    kill_enemies.run_if(in_state(GameState::Ready)),
                ),
            );
    }
}
