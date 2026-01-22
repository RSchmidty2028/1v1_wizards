use raylib::prelude::*;

pub struct Projectile {
    pub pos: Vector2,
    pub vel: Vector2,
    pub color: Color,
    pub active: bool,  
    pub owner_id: i32, 
}

impl Projectile {
    // spawns a new projectile
    pub fn new(pos: Vector2, vel: Vector2, owner_id: i32, color: Color) -> Self {
        Self {
            pos,
            vel,
            color,
            active: true,
            owner_id,
        }
    }

    // moves bullet
    pub fn update(&mut self, dt: f32) {
        self.pos += self.vel * dt;

        // despawn if too far away
        if self.pos.x < -100.0 || self.pos.x > (4.0 * 576.0) || self.pos.y < -100.0 || self.pos.y > (4.0 * 324.0) {
            self.active = false;
        }
    }

    // placeholder bullet
    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        // use self.color so it matches what we passed in
        d.draw_circle_v(self.pos, 10.0, self.color);
    }
}