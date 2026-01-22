use raylib::prelude::*;

use crate::scenes::{Scene, SceneSwitch};
use crate::game_data::GameData;
use crate::utils::*;
use crate::player::{AnimationState, Player};
use crate::projectile::Projectile;
use crate::win_scene::WinScene;

pub struct GameScene {
    players: Vec<Player>,
    gravity: f32,
    projectiles: Vec<Projectile>,
    platforms: Vec<Rectangle>,
    camera: Camera2D,
    is_on_rooftop: bool,
    rooftop_y: f32, 
    scroll_speed: f32,
    lava_y: f32,
    // cleaned up unused walk vars to stop the compiler from complaining
}

impl GameScene {
    pub fn new(_n: usize, _width: i32, _height: i32, zoom: f32) -> Self {
        let mut built_platforms = Vec::new();
        let mut current_y = 1000.0;
        let spacing = 160.0; 
        let rows = 45;
        
        // procedural platform generation
        for i in 0..rows {
            for j in 0..3 {
                let seed = (i * 13 + j * 31) as i32;
                let width = 200.0 + (seed % 150) as f32;
                let x_pos = (seed * 617) % 1500 + 100; 
                built_platforms.push(Rectangle::new(x_pos as f32, current_y, width, 40.0));
            }
            current_y -= spacing;
        }

        Self { 
            gravity: 2400.0,
            players: vec![
                Player::new(0, 600.0, 800.0), 
                Player::new(1, 1320.0, 800.0) 
            ],
            projectiles: Vec::new(),
            platforms: built_platforms,
            camera: Camera2D {
                target: Vector2::new(960.0, 540.0), 
                offset: Vector2::new(_width as f32 / 2.0, _height as f32 / 2.0), 
                rotation: 0.0,
                zoom: zoom,  
            },
            scroll_speed: 60.0, 
            lava_y: 1100.0,
            is_on_rooftop: false,
            rooftop_y: -8500.0,
        }
    }
}

impl Scene for GameScene {
    fn on_enter(&mut self, _rl: &mut RaylibHandle, _data: &mut GameData) {}

