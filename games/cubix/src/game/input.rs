use cgmath::Vector2;

#[derive(Debug, Clone, PartialEq)]
pub struct InputState {
    pub move_forward: bool,
    pub move_backward: bool,
    pub move_left: bool,
    pub move_right: bool,
    pub jump_pressed: bool,
    pub sprint_pressed: bool,
    pub place_pressed: bool,
    pub destroy_pressed: bool,
    pub look_delta: Vector2<f32>,
}

impl Default for InputState {
    fn default() -> Self {
        Self {
            move_forward: false,
            move_backward: false,
            move_left: false,
            move_right: false,
            jump_pressed: false,
            sprint_pressed: false,
            place_pressed: false,
            destroy_pressed: false,
            look_delta: Vector2::new(0.0, 0.0),
        }
    }
}

impl InputState {
    pub fn clear_frame_state(&mut self) {
        self.look_delta = Vector2::new(0.0, 0.0);
        self.place_pressed = false;
        self.destroy_pressed = false;
    }
}
