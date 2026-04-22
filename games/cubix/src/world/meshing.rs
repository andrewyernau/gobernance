use super::{BlockKind, Chunk};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChunkMeshVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChunkMesh {
    pub vertices: Vec<ChunkMeshVertex>,
    pub indices: Vec<u32>,
}

#[derive(Debug, Default)]
pub struct ChunkMesher;

impl ChunkMesher {
    pub fn build_visible_faces(&self, chunk: &Chunk) -> ChunkMesh {
        let solid_block_count = chunk
            .blocks()
            .iter()
            .filter(|block| **block != BlockKind::Air)
            .count();

        ChunkMesh {
            vertices: Vec::with_capacity(solid_block_count * 24),
            indices: Vec::with_capacity(solid_block_count * 36),
        }
    }
}
