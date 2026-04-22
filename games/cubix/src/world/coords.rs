use super::chunk::{CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_WIDTH};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ChunkCoord {
    pub x: i32,
    pub z: i32,
}

impl ChunkCoord {
    pub const ORIGIN: Self = Self { x: 0, z: 0 };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct BlockCoord {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LocalBlockCoord {
    pub x: u8,
    pub y: u16,
    pub z: u8,
}

impl LocalBlockCoord {
    pub fn new(x: usize, y: usize, z: usize) -> Option<Self> {
        if x < CHUNK_WIDTH && y < CHUNK_HEIGHT && z < CHUNK_DEPTH {
            Some(Self {
                x: x as u8,
                y: y as u16,
                z: z as u8,
            })
        } else {
            None
        }
    }

    pub fn as_usize(self) -> (usize, usize, usize) {
        (self.x as usize, self.y as usize, self.z as usize)
    }
}
