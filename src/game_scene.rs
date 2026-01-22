//! The core game play scene
//! 
//! This represents the chase game. Here we store information about the game world and the player's "character".

use raylib::prelude::*;

use crate::menu_scene::WinScene;
use crate::scenes::{Scene, SceneSwitch};
use crate::game_data::GameData;
use crate::utils::*;
use crate::player::{AnimationState, Player};
use crate::projectile::Projectile;

pub struct GameScene {
    players: Vec<Player>,
    gravity: f32,
    projectiles: Vec<Projectile>
}

impl GameScene {
    pub fn new(n: usize, width: i32, height: i32) -> Self {
        Self { 
            gravity: 300.0,
            players: vec![
                Player::new(0,300.0, (height - 15) as f32), //player 1
                Player::new(1,900.0, (height - 15) as f32)  //player 2
            ],
            projectiles: Vec::new()
        }
    }
}

impl Scene for GameScene {
    fn on_enter(&mut self, _rl: &mut RaylibHandle, _data: &mut GameData) {
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
                if axis_x.abs() > 0.1 { direction = axis_x; player.facing_left = true ; }


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
                 if _rl.is_key_down(KeyboardKey::KEY_LEFT) { direction = -1.0; player.facing_left = true;}
                 if _rl.is_key_down(KeyboardKey::KEY_RIGHT) { direction = 1.0; player.facing_left = false; }
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
                player.shoot_timer = 0.8;
                player.shooting = true;
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
                    //player.pos.y = 0.0; // respawn player
                    player.vel = Vector2::zero();
                    player.hit = true;
                }
            }
        }

    for player in &mut self.players{

        let new_state = player.determine_anim_state();
        if new_state == AnimationState::Hurt && player.anim_state != AnimationState::Hurt {
            player.anim_state = AnimationState::Hurt;
            player.anim_frame = 0;
            player.anim_timer = 0.0;
}
    else if new_state == AnimationState::Shoot && player.anim_state != AnimationState:: Shoot {
            player.anim_frame = 0;
            player.anim_timer = 0.0;
            player.attack_fired = false;
            player.anim_state = new_state;
        }
       else if new_state != player.anim_state {
            player.anim_state = new_state;
            player.anim_frame = 0;
            player.anim_timer = 0.0;
        }

        
        let frame_time = match player.anim_state {
            AnimationState::Shoot => 0.2,
            AnimationState::Jump => 0.3,
            _=> 0.1,
        };
        

        let max_frames = match player.anim_state {

            AnimationState::Hurt => _data.player1_hurt_tex.len(),
            AnimationState::Idle => _data.player1_idle_tex.len(),
            AnimationState::Run => _data.player1_run_tex.len(),
            AnimationState::Shoot => _data.player1_attack_tex.len(),
            AnimationState::Jump => _data.player1_jump_tex.len()
        };

        player.anim_timer +=dt;

        if player.anim_timer > frame_time {

            player.anim_timer = 0.0;

        match player.anim_state {
            AnimationState::Run | AnimationState::Idle => {
                player.anim_frame = (player.anim_frame + 1) % max_frames;
            }
            AnimationState::Jump  => {
                if player.anim_frame + 1 < max_frames {
                    player.anim_frame += 1;
                }
            }
            AnimationState::Hurt => {
                if player.anim_frame + 1 < max_frames {
                    player.anim_frame += 1;

                } else {
                    player.hit = false;
                }
            }
            AnimationState::Shoot => {
                let shoot_frame = max_frames - 1;
                if player.anim_frame == shoot_frame && !player.attack_fired {

                    let center = player.staff_position();
                    let speed = 800.0;
                    let direction = if player.aim.length() > 0.0 {
                        player.aim
                    } 
                    else if player.facing_left{
                        Vector2::new(-1.0,0.0)
                    }
                    else {
                        Vector2::new(1.0,0.0)
                    };

                    self.projectiles.push(Projectile::new(center, direction * speed, player.input_id, Color::WHITE));
                
                player.attack_fired = true;

                }

                if player.anim_frame + 1 < max_frames {
                    player.anim_frame += 1;
                }
                else {
                    player.anim_frame = 0;
                    player.shooting = false;
                }
                
            }
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

            if player.pos.y > (floor_y) as f32{
                player.pos.y = (floor_y) as f32;
                player.vel.y = 0.0;
                player.grounded = true;
            }
        }

        SceneSwitch::None
    }

    fn draw(&self, d: &mut RaylibDrawHandle, data: &mut GameData){
        d.clear_background(Color::WHITE);


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



    let player = &self.players[0];

        let base_height:f32 = 128.0;


    let texture = match player.anim_state {
        AnimationState::Idle => &data.player1_idle_tex[player.anim_frame],
        AnimationState::Jump => &data.player1_jump_tex[player.anim_frame],
        AnimationState::Run => &data.player1_run_tex[player.anim_frame],
        AnimationState::Shoot => &data.player1_attack_tex[player.anim_frame],
        AnimationState::Hurt => &data.player1_hurt_tex[player.anim_frame]
    };

    let scale = base_height / 380.0;

        let mut p1_source:Rectangle =  match player.anim_state {
            AnimationState::Hurt | AnimationState::Run | AnimationState::Shoot =>Rectangle::new(0.0, 0.0, texture.width() as f32, (texture.height() - 30) as f32),
            _ => Rectangle::new(0.0,0.0,texture.width() as f32, (texture.height()-10) as f32)
        };
        if player.facing_left {
            p1_source.width = -p1_source.width;
        }
        

        let w = texture.width() as f32 * scale;
        let h = texture.height() as f32 * scale;

        let p1_dest = Rectangle::new(self.players[0].pos.x as f32, self.players[0].pos.y as f32, w, h);

        let p1_origin = Vector2::new(w/2.0, h);

    d.draw_texture_pro(texture, p1_source, p1_dest, p1_origin, 0.0, Color::WHITE);




    let player2 = &self.players[1];

    let texture2 = match player2.anim_state {
        AnimationState::Idle => &data.player2_idle_tex[player2.anim_frame],
        AnimationState::Jump => &data.player2_jump_tex[player2.anim_frame],
        AnimationState::Run => &data.player2_run_tex[player2.anim_frame],
        AnimationState::Shoot => &data.player2_attack_tex[player2.anim_frame],
        AnimationState::Hurt => &data.player2_hurt_tex[player2.anim_frame]
    };
    let w2 = texture2.width() as f32 * scale;
    let h2 = texture2.height() as f32 * scale;


        let mut p2_source:Rectangle =  match player2.anim_state {
            AnimationState::Hurt | AnimationState::Run | AnimationState::Shoot =>Rectangle::new(0.0, 0.0, texture2.width() as f32, (texture2.height()- 80) as f32),
            _ => Rectangle::new(0.0,0.0, texture2.width() as f32, (texture2.height()-10) as f32)
        };
        if player2.facing_left {
            p2_source.width = -p2_source.width;
        }
        let p2_dest = Rectangle::new(self.players[1].pos.x as f32, self.players[1].pos.y as f32, w2, h2);
        let p2_origin = Vector2::new(w2/2.0, h2);
        d.draw_texture_pro(texture2, p2_source, p2_dest, p2_origin, 0.0, Color::WHITE);
        
    for p in &self.projectiles {
        p.draw(d);
    }



            
        
        let ob_1_source = Rectangle::new(16.0, 16.0, 48.0,48.0);
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