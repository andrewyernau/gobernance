use cgmath::Point3;

use crate::world::BlockKind;

use super::{Aabb, Transform, Velocity};

#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    pub transform: Transform,
    pub velocity: Velocity,
    pub collider: Aabb,
    pub selected_block: BlockKind,
    pub selected_hotbar_slot: u8,
    pub reach: f32,
    pub on_ground: bool,
}

impl Player {
    pub fn from_spawn(position: Point3<f32>) -> Self {
        Self {
            transform: Transform {
                position,
                ..Transform::default()
            },
            ..Self::default()
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            transform: Transform::default(),
            velocity: Velocity::default(),
            collider: Aabb {
                half_extents: cgmath::Vector3::new(0.3, 0.9, 0.3),
            },
            selected_block: BlockKind::Grass,
            selected_hotbar_slot: 0,
            reach: 5.0,
            on_ground: false,
        }
    }
}
