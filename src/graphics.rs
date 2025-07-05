use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use crate::game::{GameState, Player, CellState};

#[derive(Component)]
pub struct CubeMarker {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

#[derive(Component)]
pub struct CameraController {
    pub sensitivity: f32,
    pub distance: f32,
    pub yaw: f32,
    pub pitch: f32,
}

#[derive(Resource)]
pub struct CubeMaterials {
    pub empty: Handle<StandardMaterial>,
    pub human: Handle<StandardMaterial>,
    pub ai: Handle<StandardMaterial>,
    pub selected: Handle<StandardMaterial>,
}

#[derive(Resource)]
pub struct GameMeshes {
    pub cube: Handle<Mesh>,
}

pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create materials
    let cube_materials = CubeMaterials {
        empty: materials.add(StandardMaterial {
            base_color: Color::srgba(0.3, 0.3, 0.3, 0.5),
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
        human: materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.7, 0.2),
            ..default()
        }),
        ai: materials.add(StandardMaterial {
            base_color: Color::srgb(0.7, 0.2, 0.2),
            ..default()
        }),
        selected: materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.8, 0.2),
            ..default()
        }),
    };

    // Create mesh
    let cube_mesh = meshes.add(Mesh::from(Cuboid::new(0.8, 0.8, 0.8)));
    
    let game_meshes = GameMeshes {
        cube: cube_mesh.clone(),
    };

    // Create the 3x3x3 grid of cubes
    for x in 0..3 {
        for y in 0..3 {
            for z in 0..3 {
                commands.spawn((
                    PbrBundle {
                        mesh: cube_mesh.clone(),
                        material: cube_materials.empty.clone(),
                        transform: Transform::from_xyz(
                            (x as f32 - 1.0) * 2.0,
                            (y as f32 - 1.0) * 2.0,
                            (z as f32 - 1.0) * 2.0,
                        ),
                        ..default()
                    },
                    CubeMarker { x, y, z },
                ));
            }
        }
    }

    // Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        CameraController {
            sensitivity: 0.5,
            distance: 10.0,
            yaw: 0.0,
            pitch: 0.0,
        },
    ));

    // Light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 3000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 300.0,
    });

    // UI Text
    commands.spawn(
        TextBundle::from_section(
            "3D Tic-Tac-Toe\nClick on cubes to play!\nWASD + Mouse to rotate camera\nR to reset game",
            TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
    );

    // Game status text
    commands.spawn((
        TextBundle::from_section(
            "Your turn!",
            TextStyle {
                font_size: 24.0,
                color: Color::srgb(0.2, 0.7, 0.2),
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        GameStatusText,
    ));

    commands.insert_resource(cube_materials);
    commands.insert_resource(game_meshes);
}

#[derive(Component)]
pub struct GameStatusText;

pub fn handle_input(
    buttons: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    cubes_query: Query<(&GlobalTransform, &CubeMarker)>,
    mut game_state: ResMut<GameState>,
) {
    if keyboard.just_pressed(KeyCode::KeyR) {
        game_state.reset();
        return;
    }

    if game_state.game_over || game_state.current_player != Player::Human {
        return;
    }

    if buttons.just_pressed(MouseButton::Left) {
        let window = windows.single();
        if let Some(cursor_position) = window.cursor_position() {
            let (camera, camera_transform) = camera_query.single();
            
            // Convert screen coordinates to world ray
            if let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) {
                let mut closest_cube = None;
                let mut closest_distance = f32::INFINITY;

                // Check intersection with all cubes
                for (cube_transform, cube_marker) in cubes_query.iter() {
                    let cube_pos = cube_transform.translation();
                    let distance_to_cube = ray.origin.distance(cube_pos);
                    
                    // Simple distance-based selection (could be improved with proper ray-box intersection)
                    if distance_to_cube < closest_distance {
                        closest_distance = distance_to_cube;
                        closest_cube = Some((cube_marker.x, cube_marker.y, cube_marker.z));
                    }
                }

                if let Some((x, y, z)) = closest_cube {
                    game_state.make_move(x, y, z);
                }
            }
        }
    }
}

pub fn rotate_camera(
    mut motion_events: EventReader<MouseMotion>,
    mut camera_query: Query<(&mut Transform, &mut CameraController)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    if let Ok((mut transform, mut controller)) = camera_query.get_single_mut() {
        let mut rotation_delta = Vec2::ZERO;

        // Mouse look (when right mouse button is held)
        if buttons.pressed(MouseButton::Right) {
            for event in motion_events.read() {
                rotation_delta += event.delta;
            }
        }

        // Keyboard rotation
        let rotation_speed = 2.0;
        if keyboard.pressed(KeyCode::KeyA) {
            rotation_delta.x -= rotation_speed * time.delta_seconds() * 100.0;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            rotation_delta.x += rotation_speed * time.delta_seconds() * 100.0;
        }
        if keyboard.pressed(KeyCode::KeyW) {
            rotation_delta.y -= rotation_speed * time.delta_seconds() * 100.0;
        }
        if keyboard.pressed(KeyCode::KeyS) {
            rotation_delta.y += rotation_speed * time.delta_seconds() * 100.0;
        }

        if rotation_delta.length() > 0.0 {
            controller.yaw -= rotation_delta.x * controller.sensitivity * time.delta_seconds();
            controller.pitch -= rotation_delta.y * controller.sensitivity * time.delta_seconds();
            controller.pitch = controller.pitch.clamp(-1.5, 1.5);

            // Update camera position based on spherical coordinates
            let x = controller.distance * controller.yaw.cos() * controller.pitch.cos();
            let y = controller.distance * controller.pitch.sin();
            let z = controller.distance * controller.yaw.sin() * controller.pitch.cos();

            transform.translation = Vec3::new(x, y, z);
            transform.look_at(Vec3::ZERO, Vec3::Y);
        }

        // Zoom with scroll wheel
        // (This would require scroll events, keeping simple for now)
    }
}

pub fn update_cube_materials(
    mut cube_query: Query<(&mut Handle<StandardMaterial>, &CubeMarker)>,
    game_state: Res<GameState>,
    materials: Res<CubeMaterials>,
) {
    if !game_state.is_changed() {
        return;
    }

    for (mut material, cube_marker) in cube_query.iter_mut() {
        let cell_state = game_state.board[cube_marker.x][cube_marker.y][cube_marker.z];
        
        *material = match cell_state {
            CellState::Empty => {
                if Some((cube_marker.x, cube_marker.y, cube_marker.z)) == game_state.selected_cube {
                    materials.selected.clone()
                } else {
                    materials.empty.clone()
                }
            }
            CellState::Human => materials.human.clone(),
            CellState::AI => materials.ai.clone(),
        };
    }
}

pub fn check_game_over(
    game_state: Res<GameState>,
    mut status_text_query: Query<&mut Text, With<GameStatusText>>,
) {
    if !game_state.is_changed() {
        return;
    }

    if let Ok(mut text) = status_text_query.get_single_mut() {
        if game_state.game_over {
            match game_state.winner {
                Some(Player::Human) => {
                    text.sections[0].value = "You win! Press R to restart".to_string();
                    text.sections[0].style.color = Color::srgb(0.2, 0.7, 0.2);
                }
                Some(Player::AI) => {
                    text.sections[0].value = "AI wins! Press R to restart".to_string();
                    text.sections[0].style.color = Color::srgb(0.7, 0.2, 0.2);
                }
                None => {
                    text.sections[0].value = "It's a draw! Press R to restart".to_string();
                    text.sections[0].style.color = Color::srgb(0.7, 0.7, 0.2);
                }
            }
        } else {
            match game_state.current_player {
                Player::Human => {
                    text.sections[0].value = "Your turn!".to_string();
                    text.sections[0].style.color = Color::srgb(0.2, 0.7, 0.2);
                }
                Player::AI => {
                    text.sections[0].value = "AI thinking...".to_string();
                    text.sections[0].style.color = Color::srgb(0.7, 0.2, 0.2);
                }
            }
        }
    }
}

pub fn ai_move_system(
    mut game_state: ResMut<GameState>,
    time: Res<Time>,
) {
    if game_state.game_over || game_state.current_player != Player::AI {
        return;
    }

    // Add a small delay to make AI moves visible
    static mut AI_TIMER: f32 = 0.0;
    unsafe {
        AI_TIMER += time.delta_seconds();
        if AI_TIMER < 1.0 {
            return;
        }
        AI_TIMER = 0.0;
    }

    if let Some((x, y, z)) = game_state.ai.get_best_move(&game_state) {
        game_state.make_move(x, y, z);
    }
} 