    fn handle_input(&mut self, _rl: &mut RaylibHandle, _data: &mut GameData) -> SceneSwitch {
        for player in &mut self.players {
            if player.shoot_timer > 0.0 { player.shoot_timer -= _rl.get_frame_time(); }
            let mut direction = 0.0;

            // controller support
            if _rl.is_gamepad_available(player.input_id) {
                let axis_x = _rl.get_gamepad_axis_movement(player.input_id, GamepadAxis::GAMEPAD_AXIS_LEFT_X);
                if axis_x.abs() > 0.1 { direction = axis_x; player.facing_left = axis_x < 0.0; }
                
                if player.grounded && _rl.is_gamepad_button_pressed(player.input_id, GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_DOWN) {
                    player.vel.y = -1200.0; 
                    player.grounded = false;
                }
                let aim_input = Vector2::new(
                    _rl.get_gamepad_axis_movement(player.input_id, GamepadAxis::GAMEPAD_AXIS_RIGHT_X),
                    _rl.get_gamepad_axis_movement(player.input_id, GamepadAxis::GAMEPAD_AXIS_RIGHT_Y)
                );
                if aim_input.length() > 0.1 { player.aim = aim_input.normalized(); }
            }

            // p1 keyboard controls
            if player.input_id == 0 {
                if _rl.is_key_down(KeyboardKey::KEY_A) { direction = -1.0; player.facing_left = true; }
                if _rl.is_key_down(KeyboardKey::KEY_D) { direction = 1.0; player.facing_left = false; }
                if player.grounded && (_rl.is_key_pressed(KeyboardKey::KEY_W) || _rl.is_key_pressed(KeyboardKey::KEY_SPACE)) {
                    player.vel.y = -1200.0;
                    player.grounded = false;
                }
                // use mouse to aim if no gamepad is plugged in
                if !_rl.is_gamepad_available(player.input_id) {
                    let world_mouse = _rl.get_screen_to_world2D(_rl.get_mouse_position(), self.camera);
                    let diff = world_mouse - Vector2::new(player.pos.x, player.pos.y - 60.0);
                    if diff.length() > 0.0 { player.aim = diff.normalized(); }
                }
            }

            // p2 keyboard controls
            if player.input_id == 1 {
                 if _rl.is_key_down(KeyboardKey::KEY_LEFT) { direction = -1.0; player.facing_left = true; }
                 if _rl.is_key_down(KeyboardKey::KEY_RIGHT) { direction = 1.0; player.facing_left = false; }
                 if player.grounded && (_rl.is_key_pressed(KeyboardKey::KEY_UP) || _rl.is_key_pressed(KeyboardKey::KEY_RIGHT_CONTROL)) {
                      player.vel.y = -1200.0;
                      player.grounded = false;
                 }
            }

            player.vel.x = direction * 350.0;

            // shooting logic for both players
            let mut shoot_pressed = false;
            if player.input_id == 0 && _rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) { shoot_pressed = true; }
            if player.input_id == 1 && _rl.is_key_pressed(KeyboardKey::KEY_ENTER) { shoot_pressed = true; }
            if _rl.is_gamepad_available(player.input_id) && _rl.get_gamepad_axis_movement(player.input_id, GamepadAxis::GAMEPAD_AXIS_RIGHT_TRIGGER) > 0.5 { shoot_pressed = true; }

            if shoot_pressed && player.shoot_timer <= 0.0 {
                player.shoot_timer = 0.8;
                player.shooting = true; 
            }
        }
        SceneSwitch::None
    }

    fn update(&mut self, dt: f32, _data: &mut GameData) -> SceneSwitch {
        if !self.is_on_rooftop {
            // scrolling up gets faster over time
            let growth_factor = 0.09; 
            self.scroll_speed += (self.scroll_speed * growth_factor) * dt;
            if self.scroll_speed > 250.0 { self.scroll_speed = 250.0; }
            self.camera.target.y -= self.scroll_speed * dt;
            self.lava_y = self.camera.target.y + 480.0;

            // check if we reached the roof for the final duel
            if self.camera.target.y <= self.rooftop_y {
                self.is_on_rooftop = true;
                self.camera.target.y = self.rooftop_y;
                self.platforms.clear(); 
                self.platforms.push(Rectangle::new(-5000.0, self.rooftop_y + 400.0, 10000.0, 100.0));
            }

            // platform recycling to keep the tower going
            let tower_height = 7200.0;
            let bottom_limit = self.lava_y + 100.0;
            for plat in &mut self.platforms {
                if plat.y > bottom_limit {
                    plat.y -= tower_height;
                    let seed = (plat.y.abs() as i32 + plat.x as i32) * 17;
                    plat.x = 100.0 + (seed % 1400) as f32;
                    plat.width = 200.0 + (seed % 150) as f32;
                }
            }
        }

        for player in &mut self.players {
            // figure out what animation should be playing
            let new_state = player.determine_anim_state();
            if new_state != player.anim_state {
                player.anim_state = new_state;
                player.anim_frame = 0;
                player.anim_timer = 0.0;
                if new_state == AnimationState::Shoot { player.attack_fired = false; }
            }

            // pick the right texture list
            let tex_vec = match (player.input_id, player.anim_state) {
                (0, AnimationState::Idle) => &_data.player1_idle_tex,
                (0, AnimationState::Run) => &_data.player1_run_tex,
                (0, AnimationState::Jump) => &_data.player1_jump_tex,
                (0, AnimationState::Shoot) => &_data.player1_attack_tex,
                (0, AnimationState::Hurt) => &_data.player1_hurt_tex,
                (1, AnimationState::Idle) => &_data.player2_idle_tex,
                (1, AnimationState::Run) => &_data.player2_run_tex,
                (1, AnimationState::Jump) => &_data.player2_jump_tex,
                (1, AnimationState::Shoot) => &_data.player2_attack_tex,
                _ => &_data.player2_hurt_tex,
            };

            // handle frame timing
            player.anim_timer += dt;
            if player.anim_timer > 0.1 {
                player.anim_timer = 0.0;
                let max_frames = tex_vec.len();
                match player.anim_state {
                    AnimationState::Run | AnimationState::Idle => player.anim_frame = (player.anim_frame + 1) % max_frames,
                    AnimationState::Jump | AnimationState::Hurt => if player.anim_frame + 1 < max_frames { player.anim_frame += 1; } else if player.anim_state == AnimationState::Hurt { player.hit = false; },
                    AnimationState::Shoot => {
                        // fire the projectile at the right frame
                        if player.anim_frame == max_frames - 1 && !player.attack_fired {
                            self.projectiles.push(Projectile::new(player.staff_position(), player.aim * 800.0, player.input_id, Color::WHITE));
                            player.attack_fired = true;
                        }
                        if player.anim_frame + 1 < max_frames { player.anim_frame += 1; } else { player.anim_frame = 0; player.shooting = false; }
                    }
                }
            }
        }

        // move projectiles and check for hits
        for p in &mut self.projectiles { p.update(dt); }
        for p in &mut self.projectiles {
            if !p.active { continue; }
            for player in &mut self.players {
                if p.owner_id == player.input_id || player.is_invincible() { continue; }
                if check_collision_circle_rec(p.pos, 10.0, player.rect()) {
                    p.active = false;
                    player.hp -= 1;
                    player.i_frame_timer = 1.5;
                    player.hit = true; 
                    if player.hp <= 0 {
                        _data.player_scores[p.owner_id as usize] += 1;
                        player.hp = 3; 
                        player.pos = Vector2::new(self.camera.target.x, self.camera.target.y - 400.0); 
                        player.vel = Vector2::zero();
                    }
                }
            }
        }
        self.projectiles.retain(|p| p.active);

        // screen boundary logic
        let view_half_width = _data.screen_width as f32 / _data.zoom_factor / 2.0;
        let left_edge = self.camera.target.x - view_half_width;
        let right_edge = self.camera.target.x + view_half_width;

        for player in &mut self.players {
            if player.i_frame_timer > 0.0 { player.i_frame_timer -= dt; }
            player.vel.y += self.gravity * dt;
            player.pos += player.vel * dt;

            let buffer = 64.0; 
            if player.pos.x < left_edge + buffer { player.pos.x = left_edge + buffer; player.vel.x = 0.0; }
            if player.pos.x > right_edge - buffer { player.pos.x = right_edge - buffer; player.vel.x = 0.0; }

            // floor collisions
            player.grounded = false;
            for plat in &self.platforms {
                // only snap to top of platforms if falling down
                if player.vel.y > 0.0 && player.pos.y >= plat.y && player.pos.y <= plat.y + 30.0 && 
                   player.pos.x + 23.0 > plat.x && player.pos.x - 23.0 < plat.x + plat.width {
                    player.pos.y = plat.y;
                    player.vel.y = 0.0;           
                    player.grounded = true;       
                }
            }

            // lava death check
            if player.pos.y > self.lava_y { 
                if !player.is_invincible() { player.hp -= 1; player.i_frame_timer = 1.0; player.vel.y = -650.0; player.hit = true; }
                if player.hp <= 0 || player.pos.y > self.lava_y + 200.0 {
                    _data.player_scores[if player.input_id == 0 { 1 } else { 0 }] += 1;
                    player.hp = 3; 
                    player.pos = Vector2::new(self.camera.target.x, self.camera.target.y - 400.0); 
                    player.vel = Vector2::zero();
                }
            }
        }
        
        // win condition
        if _data.player_scores[0] >= 5 || _data.player_scores[1] >= 5 { return SceneSwitch::Push(Box::new(WinScene)); }
        SceneSwitch::None
    }

    fn draw(&self, d: &mut RaylibDrawHandle, data: &mut GameData){
        d.clear_background(Color::WHITE);
        // store time here to avoid borrow checker issues later
        let time = d.get_time();
        
        // render background
        if !data.background_tex_vec.is_empty() {
            d.draw_texture_pro(&data.background_tex_vec[0], 
                Rectangle::new(0.0,0.0,576.0,324.0), 
                Rectangle::new(0.0,0.0,d.get_screen_width() as f32, d.get_screen_height() as f32), 
                Vector2::zero(), 0.0, Color::WHITE);
        }

        {
            let mut d_cam = d.begin_mode2D(self.camera);
            
            // render platforms
            for (i, plat) in self.platforms.iter().enumerate() {
                if self.is_on_rooftop && i == self.platforms.len() - 1 {
                    d_cam.draw_rectangle_rec(*plat, Color::DARKGRAY);
                    d_cam.draw_rectangle_lines_ex(*plat, 5.0, Color::BLACK);
                } else {
                    let tex = &data.obstacle_tex_vec[(i % 3) % data.obstacle_tex_vec.len()];
                    d_cam.draw_texture_pro(tex, Rectangle::new(0.0,16.0,48.0,15.0), *plat, Vector2::zero(), 0.0, Color::WHITE);
                }
            }

            // magical energy ball effects
            for p in &self.projectiles { 
                // p1 is blueish magic, p2 is fiery orange
                let (core_color, glow_color) = if p.owner_id == 0 {
                    (Color::CYAN, Color::BLUE.alpha(0.4))
                } else {
                    (Color::ORANGE, Color::RED.alpha(0.4))
                };
                d_cam.draw_circle_v(p.pos, 13.0, glow_color);
                d_cam.draw_circle_v(p.pos, 8.0, core_color);
            }

            for (index, player) in self.players.iter().enumerate() {
                // blink effect for invincibility frames
                let mut tint = Color::WHITE;
                if player.is_invincible() { 
                    tint = tint.alpha((((time * 20.0).sin() + 1.0) / 2.0) as f32); 
                }

                let tex_vec = match (index, player.anim_state) {
                    (0, AnimationState::Idle) => &data.player1_idle_tex,
                    (0, AnimationState::Run) => &data.player1_run_tex,
                    (0, AnimationState::Jump) => &data.player1_jump_tex,
                    (0, AnimationState::Shoot) => &data.player1_attack_tex,
                    (0, AnimationState::Hurt) => &data.player1_hurt_tex,
                    (1, AnimationState::Idle) => &data.player2_idle_tex,
                    (1, AnimationState::Run) => &data.player2_run_tex,
                    (1, AnimationState::Jump) => &data.player2_jump_tex,
                    (1, AnimationState::Shoot) => &data.player2_attack_tex,
                    _ => &data.player2_hurt_tex,
                };

                // draw the wizard
                if !tex_vec.is_empty() {
                    let texture = &tex_vec[player.anim_frame % tex_vec.len()];
                    let flip = if player.facing_left { -1.0 } else { 1.0 };
                    
                    let scale = 128.0 / 380.0;
                    let dest = Rectangle::new(player.pos.x, player.pos.y, texture.width() as f32 * scale, texture.height() as f32 * scale);
                    let mut source = Rectangle::new(0.0, 0.0, texture.width() as f32, (texture.height() - 30) as f32);
                    source.width *= flip;
                    d_cam.draw_texture_pro(texture, source, dest, Vector2::new((texture.width() as f32 * scale)/2.0, texture.height() as f32 * scale), 0.0, tint);
                }
                
                // hp heart icons
                if !data.ui_assets_tex_vec.is_empty() {
                    let heart_tex = &data.ui_assets_tex_vec[0];
                    let start_x = player.pos.x - ((player.hp as f32 * 50.0) / 2.0);
                    for h in 0..player.hp {
                        d_cam.draw_texture_pro(
                            heart_tex, 
                            Rectangle::new(0.0,0.0,heart_tex.width as f32, heart_tex.height as f32), 
                            Rectangle::new(start_x + (h as f32 * 50.0), player.pos.y - 200.0, 90.0, 90.0), 
                            Vector2::zero(), 
                            0.0, 
                            tint
                        );
                    }
                }
            }
            
            // repeating lava floor textures
            let lava_w = data.lava_tex.width() as f32;
            let lava_h = data.lava_tex.height() as f32;
            let left_x = -3000.0;
            let right_x = 3000.0;
            let bottom_y = self.camera.target.y + 3000.0;
            let cols = ((right_x - left_x) / (lava_w - 0.5)).ceil() as i32;
            for col in 0..cols {
                let x = left_x + col as f32 * (lava_w - 0.5);
                d_cam.draw_texture_pro(&data.lava_tex, Rectangle::new(0.0, 0.0, lava_w, lava_h / 2.0), Rectangle::new(x, self.lava_y, lava_w, lava_h / 2.0), Vector2::zero(), 0.0, Color::WHITE);
            }
            d_cam.draw_texture_pro(&data.lava_tex, Rectangle::new(0.0, lava_h / 2.0, lava_w, lava_h / 2.0), Rectangle::new(left_x, self.lava_y + lava_h / 2.0, right_x - left_x, bottom_y - self.lava_y), Vector2::zero(), 0.0, Color::WHITE);
            if self.is_on_rooftop { d_cam.draw_text("FINAL DUEL!", 750, (self.rooftop_y - 200.0) as i32, 60, Color::BLACK); }
        } 
        
        // ui overlay
        d.draw_text(&format!("P1 Score: {}", data.player_scores[0]), 20, 20, 30, Color::BLUE);
        d.draw_text(&format!("P2 Score: {}", data.player_scores[1]), d.get_screen_width() - 250, 20, 30, Color::RED);
    }

    fn on_exit(&mut self, _rl: &mut RaylibHandle, _data: &mut GameData) {}
}