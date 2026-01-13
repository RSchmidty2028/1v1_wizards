//! The core game play scene
//! 
//! This represents the chase game. Here we store information about the game world and the player's "character".

use raylib::ffi::{LoadTexture, LoadTextureFromImage};
use raylib::prelude::*;

use crate::menu_scene::WinScene;
use crate::scenes::{Scene, SceneSwitch};
use crate::game_data::GameData;
use crate::utils::*;

pub struct GameScene {
    points: Vec<Vector2>,
    player_position: Vector2,
    player_direction: Vector2,
    player_1_speed: f32,
    gravity: f32,
    player_1_grounded: bool,
    player_1_velo: Vector2
}

impl GameScene {
    pub fn new(n: usize, width: i32, height: i32) -> Self {
        let mut points = Vec::new();
        for _ in 0..n {
            points.push(random_point(width, height));
        }
        Self { 
            points: points,
            player_position: Vector2::new((450) as f32, (835) as f32),
            player_direction: Vector2::zero(),
            player_1_speed: 300.0,
            gravity: 0.8,
            player_1_grounded: true,
            player_1_velo: Vector2::zero(),
        }
    }
}

impl Scene for GameScene {
    fn on_enter(&mut self, _rl: &mut RaylibHandle, _data: &mut GameData) {
        }


    fn handle_input(&mut self, _rl: &mut RaylibHandle, _data: &mut GameData) -> SceneSwitch {
        
        // set the intention to move in the given direction.
        let mut direction = Vector2::zero();
        let gravity: f32 =  0.8;
        if _rl.is_key_down(KeyboardKey::KEY_A) || 
            _rl.is_key_down(KeyboardKey::KEY_LEFT) && self.player_position.x > 15.0
        {
            direction += Vector2::new(-1.0, 0.0);
        }
        
        if _rl.is_key_down(KeyboardKey::KEY_D) || 
            _rl.is_key_down(KeyboardKey::KEY_RIGHT)  && self.player_position.x < 880.0
        {
            direction += Vector2::new(1.0, 0.0);
        }

        if self.player_1_grounded && (_rl.is_key_pressed(KeyboardKey::KEY_W) || _rl.is_key_pressed(KeyboardKey::KEY_UP)){
            self.player_1_velo.y = -1000.0;
            self.player_1_grounded = false;
        }
        
        self.player_direction = direction;
        

        SceneSwitch::None
    }

    fn update(&mut self, _dt: f32, data: &mut GameData) -> SceneSwitch {
        // update position of player, deal with collisions (later ...)
        if !self.player_1_grounded {
            self.player_1_velo.y += self.gravity;
        }

        self.player_position.x += self.player_direction.x * self.player_1_speed * _dt;
        self.player_position.y += self.player_1_velo.y * _dt;

        if self.player_position.y >= 835.0 {
            self.player_position.y = 835.0;
            self.player_1_velo.y = 0.0;
            self.player_1_grounded = true;

        }
        if let Some(last) = self.points.last() {
            // remove the last point.
            if last.distance_to(self.player_position) < 25.0 {
                self.points.pop();
                data.score();
            } 
        } else {
            println!("Deal with win condition, send new scene");
            return SceneSwitch::Push(Box::new(WinScene));
        }


        SceneSwitch::None
    }

    fn draw(&self, d: &mut RaylibDrawHandle, data: &mut GameData){
        d.clear_background(Color::WHITE);
        // Draw player
        d.draw_circle(self.player_position.x as i32,
             self.player_position.y as i32, 
             15.0, 
             Color::BLACK);
        
        // Draw last point in the vector
        if let Some(last) = self.points.last() {
            d.draw_circle(last.x as i32,
             last.y as i32, 
            20.0, 
             Color::BLUE);
        }
        //draw rectangle
            d.draw_rectangle(0,
                 850,
                  900, 
                  50, 
                  Color::ORANGE);
        // Draw score based on game data
        let message = format!("Score: {}", data.points);
        d.draw_text(message.as_str(), 10, data.screen_height - 25, 20, Color::BLACK);
    }

    fn on_exit(&mut self, _rl: &mut RaylibHandle, _data: &mut GameData) {}
}