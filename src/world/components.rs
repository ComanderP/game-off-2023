use bevy::prelude::*;

#[derive(Component, Clone, Copy, PartialEq, Debug)]
pub enum TileType
{
    Grass,
    Water,
}
