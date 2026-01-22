//! The data for each game session.
//!
//! This stores the global game state, including player scores, screen dimensions,
//! and all texture assets used across different scenes.
use raylib::texture::Texture2D;

pub struct GameData {
    pub zoom_factor: f32,       // keeps things proportional on different screens
    pub player_scores: Vec<i32>, // [p1 score, p2 score]
    pub screen_width: i32,
    pub screen_height: i32,
    
    // p1 sprites
    pub player1_run_tex: Vec<Texture2D>,
    pub player1_idle_tex: Vec<Texture2D>,
    pub player1_jump_tex: Vec<Texture2D>,
    pub player1_hurt_tex: Vec<Texture2D>,
    pub player1_attack_tex: Vec<Texture2D>,
    
    // p2 sprites
    pub player2_run_tex: Vec<Texture2D>,
    pub player2_idle_tex: Vec<Texture2D>,
    pub player2_jump_tex: Vec<Texture2D>,
    pub player2_hurt_tex: Vec<Texture2D>,
    pub player2_attack_tex: Vec<Texture2D>,
    
    // misc game state
    pub p1_facing_left: bool,
    pub p2_facing_left: bool,
    pub p1_current_state: f32,
    pub background_tex_vec: Vec<Texture2D>,
    pub obstacle_tex_vec: Vec<Texture2D>,
    pub ui_assets_tex_vec: Vec<Texture2D>,
    pub lava_tex: Texture2D,
}

impl GameData {
    pub fn new(
        width: i32,
        height: i32,
        zoom: f32,
        p1_run: Vec<Texture2D>, p1_idle: Vec<Texture2D>, p1_jump: Vec<Texture2D>, p1_hurt: Vec<Texture2D>, p1_attack: Vec<Texture2D>,
        p2_run: Vec<Texture2D>, p2_idle: Vec<Texture2D>, p2_jump: Vec<Texture2D>, p2_hurt: Vec<Texture2D>, p2_attack: Vec<Texture2D>,
        background_tex: Vec<Texture2D>,
        obstacle_tex: Vec<Texture2D>,
        ui_assets: Vec<Texture2D>,
        lava_tex: Texture2D,
    ) -> Self {
        Self {
            zoom_factor: zoom,
            player_scores: vec![0, 0],
            screen_width: width,
            screen_height: height,
            player1_run_tex: p1_run,
            player1_idle_tex: p1_idle,
            player1_jump_tex: p1_jump,
            player1_hurt_tex: p1_hurt,
            player1_attack_tex: p1_attack,
            player2_run_tex: p2_run,
            player2_idle_tex: p2_idle,
            player2_jump_tex: p2_jump,
            player2_hurt_tex: p2_hurt,
            player2_attack_tex: p2_attack,
            p1_facing_left: false,
            p2_facing_left: true,
            p1_current_state: 0.0,
            background_tex_vec: background_tex,
            obstacle_tex_vec: obstacle_tex,
            ui_assets_tex_vec: ui_assets,
            lava_tex,
        }
    }

    // just increments the score for whichever player won the round
    pub fn score(&mut self, player_id: usize) {
        if player_id < self.player_scores.len() {
            self.player_scores[player_id] += 1;
        }
    }
}