use crate::world::World;

use super::{InputState, Player};

#[derive(Debug, Default)]
pub struct GameState {
    pub world: World,
    pub player: Player,
    pub input: InputState,
    pub tick: u64,
}

impl GameState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn advance_tick(&mut self) {
        self.tick += 1;
        self.input.clear_frame_state();
    }
}
