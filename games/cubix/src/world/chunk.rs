use super::{BlockKind, ChunkCoord, LocalBlockCoord};

pub const CHUNK_WIDTH: usize = 16;
pub const CHUNK_HEIGHT: usize = 128;
pub const CHUNK_DEPTH: usize = 16;
pub const CHUNK_VOLUME: usize = CHUNK_WIDTH * CHUNK_HEIGHT * CHUNK_DEPTH;

#[derive(Debug, Clone)]
pub struct Chunk {
    pub coord: ChunkCoord,
    blocks: Vec<BlockKind>,
    pub dirty_mesh: bool,
}

impl Chunk {
    pub fn new(coord: ChunkCoord) -> Self {
        Self {
            coord,
            blocks: vec![BlockKind::Air; CHUNK_VOLUME],
            dirty_mesh: true,
        }
    }

    pub fn blocks(&self) -> &[BlockKind] {
        &self.blocks
    }

    pub fn get(&self, coord: LocalBlockCoord) -> BlockKind {
        self.blocks[Self::index(coord)]
    }

    pub fn set(&mut self, coord: LocalBlockCoord, block: BlockKind) {
        let index = Self::index(coord);
        if self.blocks[index] != block {
            self.blocks[index] = block;
            self.dirty_mesh = true;
        }
    }

    pub fn mark_mesh_clean(&mut self) {
        self.dirty_mesh = false;
    }

    fn index(coord: LocalBlockCoord) -> usize {
        let (x, y, z) = coord.as_usize();
        (y * CHUNK_DEPTH * CHUNK_WIDTH) + (z * CHUNK_WIDTH) + x
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Self::new(ChunkCoord::ORIGIN)
    }
}
