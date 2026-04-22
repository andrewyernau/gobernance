use cgmath::Vector3;

use super::{InputState, Player};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MovementIntent {
    pub direction: Vector3<f32>,
    pub jump: bool,
    pub sprint: bool,
}

impl Default for MovementIntent {
    fn default() -> Self {
        Self {
            direction: Vector3::new(0.0, 0.0, 0.0),
            jump: false,
            sprint: false,
        }
    }
}

pub fn build_movement_intent(input: &InputState) -> MovementIntent {
    let mut direction = Vector3::new(0.0, 0.0, 0.0);

    if input.move_forward {
        direction.z -= 1.0;
    }
    if input.move_backward {
        direction.z += 1.0;
    }
    if input.move_left {
        direction.x -= 1.0;
    }
    if input.move_right {
        direction.x += 1.0;
    }

    MovementIntent {
        direction,
        jump: input.jump_pressed,
        sprint: input.sprint_pressed,
    }
}

pub fn integrate_player(player: &mut Player, delta_seconds: f32) {
    player.transform.position += player.velocity.value * delta_seconds;
}
