#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum BlockKind {
    #[default]
    Air,
    Grass,
    Dirt,
    Stone,
    Water,
}

impl BlockKind {
    pub fn is_solid(self) -> bool {
        !matches!(self, Self::Air | Self::Water)
    }

    pub fn is_transparent(self) -> bool {
        matches!(self, Self::Air | Self::Water)
    }

    pub fn is_replaceable(self) -> bool {
        matches!(self, Self::Air | Self::Water)
    }
}
