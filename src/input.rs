use winit::event::MouseButton;

/// Input state for handling mouse interactions
pub struct InputState {
    dragging: bool,
    last_pos: Option<(f32, f32)>,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            dragging: false,
            last_pos: None,
        }
    }

    /// Handle mouse button press/release
    pub fn handle_mouse_button(&mut self, button: MouseButton, pressed: bool) {
        if button == MouseButton::Left {
            self.dragging = pressed;
            if !pressed {
                self.last_pos = None;
            }
        }
    }

    /// Handle cursor movement, returns delta if dragging
    pub fn handle_cursor_move(&mut self, x: f32, y: f32) -> Option<(f32, f32)> {
        if self.dragging {
            if let Some((last_x, last_y)) = self.last_pos {
                let delta = (x - last_x, y - last_y);
                self.last_pos = Some((x, y));
                return Some(delta);
            } else {
                self.last_pos = Some((x, y));
            }
        }
        None
    }
}

impl Default for InputState {
    fn default() -> Self {
        Self::new()
    }
}
