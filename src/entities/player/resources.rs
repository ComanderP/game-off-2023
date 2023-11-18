use bevy::prelude::*;

#[derive(Resource)]
pub struct PlayerSettings
{
    pub camera_locked: bool,
}
