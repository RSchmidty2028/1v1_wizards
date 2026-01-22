use raylib::prelude::*;
use crate::game_data::GameData;

// signals for the manager to swap or stack scenes
pub enum SceneSwitch {
    None,
    Push(Box<dyn Scene>),
    Replace(Box<dyn Scene>),
    Pop,
    Quit,
}

// standard game loop trait
pub trait Scene {
    
    // logic for when the scene first boots up
    fn on_enter(&mut self, _rl: &mut RaylibHandle, _data: &mut GameData) {}

    // keyboard/gamepad checks go here
    fn handle_input(&mut self, _rl: &mut RaylibHandle, _data: &mut GameData) -> SceneSwitch {
        SceneSwitch::None
    }

    // physics and state updates based on delta time
    fn update(&mut self, _dt: f32, _data: &mut GameData) -> SceneSwitch {
        SceneSwitch::None
    }

    // strictly for rendering to the screen
    fn draw(&self, d: &mut RaylibDrawHandle, data: &mut GameData);

    // cleanup logic for when the scene is killed
    fn on_exit(&mut self, _rl: &mut RaylibHandle, _data: &mut GameData) {}
}