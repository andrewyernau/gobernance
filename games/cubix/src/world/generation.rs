use super::{
    BlockKind, CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_WIDTH, Chunk, ChunkCoord, LocalBlockCoord,
};

pub trait ChunkGenerator {
    fn generate_chunk(&self, coord: ChunkCoord) -> Chunk;
}

#[derive(Debug, Clone, Copy)]
pub struct FlatWorldGenerator {
    pub surface_height: usize,
}

impl FlatWorldGenerator {
    pub fn new(surface_height: usize) -> Self {
        Self { surface_height }
    }
}

impl Default for FlatWorldGenerator {
    fn default() -> Self {
        Self { surface_height: 32 }
    }
}

impl ChunkGenerator for FlatWorldGenerator {
    fn generate_chunk(&self, coord: ChunkCoord) -> Chunk {
        let mut chunk = Chunk::new(coord);
        let top = self.surface_height.min(CHUNK_HEIGHT.saturating_sub(1));

        for z in 0..CHUNK_DEPTH {
            for x in 0..CHUNK_WIDTH {
                for y in 0..=top {
                    let local = LocalBlockCoord::new(x, y, z).expect("valid flat chunk coords");
                    let block = if y == 0 {
                        BlockKind::Stone
                    } else if y == top {
                        BlockKind::Grass
                    } else {
                        BlockKind::Dirt
                    };
                    chunk.set(local, block);
                }
            }
        }

        chunk
    }
}
