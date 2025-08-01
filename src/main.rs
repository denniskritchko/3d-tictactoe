use bevy::prelude::*;

mod game;
mod ai;
mod graphics;

use game::*;
use graphics::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "3D Tic-Tac-Toe".into(),
                resolution: (1024., 768.).into(),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<GameState>()
        .add_event::<SoundEvent>()
        .add_systems(Startup, setup_scene)
        .add_systems(Update, (
            handle_hover,
            handle_input,
            rotate_camera,
            trigger_move_animations,
            animate_moves,
            clear_animations_on_reset,
            update_cube_materials,
            check_game_over,
            ai_move_system,
            randomize_light_on_reset,
            play_sound_effects,
        ))
        .run();
} 