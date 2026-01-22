use raylib::prelude::*;
use crate::game_data::GameData;
use crate::game_scene::GameScene;
use crate::scenes::{Scene, SceneSwitch};

pub struct MenuScene;

impl Scene for MenuScene {
    fn on_enter(&mut self, _rl: &mut RaylibHandle, _data: &mut GameData) {}

    fn handle_input(&mut self, rl: &mut RaylibHandle, data: &mut GameData) -> SceneSwitch {
        // start game on enter or gamepad start
        if rl.is_key_pressed(KeyboardKey::KEY_ENTER)
            || rl.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_MIDDLE_RIGHT)
        {
            // reset scores for the fresh run
            data.player_scores = vec![0, 0];
            return SceneSwitch::Push(Box::new(GameScene::new(
                0,
                data.screen_width,
                data.screen_height,
                data.zoom_factor,
            )));
        }
        SceneSwitch::None
    }

    fn update(&mut self, _dt: f32, _data: &mut GameData) -> SceneSwitch {
        SceneSwitch::None
    }

    fn draw(&self, d: &mut RaylibDrawHandle, data: &mut GameData) {
        // fallback if bg fails
        d.clear_background(Color::BLACK);

        let screen_w = d.get_screen_width() as f32;
        let screen_h = d.get_screen_height() as f32;

        // render that main menu background
        if let Some(bg) = data.background_tex_vec.first() {
            d.draw_texture_pro(
                bg,
                Rectangle::new(0.0, 0.0, bg.width as f32, bg.height as f32),
                Rectangle::new(0.0, 0.0, screen_w, screen_h),
                Vector2::zero(),
                0.0,
                Color::WHITE,
            );
        }

        // just a subtle dark tint to make the title pop
        d.draw_rectangle(0, 0, screen_w as i32, screen_h as i32, Color::new(0, 0, 0, 90));

        let title_size = 100;
        let outline_thickness = 4;
        let letter_spacing = 40; 
        let title_y = (screen_h / 3.0) as i32;

        // split title into segments for custom colors
        let segments = vec![
            ("WIZARD", Color::BLACK, Color::WHITE),
            ("1", Color::PURPLE, Color::ORANGE),
            ("v", Color::BLACK, Color::WHITE),
            ("1", Color::ORANGE, Color::PURPLE),
        ];

        // math to keep the multi-colored title centered
        let text_widths: Vec<i32> = segments.iter()
            .map(|(text, _, _)| d.measure_text(text, title_size))
            .collect();
        
        let total_text_width: i32 = text_widths.iter().sum();
        let total_spacing = (segments.len() as i32 - 1) * letter_spacing;
        let final_total_width = total_text_width + total_spacing;

        let mut current_x = (screen_w / 2.0 - final_total_width as f32 / 2.0) as i32;

        for (i, (text, main_color, outline_color)) in segments.into_iter().enumerate() {
            // cheap shadow/outline effect
            d.draw_text(text, current_x - outline_thickness, title_y, title_size, outline_color);
            d.draw_text(text, current_x + outline_thickness, title_y, title_size, outline_color);
            d.draw_text(text, current_x, title_y - outline_thickness, title_size, outline_color);
            d.draw_text(text, current_x, title_y + outline_thickness, title_size, outline_color);

            // actual text layer
            d.draw_text(text, current_x, title_y, title_size, main_color);

            current_x += text_widths[i] + letter_spacing;
        }

        // start prompt at the bottom
        let prompt = "PRESS ENTER / START";
        let prompt_size = 30;
        let prompt_width = d.measure_text(prompt, prompt_size);

        d.draw_text(
            prompt,
            (screen_w / 2.0 - prompt_width as f32 / 2.0) as i32,
            (screen_h / 2.0 + 80.0) as i32,
            prompt_size,
            Color::LIGHTGRAY,
        );
    }

    fn on_exit(&mut self, _rl: &mut RaylibHandle, _data: &mut GameData) {}
}