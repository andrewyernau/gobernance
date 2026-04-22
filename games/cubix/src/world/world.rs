use std::collections::HashMap;

use super::{BlockKind, Chunk, ChunkCoord, LocalBlockCoord};

#[derive(Debug, Default)]
pub struct World {
    chunks: HashMap<ChunkCoord, Chunk>,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn chunk(&self, coord: ChunkCoord) -> Option<&Chunk> {
        self.chunks.get(&coord)
    }

    pub fn chunk_mut(&mut self, coord: ChunkCoord) -> Option<&mut Chunk> {
        self.chunks.get_mut(&coord)
    }

    pub fn ensure_chunk(&mut self, coord: ChunkCoord) -> &mut Chunk {
        self.chunks
            .entry(coord)
            .or_insert_with(|| Chunk::new(coord))
    }

    pub fn insert_chunk(&mut self, chunk: Chunk) {
        self.chunks.insert(chunk.coord, chunk);
    }

    pub fn set_block_in_chunk(
        &mut self,
        chunk_coord: ChunkCoord,
        local_coord: LocalBlockCoord,
        block: BlockKind,
    ) {
        self.ensure_chunk(chunk_coord).set(local_coord, block);
    }

    pub fn loaded_chunks(&self) -> impl Iterator<Item = &Chunk> {
        self.chunks.values()
    }
}
