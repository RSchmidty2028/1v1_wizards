use raylib::prelude::*;

#[derive(Clone)]
pub struct Player {
    pub pos: Vector2,
    pub vel: Vector2,
    pub grounded: bool,
    pub input_id: i32,
    pub aim: Vector2,
    pub shoot_timer: f32, 
    pub facing_left: bool
}

impl Player {
    pub fn new(id: i32, x: f32, y: f32) -> Self {
        Self {
            pos: Vector2::new(x, y),
            vel: Vector2::zero(),
            grounded: false,
            input_id: id,
            aim: Vector2::new(1.0, 0.0),
            shoot_timer: 0.0,
            facing_left: false, //able to shoot first frame i think
        }
    }

    pub fn rect(&self) -> Rectangle {
        Rectangle::new(self.pos.x, self.pos.y, 30.0, 30.0)
    }
}