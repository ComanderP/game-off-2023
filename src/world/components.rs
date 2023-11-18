use bevy::prelude::*;

#[derive(Component, Clone, Copy, PartialEq)]
pub enum TileType
{
    Grass,
    Water,
}
