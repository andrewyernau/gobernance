pub mod entity;
pub mod input;
pub mod physics;
pub mod player;
pub mod state;

pub use entity::{Aabb, Entity, EntityId, EntityKind, Transform, Velocity};
pub use input::InputState;
pub use physics::{MovementIntent, build_movement_intent, integrate_player};
pub use player::Player;
pub use state::GameState;
