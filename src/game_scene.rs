//! The core game play scene
//! 
//! This represents the chase game. Here we store information about the game world and the player's "character".

use raylib::prelude::*;

use crate::menu_scene::WinScene;
use crate::scenes::{Scene, SceneSwitch};
use crate::game_data::GameData;
use crate::utils::*;
use crate::player::Player;
use crate::projectile::Projectile;

pub struct GameScene {
    players: Vec<Player>,
    // player_position: Vector2,
    // player_direction: Vector2,
    // player_1_speed: f32,
    gravity: f32,
    // player_1_grounded: bool,
    // player_1_velo: Vector2,
    walk_frame: usize,
    walk_timing: f32,
    frame_time: f32,
    projectiles: Vec<Projectile>
}

impl GameScene {
    pub fn new(n: usize, width: i32, height: i32) -> Self {
        Self { 
            // player_position: Vector2::new((450) as f32, (width - 15) as f32),
            // player_direction: Vector2::zero(),
            // player_1_speed: 300.0,
            gravity: 300.0,
            players: vec![
                Player::new(0,300.0, (height - 15) as f32), //player 1
                Player::new(1,900.0, (height - 15) as f32) //player 2
            ],
            // player_1_grounded: true,
            // player_1_velo: Vector2::zero(),
            walk_frame: 0,
            walk_timing: 0.0,
            frame_time: 0.1,
            projectiles: Vec::new()
        }
    }
}

impl Scene for GameScene {
    fn on_enter(&mut self, _rl: &mut RaylibHandle, _data: &mut GameData) {
        self.walk_frame = 0;
        self.walk_timing = 0.0;
        }



    fn handle_input(&mut self, _rl: &mut RaylibHandle, _data: &mut GameData) -> SceneSwitch {
       let mut new_shots = Vec::new(); 

        for player in &mut self.players {
            

            
            // firing cooldown
            if player.shoot_timer > 0.0 {
                player.shoot_timer -= _rl.get_frame_time();
            }

            let mut direction = 0.0;
            let mut aimed_with_stick = false;

            // gamepad input
            if _rl.is_gamepad_available(player.input_id) {
                // move
                let axis_x = _rl.get_gamepad_axis_movement(player.input_id, GamepadAxis::GAMEPAD_AXIS_LEFT_X);
                if axis_x.abs() > 0.1 { direction = axis_x; player.facing_left == true ; }


                // jump
                if player.grounded && _rl.is_gamepad_button_pressed(player.input_id, GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_DOWN) {
                    player.vel.y = -550.0;
                    player.grounded = false;
                }
                
                // aim
                let aim_x = _rl.get_gamepad_axis_movement(player.input_id, GamepadAxis::GAMEPAD_AXIS_RIGHT_X);
                let aim_y = _rl.get_gamepad_axis_movement(player.input_id, GamepadAxis::GAMEPAD_AXIS_RIGHT_Y);
                let aim_input = Vector2::new(aim_x, aim_y);
                
                if aim_input.length() > 0.1 { 
                    player.aim = aim_input.normalized(); 
                    aimed_with_stick = true;
                }
            }

            // kbm input (player 1 only)
            if player.input_id == 0 {
                if _rl.is_key_down(KeyboardKey::KEY_A) { direction = -1.0; player.facing_left = true; }
                if _rl.is_key_down(KeyboardKey::KEY_D) { direction = 1.0; player.facing_left = false; }
                if player.grounded && (_rl.is_key_pressed(KeyboardKey::KEY_W) || _rl.is_key_pressed(KeyboardKey::KEY_SPACE)) {
                    player.vel.y = -550.0;
                    player.grounded = false;
                }
                
                // only use mouse if controller isnt aiming
                if !aimed_with_stick {
                    let mouse_pos = _rl.get_mouse_position();
                    let center = Vector2::new(player.pos.x + 15.0, player.pos.y + 15.0);
                    let diff = mouse_pos - center;
                    if diff.length() > 0.0 { player.aim = diff.normalized(); }
                }
            }
            
            // keyboard input (player 2)
            if player.input_id == 1 {
                 if _rl.is_key_down(KeyboardKey::KEY_LEFT) { direction = -1.0; }
                 if _rl.is_key_down(KeyboardKey::KEY_RIGHT) { direction = 1.0; }
                 if player.grounded && (_rl.is_key_pressed(KeyboardKey::KEY_UP) || _rl.is_key_pressed(KeyboardKey::KEY_RIGHT_CONTROL)) {
                     player.vel.y = -550.0;
                     player.grounded = false;
                 }
            }

            player.vel.x = direction * 300.0;

            // shooting logic
            let mut shoot = false;

            // gamepad trigger
            if _rl.is_gamepad_available(player.input_id) {
                let trigger = _rl.get_gamepad_axis_movement(player.input_id, GamepadAxis::GAMEPAD_AXIS_RIGHT_TRIGGER);
                if trigger > 0.5 { shoot = true; }
            }

            // mouse/key press
            if player.input_id == 0 && _rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) { shoot = true; }
            if player.input_id == 1 && _rl.is_key_pressed(KeyboardKey::KEY_ENTER) { shoot = true; }

            // fire if button pressed AND cooldown is 0
            if shoot && player.shoot_timer <= 0.0 {
                let center = Vector2::new(player.pos.x + 15.0, player.pos.y + 15.0);
                let speed = 800.0;
                new_shots.push(Projectile::new(center, player.aim * speed, player.input_id, Color::BLACK));
                
                // reset timer (0.5s delay)
                player.shoot_timer = 0.5; 
            }
        }

