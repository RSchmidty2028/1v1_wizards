use raylib::prelude::*;

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
   pub anim_state: AnimationState,
   pub anim_frame: usize,
   pub anim_timer: f32,
   pub width: f32,
   pub height: f32,
   pub attack_fired: bool,
   pub hit: bool
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationState{
    Idle,
    Run,
    Jump,
    Shoot,
    Hurt,
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
            anim_state: AnimationState::Idle,
            anim_frame: 0,
            anim_timer: 0.0,
            shooting: false,
            width: 64.0,
            height: 128.0,
            attack_fired: false,
            hit: false
        }
    }

    pub fn rect(&self) -> Rectangle {
        Rectangle::new(self.pos.x -self.width /2.0 , self.pos.y-self.height, self.width, self.height)
    }
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
    pub fn staff_position(&self) -> Vector2 {
        let facing = if self.facing_left {-1.0} else {1.0};
        
        Vector2::new(
        self.pos.x + facing * (self.width * 0.45),
        self.pos.y - self.height *0.65)
    }
}