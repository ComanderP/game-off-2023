use bevy::prelude::*;

use super::CHUNK_SIDE;

struct ChunkData
{
    tiles: [[i32; CHUNK_SIDE as usize]; CHUNK_SIDE as usize],
}

#[derive(Resource)]
struct WorldData
{
    chunks: bevy::utils::HashMap<(i32, i32), ChunkData>,
}
