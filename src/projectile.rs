use raylib::prelude::*;

pub struct Projectile {
    pub pos: Vector2,
    pub vel: Vector2,
    pub color: Color,
    pub active: bool,  
    pub owner_id: i32, 
}

impl Projectile {
    // constructor for the magic bolts
    pub fn new(pos: Vector2, vel: Vector2, owner_id: i32, color: Color) -> Self {
        Self {
            pos,
            vel,
            color,
            active: true,
            owner_id,
        }
    }

    // moves the bolt and kills it if it flies off-screen
    pub fn update(&mut self, dt: f32) {
        self.pos += self.vel * dt;

        // generous bounds so they don't despawn too early during the tower climb
        if self.pos.x < -500.0 || self.pos.x > 2500.0 || self.pos.y < -10000.0 || self.pos.y > 2000.0 {
            self.active = false;
        }
    }

    // fallback draw call if we aren't using the fancy glow in game_scene
    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_circle_v(self.pos, 10.0, self.color);
    }
}