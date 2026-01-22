use raylib::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationState {
    Idle,
    Run,
    Jump,
    Shoot,
    Hurt,
}

#[derive(Clone)]
pub struct Player {
    pub pos: Vector2,
    pub vel: Vector2,
    pub grounded: bool,
    pub input_id: i32,
    pub aim: Vector2,
    pub shoot_timer: f32,
    pub shooting: bool,
    pub facing_left: bool,

    // animation state tracking
    pub anim_state: AnimationState,
    pub anim_frame: usize,
    pub anim_timer: f32,
    pub width: f32,
    pub height: f32,
    pub attack_fired: bool,
    pub hit: bool,

    // health and damage cooldowns
    pub hp: i32,
    pub i_frame_timer: f32,
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
            shooting: false,
            facing_left: false,

            // starting animation values
            anim_state: AnimationState::Idle,
            anim_frame: 0,
            anim_timer: 0.0,
            width: 64.0,
            height: 128.0,
            attack_fired: false,
            hit: false,

            // three hits and you're out
            hp: 3,
            i_frame_timer: 0.0,
        }
    }

    // helper to see if we're still in that post-hit blink
    pub fn is_invincible(&self) -> bool {
        self.i_frame_timer > 0.0
    }

    // custom hitbox dimensions
    pub fn rect(&self) -> Rectangle {
        let w = 50.0;
        let h = 80.0;
        let x = self.pos.x - (w / 2.0) - 10.0;
        let y = self.pos.y - h;
        Rectangle::new(x, y, w, h)
    }

    // logic tree for picking the right sprite set
    pub fn determine_anim_state(&self) -> AnimationState {
        if self.hit {
            return AnimationState::Hurt;
        }
        if self.shooting {
            return AnimationState::Shoot;
        }

        if !self.grounded {
            return AnimationState::Jump;
        }
        if self.vel.x.abs() > 0.0 {
            return AnimationState::Run;
        }

        AnimationState::Idle
    }

    // figures out where the fireball should spawn relative to the wizard
    pub fn staff_position(&self) -> Vector2 {
        let facing = if self.facing_left { -1.0 } else { 1.0 };

        Vector2::new(
            self.pos.x + facing * (self.width * 0.45),
            self.pos.y - self.height * 0.65,
        )
    }
}