        self.projectiles.extend(new_shots);
        SceneSwitch::None
    }

    fn update(&mut self, dt: f32, _data: &mut GameData) -> SceneSwitch {
        // move projectiles
        for p in &mut self.projectiles {
            p.update(dt);
        }

        // collision logic
        for p in &mut self.projectiles {
            if !p.active { continue; }

            for player in &mut self.players {
                // dont shoot urself
                if p.owner_id == player.input_id { continue;}
                // hit detection
                if check_collision_circle_rec(p.pos, 10.0, player.rect()) {
                    p.active = false; // destroy bullet
                    player.pos.y = 0.0; // respawn player
                    player.vel = Vector2::zero();
                }
            }
        }
        for player in &mut self.players {
             let player_1_moving = player.vel.x != 0.0;
                     if player_1_moving {
                        self.walk_timing += dt;
                        self.frame_time = 0.1;

                    if self.walk_timing >= self.frame_time {
                        self.walk_timing = 0.0;
                        self.walk_frame = (self.walk_frame + 1) % _data.player_walk_tex.len();
            }
        } 
        }

        // clean up old bullets
        self.projectiles.retain(|p| p.active);

        // physics update
        let floor_y = _data.screen_height; 
        for player in &mut self.players {
            player.vel.y += self.gravity * dt;
            player.pos.x += player.vel.x * dt;
            player.pos.y += player.vel.y * dt;

            if player.pos.y > floor_y as f32{
                player.pos.y = floor_y as f32;
                player.vel.y = 0.0;
                player.grounded = true;
            }
        }

        SceneSwitch::None
    }

    fn draw(&self, d: &mut RaylibDrawHandle, data: &mut GameData){
        d.clear_background(Color::WHITE);
        // // Draw last point in the vector
        // if let Some(last) = self.points.last() {
        //     d.draw_circle(last.x as i32,
        //      last.y as i32, 
        //     20.0, 
        //      Color::BLUE);
        // }


        let back_source = Rectangle::new(
            0.0, 
            0.0, 
            576.0, 
            324.0);
            
        let back_draw = Rectangle::new(
            0.0,
            0.0,
             data.screen_width as f32,
             data.screen_height as f32);

        let back_origin = Vector2::new(0.0, 0.0);

        d.draw_texture_pro(
            &data.background_tex_vec[0],
             back_source,
              back_draw,back_origin,
               0.0,
                Color::WHITE);



        let r1; 
        if self.players[0].facing_left {
             r1 = Rectangle::new(0.0, 0.0, -419.0, 380.0);
        }
        else {
            r1 = Rectangle::new(0.0,0.0,419.0,380.0);
        }
        let r2 = Rectangle::new(self.players[0].pos.x as f32, self.players[0].pos.y as f32, 128.0, 128.0);
        let origin = Vector2::new(64.0, 110.0);
        d.draw_texture_pro(&data.player_walk_tex[self.walk_frame], r1, r2, origin, 0.0, Color::WHITE);
        
        let ob_1_source = Rectangle::new(0.0, 0.0, 64.0,64.0);
        let ob_1_destination = Rectangle::new(500.0, 800.0, 128.0, 128.0);
        let ob_origin = Vector2::new(0.0,0.0);

        let ob_2source = Rectangle::new(0.0,0.0,64.0,64.0);
        let ob_2_destination = Rectangle::new(628.0,800.0, 128.0,128.0);
        let ob_2_origin = Vector2::new(0.0,0.0);

        let ob_3source = Rectangle::new(0.0,0.0,64.0,64.0);
        let ob_3_destination = Rectangle::new(756.0,800.0, 128.0,128.0);
        let ob_3_origin = Vector2::new(0.0,0.0);
    
        d.draw_texture_pro(&data.obstacle_tex_vec[0],ob_1_source, ob_1_destination, ob_origin, 0.0, Color::WHITE);
        d.draw_texture_pro(&data.obstacle_tex_vec[1], ob_2source, ob_2_destination, ob_2_origin, 0.0, Color::WHITE);
        d.draw_texture_pro(&data.obstacle_tex_vec[2], ob_3source, ob_3_destination, ob_3_origin, 0.0, Color::WHITE);
        
        
        // Draw score based on game data
        let message = format!("Score: {}", data.points);

        d.draw_text(message.as_str(), 10, data.screen_height - 25, 20, Color::BLACK);
    }

    fn on_exit(&mut self, _rl: &mut RaylibHandle, _data: &mut GameData) {}
}