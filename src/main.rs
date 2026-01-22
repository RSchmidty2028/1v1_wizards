use raylib::prelude::*;

use raylib_framework_testing::game_data::GameData;
use raylib_framework_testing::menu_scene::MenuScene;
use raylib_framework_testing::scenes::{Scene, SceneSwitch};


use std::fs::OpenOptions;
use std::sync::Arc;
use std::time::Instant;


use tracing::{debug, info, warn};
use tracing_subscriber::prelude::*;


fn main() {
    let scale = 4; 
    let base_width: i32 = 576;
    let base_height: i32 = 324;
    let width = scale * base_width;
    let height = scale * base_height;
    let (mut rl, thread) = raylib::init()
        .size(width, height)
        .title("Raylib Framework Demo")
        .build();



    let stdout_log = tracing_subscriber::fmt::layer().pretty();

    let file = OpenOptions::new()
    .append(true)
    .create(true)
    .open("debug.log");

    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Error {:?}", error),
    };

    let debug_log = tracing_subscriber::fmt::layer()
    .json()
    .with_writer(Arc::new(file));

    tracing_subscriber::Registry::default()
    .with(stdout_log)
    .with(debug_log)
    .init();





    // load in all the player1_running_textures

    let run_tex_1: Texture2D = rl.load_texture(&thread, "./resources/p1_RUN_000.png").unwrap();
    let run_tex_2: Texture2D = rl.load_texture(&thread, "./resources/p1_RUN_001.png").unwrap();
    let run_tex_3: Texture2D = rl.load_texture(&thread, "./resources/p1_RUN_002.png").unwrap();
    let run_tex_4: Texture2D = rl.load_texture(&thread, "./resources/p1_RUN_003.png").unwrap();
    let run_tex_5: Texture2D = rl.load_texture(&thread, "./resources/p1_RUN_004.png").unwrap();
    
    let run1_vec: Vec<Texture2D> = vec![run_tex_1, run_tex_2, run_tex_3, run_tex_4, run_tex_5];

    // load in all the idle1_textures
    let idle_tex_1: Texture2D = rl.load_texture(&thread, "./resources/1_IDLE_000.png").unwrap();
    let idle_tex_2: Texture2D = rl.load_texture(&thread, "./resources/1_IDLE_001.png").unwrap();
    let idle_tex_3: Texture2D = rl.load_texture(&thread, "./resources/1_IDLE_002.png").unwrap();
    let idle_tex_4: Texture2D = rl.load_texture(&thread, "./resources/1_IDLE_003.png").unwrap();
    let idle_tex_5: Texture2D = rl.load_texture(&thread, "./resources/1_IDLE_004.png").unwrap();

    let idle1_vec: Vec<Texture2D> = vec![idle_tex_1,idle_tex_2,idle_tex_3,idle_tex_4,idle_tex_5];

    //load in all the jump1_tex
    let jump1_tex_1: Texture2D = rl.load_texture(&thread, "./resources/4_JUMP_000.png").unwrap();
    let jump1_tex_2: Texture2D = rl.load_texture(&thread, "./resources/4_JUMP_001.png").unwrap();
    let jump1_tex_3: Texture2D = rl.load_texture(&thread, "./resources/4_JUMP_002.png").unwrap();
    let jump1_tex_4: Texture2D = rl.load_texture(&thread, "./resources/4_JUMP_003.png").unwrap();
    let jump1_tex_5: Texture2D = rl.load_texture(&thread, "./resources/4_JUMP_004.png").unwrap();

    let jump1_vec: Vec<Texture2D> = vec![jump1_tex_1,jump1_tex_2,jump1_tex_3,jump1_tex_4,jump1_tex_5];

    //load in all the hurt1_tex
    let hurt1_tex_1: Texture2D = rl.load_texture(&thread, "./resources/6_HURT_000.png").unwrap();
    let hurt1_tex_2: Texture2D = rl.load_texture(&thread, "./resources/6_HURT_001.png").unwrap();
    let hurt1_tex_3: Texture2D = rl.load_texture(&thread, "./resources/6_HURT_002.png").unwrap();
    let hurt1_tex_4: Texture2D = rl.load_texture(&thread, "./resources/6_HURT_003.png").unwrap();
    let hurt1_tex_5: Texture2D = rl.load_texture(&thread, "./resources/6_HURT_004.png").unwrap();

    let hurt1_vec: Vec<Texture2D> = vec![hurt1_tex_1,hurt1_tex_2,hurt1_tex_3,hurt1_tex_4,hurt1_tex_5];


    //load in all the attack1_tex
    let attack1_tex_1: Texture2D = rl.load_texture(&thread, "./resources/5_ATTACK_000.png").unwrap();
    let attack1_tex_2: Texture2D = rl.load_texture(&thread, "./resources/5_ATTACK_001.png").unwrap();
    let attack1_tex_3: Texture2D = rl.load_texture(&thread, "./resources/5_ATTACK_002.png").unwrap();
    let attack1_tex_4: Texture2D = rl.load_texture(&thread, "./resources/5_ATTACK_003.png").unwrap();
    let attack1_tex_5: Texture2D = rl.load_texture(&thread, "./resources/5_ATTACK_004.png").unwrap();
    


    let attack1_vec: Vec<Texture2D> = vec![attack1_tex_1,attack1_tex_2,attack1_tex_3,attack1_tex_4,attack1_tex_5];


    //load in all the jump2_tex
    let jump2_tex_1: Texture2D = rl.load_texture(&thread, "./resources/p2_JUMP_000.png").unwrap();
    let jump2_tex_2: Texture2D = rl.load_texture(&thread, "./resources/p2_JUMP_001.png").unwrap();
    let jump2_tex_3: Texture2D = rl.load_texture(&thread, "./resources/p2_JUMP_002.png").unwrap();
    let jump2_tex_4: Texture2D = rl.load_texture(&thread, "./resources/p2_JUMP_003.png").unwrap();
    let jump2_tex_5: Texture2D = rl.load_texture(&thread, "./resources/p2_JUMP_004.png").unwrap();

    let jump2_vec: Vec<Texture2D> = vec![jump2_tex_1,jump2_tex_2,jump2_tex_3,jump2_tex_4,jump2_tex_5];

    //load in all the hurt2_tex
    let hurt2_tex_1: Texture2D = rl.load_texture(&thread, "./resources/p2_HURT_000.png").unwrap();
    let hurt2_tex_2: Texture2D = rl.load_texture(&thread, "./resources/p2_HURT_001.png").unwrap();
    let hurt2_tex_3: Texture2D = rl.load_texture(&thread, "./resources/p2_HURT_002.png").unwrap();
    let hurt2_tex_4: Texture2D = rl.load_texture(&thread, "./resources/p2_HURT_003.png").unwrap();
    let hurt2_tex_5: Texture2D = rl.load_texture(&thread, "./resources/p2_HURT_004.png").unwrap();

    let hurt2_vec: Vec<Texture2D> = vec![hurt2_tex_1,hurt2_tex_2,hurt2_tex_3,hurt2_tex_4,hurt2_tex_5];

    //load in all the attack2_tex
    let attack2_tex_1: Texture2D = rl.load_texture(&thread, "./resources/p2_ATTACK_000.png").unwrap();
    let attack2_tex_2: Texture2D = rl.load_texture(&thread, "./resources/p2_ATTACK_002.png").unwrap();
    let attack2_tex_3: Texture2D = rl.load_texture(&thread, "./resources/p2_ATTACK_004.png").unwrap();
    let attack2_tex_4: Texture2D = rl.load_texture(&thread, "./resources/p2_ATTACK_005.png").unwrap();
    let attack2_tex_5: Texture2D = rl.load_texture(&thread, "./resources/p2_ATTACK_006.png").unwrap();


    let attack2_vec: Vec<Texture2D> = vec![attack2_tex_1,attack2_tex_2,attack2_tex_3,attack2_tex_4,attack2_tex_5];

    

    // load in all the idle2_tex

    let idle2_tex_1: Texture2D = rl.load_texture(&thread, "./resources/p2_IDLE_000.png").unwrap();
    let idle2_tex_2: Texture2D = rl.load_texture(&thread, "./resources/p2_IDLE_001.png").unwrap();
    let idle2_tex_3: Texture2D = rl.load_texture(&thread, "./resources/p2_IDLE_002.png").unwrap();
    let idle2_tex_4: Texture2D = rl.load_texture(&thread, "./resources/p2_IDLE_003.png").unwrap();
    let idle2_tex_5: Texture2D = rl.load_texture(&thread, "./resources/p2_IDLE_000.png").unwrap();

    let idle2_vec: Vec<Texture2D> = vec![idle2_tex_1,idle2_tex_2,idle2_tex_3, idle2_tex_4, idle2_tex_5];


    // load in background textures
    let background_tex_1: Texture2D = rl.load_texture(&thread, "./resources/background 1.png").unwrap();
    let background_vec: Vec<Texture2D> = vec![background_tex_1];

    //load in player_2 running_textures

    let run_tex: Texture2D = rl.load_texture(&thread, "./resources/3_RUN_000.png").unwrap();
    let run_tex2: Texture2D = rl.load_texture(&thread, "./resources/3_RUN_001.png").unwrap();
    let run_tex3: Texture2D = rl.load_texture(&thread, "./resources/3_RUN_002.png").unwrap();
    let run_tex4: Texture2D = rl.load_texture(&thread, "./resources/3_RUN_003.png").unwrap();
    let run_tex5: Texture2D = rl.load_texture(&thread, "./resources/3_RUN_004.png").unwrap();

    let p2_run_vec: Vec<Texture2D> = vec![run_tex,run_tex2,run_tex3,run_tex4,run_tex5];
    
    // load in platforms
    let obstacle_1_tex: Texture2D = rl.load_texture(&thread, "./resources/tile1.png").unwrap();
    let obstacle_2_tex: Texture2D = rl.load_texture(&thread, "./resources/tile2.png").unwrap();
    let obstacle_3_tex: Texture2D = rl.load_texture(&thread, "./resources/tile3.png").unwrap();

    let obstacle_vec: Vec<Texture2D> = vec![obstacle_1_tex, obstacle_2_tex, obstacle_3_tex];

     // create the game data. This prepresents data associated with the human player.
    let mut game_data = GameData::new(
        width, 
        height,
         run1_vec,
         p2_run_vec,
         idle1_vec,
        idle2_vec,
         hurt1_vec,
         hurt2_vec,
         jump1_vec,
         jump2_vec,
         attack1_vec,
         attack2_vec,
         background_vec,
         obstacle_vec
        );

    // store scenes in a Vec. Box<dyn Scene> means a pointer to any type that implements the Scene trait.
    let mut scenes: Vec<Box<dyn Scene>> = vec![Box::new(MenuScene)];

    // A variable for the time to calculate update steps in the game. Use for physics and animation.
    let mut last_time = Instant::now();
    let mut keep_playing = true;
     
    // the main game / draw loop 
    while !rl.window_should_close() && keep_playing {
        // update timing.
        let temp = Instant::now();
        let delta = (temp - last_time).as_secs_f32();
        last_time = temp;

        // Below is a general framework for working with a game. Look at the Scene trait for some more information
        //
        // 1) preprocessing / setup. This was named on_enter. Called when the start is first started.
        // --- Main loop ---
        // 2) handle user input. Get the player's intent.
        // 3) update the world / do simulation processing.
        // 4) draw the game elements
        // --- End of main loop ---
        // 5) postprocessing / clean up. This was named on_exit. Do any necessary clean up

        let mut the_scene = scenes.last_mut().unwrap();

        // handle user input. This 
        //let result = scenes.last_mut().unwrap().handle_input(&mut rl, &mut game_data);
        let result = the_scene.handle_input(&mut rl, &mut game_data);
        match result {
            SceneSwitch::Push(new_scene) => {
                println!("got scene");
                scenes.push(new_scene);
            },
            SceneSwitch::Quit => keep_playing = false,
            _ => ()
        }


        let mut the_scene = scenes.last_mut().unwrap();
        //let result = scenes.last_mut().unwrap().update(delta, &mut game_data);
        let result = the_scene.update(delta, &mut game_data);
        match result {
            SceneSwitch::Push(new_scene) => {
                println!("got scene");
                scenes.push(new_scene);
            },
            SceneSwitch::Quit => keep_playing = false,
            _ => ()
        }


        // Draw
        let mut the_scene = scenes.last().unwrap();
        let mut d = rl.begin_drawing(&thread); 
        the_scene.draw(&mut d, &mut  game_data);

    }
}