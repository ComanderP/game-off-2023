use bevy::prelude::*;

use crate::GameState;

pub mod components;
mod resources;
mod systems;

use self::resources::*;
use self::systems::*;

pub const CAMERA_OFFSET: Vec3 = Vec3::new(0., 10., 25.);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin
{
    fn build(&self, app: &mut App)
    {
        app.insert_resource(PlayerSettings {
            camera_locked: true,
        });
        app.add_systems(OnEnter(GameState::Spawning), spawn_player);
        app.add_systems(Update, update_player.run_if(in_state(GameState::Ready)));
        app.add_systems(Update, update_slash.run_if(in_state(GameState::Ready)));
        app.add_systems(
            Update,
            update_player_sprite.run_if(in_state(GameState::Ready)),
        );
        //app.add_systems(Startup, spawn_player)
        //    .add_systems(Update, (update_player, level_up));
    }
}
