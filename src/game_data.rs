//! The data for each game session. 
//! 
//! You could also store data associated with each human player here.
//! We could also store the player's gamepad_id here.

//use raylib::ffi::{Texture2D, Vector2};
use raylib::texture::Texture2D;

pub struct GameData {
    pub points: u32,
    pub screen_width: i32,
    pub screen_height: i32,
    pub player1_run_tex: Vec<Texture2D>,
    pub player2_run_tex: Vec<Texture2D>,
     pub player1_idle_tex: Vec<Texture2D>,
     pub player2_idle_tex: Vec<Texture2D>,
     pub player1_jump_tex: Vec<Texture2D>,
     pub player2_jump_tex: Vec<Texture2D>,
     pub player1_hurt_tex: Vec<Texture2D>,
     pub player2_hurt_tex: Vec<Texture2D>,
     pub player1_attack_tex: Vec<Texture2D>,
     pub player2_attack_tex: Vec<Texture2D>,
    pub p1_facing_left: bool,
     pub p2_facing_left: bool,
    pub p1_current_state: f32,
    pub background_tex_vec: Vec<Texture2D>,
    pub obstacle_tex_vec: Vec<Texture2D>
}

impl GameData {
    pub fn new(
        width: i32,
         height: i32,
         player1_run: Vec<Texture2D>,
         player2_run: Vec<Texture2D>,
        player1_idle: Vec<Texture2D>,
        player2_idle: Vec<Texture2D>,
        player1_hurt: Vec<Texture2D>,
        player2_hurt: Vec<Texture2D>,
        player1_jump: Vec<Texture2D>,
        player2_jump: Vec<Texture2D>,
        player1_attack: Vec<Texture2D>,
        player2_attack: Vec<Texture2D>,
        background_tex: Vec<Texture2D>,
        obstacle_tex: Vec<Texture2D>
        )-> Self  {
        Self {
            points: 0,
            screen_width: width,
            screen_height: height,
            player1_idle_tex: player1_idle,
            player1_attack_tex: player1_attack,
            player2_attack_tex: player2_attack,
            player2_run_tex: player2_run,
            player1_run_tex: player1_run,
            player1_hurt_tex: player1_hurt,
            player2_hurt_tex: player2_hurt,
            player1_jump_tex: player1_jump,
            player2_jump_tex: player2_jump,
            p1_facing_left: false,
            p1_current_state: 0.0,
            background_tex_vec: background_tex,
            obstacle_tex_vec: obstacle_tex,
            player2_idle_tex: player2_idle,
            p2_facing_left: true,
        }
    }

    /// add one to the player's total points.
    pub fn score(&mut self) {
        self.points += 1;
    }
}