use raylib::prelude::*;
use raylib_framework_testing::game_data::GameData;
use raylib_framework_testing::menu_scene::MenuScene;
use raylib_framework_testing::scenes::{Scene, SceneSwitch};

use std::fs::OpenOptions;
use std::sync::Arc;
use std::time::Instant;
use tracing_subscriber::prelude::*;

fn main() {
    // window and monitor setup
    let base_height: i32 = 1080;

    let (mut rl, thread) = raylib::init()
        .resizable() 
        .title("Wizard Duel")
        .build();

    rl.maximize_window();
    rl.toggle_fullscreen();

    // scaling everything based on the monitor height
    let monitor_w = rl.get_screen_width(); 
    let monitor_h = rl.get_screen_height();
    let zoom_factor = (monitor_h as f32) / (base_height as f32);

    // debug logging to a file so we can see what's breaking
    let stdout_log = tracing_subscriber::fmt::layer().pretty();
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("debug.log")
        .expect("Failed to open log file");

    let debug_log = tracing_subscriber::fmt::layer()
        .json()
        .with_writer(Arc::new(file));

    tracing_subscriber::Registry::default()
        .with(stdout_log)
        .with(debug_log)
        .init();

    // loading all the player 1 stuff
    let run1_vec = vec![
        rl.load_texture(&thread, "./resources/p1_RUN_000.png").unwrap(),
        rl.load_texture(&thread, "./resources/p1_RUN_001.png").unwrap(),
        rl.load_texture(&thread, "./resources/p1_RUN_002.png").unwrap(),
        rl.load_texture(&thread, "./resources/p1_RUN_003.png").unwrap(),
        rl.load_texture(&thread, "./resources/p1_RUN_004.png").unwrap(),
    ];
    let idle1_vec = vec![
        rl.load_texture(&thread, "./resources/1_IDLE_000.png").unwrap(),
        rl.load_texture(&thread, "./resources/1_IDLE_001.png").unwrap(),
        rl.load_texture(&thread, "./resources/1_IDLE_002.png").unwrap(),
        rl.load_texture(&thread, "./resources/1_IDLE_003.png").unwrap(),
        rl.load_texture(&thread, "./resources/1_IDLE_004.png").unwrap(),
    ];
    let jump1_vec = vec![
        rl.load_texture(&thread, "./resources/4_JUMP_000.png").unwrap(),
        rl.load_texture(&thread, "./resources/4_JUMP_001.png").unwrap(),
        rl.load_texture(&thread, "./resources/4_JUMP_002.png").unwrap(),
        rl.load_texture(&thread, "./resources/4_JUMP_003.png").unwrap(),
        rl.load_texture(&thread, "./resources/4_JUMP_004.png").unwrap(),
    ];
    let hurt1_vec = vec![
        rl.load_texture(&thread, "./resources/6_HURT_000.png").unwrap(),
        rl.load_texture(&thread, "./resources/6_HURT_001.png").unwrap(),
    ];
    let attack1_vec = vec![
        rl.load_texture(&thread, "./resources/5_ATTACK_000.png").unwrap(),
        rl.load_texture(&thread, "./resources/5_ATTACK_001.png").unwrap(),
        rl.load_texture(&thread, "./resources/5_ATTACK_002.png").unwrap(),
    ];

    // loading all the player 2 stuff
    let run2_vec = vec![
        rl.load_texture(&thread, "./resources/3_RUN_000.png").unwrap(),
        rl.load_texture(&thread, "./resources/3_RUN_001.png").unwrap(),
        rl.load_texture(&thread, "./resources/3_RUN_002.png").unwrap(),
        rl.load_texture(&thread, "./resources/3_RUN_003.png").unwrap(),
        rl.load_texture(&thread, "./resources/3_RUN_004.png").unwrap(),
    ];
    let idle2_vec = vec![
        rl.load_texture(&thread, "./resources/p2_IDLE_000.png").unwrap(),
        rl.load_texture(&thread, "./resources/p2_IDLE_001.png").unwrap(),
        rl.load_texture(&thread, "./resources/p2_IDLE_002.png").unwrap(),
        rl.load_texture(&thread, "./resources/p2_IDLE_003.png").unwrap(),
        rl.load_texture(&thread, "./resources/p2_IDLE_004.png").unwrap(),
    ];
    let jump2_vec = vec![
        rl.load_texture(&thread, "./resources/p2_JUMP_000.png").unwrap(),
        rl.load_texture(&thread, "./resources/p2_JUMP_001.png").unwrap(),
        rl.load_texture(&thread, "./resources/p2_JUMP_002.png").unwrap(),
        rl.load_texture(&thread, "./resources/p2_JUMP_003.png").unwrap(),
        rl.load_texture(&thread, "./resources/p2_JUMP_004.png").unwrap(),
    ];
    let hurt2_vec = vec![
        rl.load_texture(&thread, "./resources/p2_HURT_000.png").unwrap(),
        rl.load_texture(&thread, "./resources/p2_HURT_001.png").unwrap(),
    ];
    let attack2_vec = vec![
        rl.load_texture(&thread, "./resources/p2_ATTACK_000.png").unwrap(),
        rl.load_texture(&thread, "./resources/p2_ATTACK_002.png").unwrap(),
        rl.load_texture(&thread, "./resources/p2_ATTACK_004.png").unwrap(),
    ];

    // loading maps and ui bits
    let background_vec = vec![rl.load_texture(&thread, "./resources/background 1.png").unwrap()];
    let obstacle_vec = vec![
        rl.load_texture(&thread, "./resources/tile1.png").unwrap(),
        rl.load_texture(&thread, "./resources/tile2.png").unwrap(),
        rl.load_texture(&thread, "./resources/tile3.png").unwrap(),
    ];
    let ui_vec = vec![rl.load_texture(&thread, "./resources/heart.png").unwrap()];
    let lava_tex = rl.load_texture(&thread, "./resources/lava.png").unwrap();

    // dumping all the assets into the game state
    let mut game_data = GameData::new(
        monitor_w, monitor_h, zoom_factor, 
        run1_vec, idle1_vec, jump1_vec, hurt1_vec, attack1_vec,
        run2_vec, idle2_vec, jump2_vec, hurt2_vec, attack2_vec,
        background_vec, obstacle_vec, ui_vec, lava_tex
    );

    // starting at the menu
    let mut scenes: Vec<Box<dyn Scene>> = vec![Box::new(MenuScene)];
    let mut last_time = Instant::now();
    let mut keep_playing = true;
     
    // game loop
    while !rl.window_should_close() && keep_playing {
        let temp = Instant::now();
        let delta = (temp - last_time).as_secs_f32();
        last_time = temp;

        // handle inputs and scene swapping
        let result = scenes.last_mut().unwrap().handle_input(&mut rl, &mut game_data);
        match result {
            SceneSwitch::Push(new_scene) => scenes.push(new_scene),
            SceneSwitch::Replace(new_scene) => {
                scenes.pop();
                scenes.push(new_scene);
            },
            SceneSwitch::Quit => keep_playing = false,
            _ => ()
        }

        // update game logic
        let result = scenes.last_mut().unwrap().update(delta, &mut game_data);
        match result {
            SceneSwitch::Push(new_scene) => scenes.push(new_scene),
            SceneSwitch::Replace(new_scene) => {
                scenes.pop();
                scenes.push(new_scene);
            },
            SceneSwitch::Quit => keep_playing = false,
            _ => ()
        }

        // draw everything
        let mut d = rl.begin_drawing(&thread); 
        scenes.last().unwrap().draw(&mut d, &mut game_data);
    }
}