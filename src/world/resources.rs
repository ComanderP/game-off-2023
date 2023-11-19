use bevy::{prelude::*, utils::HashMap};

use super::{components::TileType, CHUNK_SIDE};

#[derive(Resource)]
pub struct WorldData
{
    pub chunks: HashMap<(i32, i32), Chunk>,
}

#[derive(Debug)]
pub struct Chunk
{
    pub tiles: [[TileType; CHUNK_SIDE as usize]; CHUNK_SIDE as usize],
    pub is_loaded: bool,
}

impl Chunk
{
    pub fn is_loaded(&self) -> bool
    {
        self.is_loaded
    }

    pub fn set_is_loaded(&mut self, is_loaded: bool)
    {
        self.is_loaded = is_loaded;
    }
}

impl Default for Chunk
{
    fn default() -> Self
    {
        let mut tiles = [[TileType::Grass; CHUNK_SIDE as usize]; CHUNK_SIDE as usize];
        for i in 0..CHUNK_SIDE
        {
            for j in 0..CHUNK_SIDE
            {
                if i == 0 || j == 0 || i == CHUNK_SIDE - 1 || j == CHUNK_SIDE - 1
                {
                    tiles[i as usize][j as usize] = TileType::Water;
                }
            }
        }
        Self {
            tiles,
            is_loaded: false,
        }
    }
}
