use raylib::prelude::*;
use crate::game_data::GameData;
use crate::menu_scene::MenuScene;
use crate::scenes::{Scene, SceneSwitch};

pub struct WinScene;

impl Scene for WinScene {
    fn on_enter(&mut self, _rl: &mut RaylibHandle, _data: &mut GameData) {
        // just a log to let us know we made it here
        println!("entered win scene");
    }

    fn handle_input(&mut self, _rl: &mut RaylibHandle, _data: &mut GameData) -> SceneSwitch {
        // checking for start buttons or enter to head home
        let p1_start = _rl.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_MIDDLE_RIGHT);
        let p2_start = _rl.is_gamepad_button_pressed(1, GamepadButton::GAMEPAD_BUTTON_MIDDLE_RIGHT);
        let enter = _rl.is_key_pressed(KeyboardKey::KEY_ENTER);

        if p1_start || p2_start || enter {
            // go back to menu - main.rs handles the push
            return SceneSwitch::Push(Box::new(MenuScene));
        }
        SceneSwitch::None
    }

    fn update(&mut self, _dt: f32, _data: &mut GameData) -> SceneSwitch {
        SceneSwitch::None
    }

    fn draw(&self, d: &mut RaylibDrawHandle, data: &mut GameData) {
        d.clear_background(Color::BLACK);

        let screen_w = d.get_screen_width();
        let screen_h = d.get_screen_height();

        // figure out who hit the 5 point cap
        let p1_score = data.player_scores[0];

        let (winner_text, color) = if p1_score >= 5 {
            ("PLAYER 1 WINS!", Color::BLUE)
        } else {
            ("PLAYER 2 WINS!", Color::RED)
        };

        // centering math for the big text
        let font_size = 80;
        let text_width = d.measure_text(winner_text, font_size);
        let center_x = (screen_w / 2) - (text_width / 2);
        let center_y = screen_h / 3;

        // basic outline effect
        d.draw_text(winner_text, center_x - 4, center_y, font_size, Color::WHITE);
        d.draw_text(winner_text, center_x + 4, center_y, font_size, Color::WHITE);
        d.draw_text(winner_text, center_x, center_y - 4, font_size, Color::WHITE);
        d.draw_text(winner_text, center_x, center_y + 4, font_size, Color::WHITE);
        
        // main colored text
        d.draw_text(winner_text, center_x, center_y, font_size, color);

        // instructions to go back
        let sub_text = "PRESS START TO RETURN";
        let sub_size = 40;
        let sub_width = d.measure_text(sub_text, sub_size);
        d.draw_text(sub_text, (screen_w / 2) - (sub_width / 2), screen_h / 2, sub_size, Color::GRAY);
    }

    fn on_exit(&mut self, _rl: &mut RaylibHandle, _data: &mut GameData) {}
}