use bevy::{prelude::*, utils::HashMap};
use noise::utils::NoiseMap;

use crate::world::MAP_SIDE;

use super::{components::TileType, CHUNK_SIDE};

#[derive(Resource)]
pub struct WorldData
{
    pub chunks: HashMap<(i32, i32), Chunk>,
    pub noise_map: NoiseMap,
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

pub trait Generate where {
    fn generate(map: &NoiseMap, translate: Vec2) -> Self;
}

impl Generate for Chunk
{
    fn generate(map: &NoiseMap, translate: Vec2) -> Self {
        let mut tiles = [[TileType::Grass; CHUNK_SIDE as usize]; CHUNK_SIDE as usize];
        for i in 0..CHUNK_SIDE
        {
            for j in 0..CHUNK_SIDE
            {
                // random gen
                let value = map.get_value((translate.x * CHUNK_SIDE as f32 + i as f32) as usize % MAP_SIDE,( translate.y  * CHUNK_SIDE as f32 + j as f32 + 500.) as usize % MAP_SIDE);
                if value > 0.0
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
