use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use rand::Rng;
use crate::game::{GameState, Player, CellState};

// Helper function for ray-box intersection
fn ray_box_intersection(ray_origin: Vec3, ray_dir: Vec3, box_min: Vec3, box_max: Vec3) -> Option<f32> {
    let mut tmin = (box_min.x - ray_origin.x) / ray_dir.x;
    let mut tmax = (box_max.x - ray_origin.x) / ray_dir.x;
    
    if tmin > tmax {
        std::mem::swap(&mut tmin, &mut tmax);
    }
    
    let mut tymin = (box_min.y - ray_origin.y) / ray_dir.y;
    let mut tymax = (box_max.y - ray_origin.y) / ray_dir.y;
    
    if tymin > tymax {
        std::mem::swap(&mut tymin, &mut tymax);
    }
    
    if tmin > tymax || tymin > tmax {
        return None;
    }
    
    if tymin > tmin {
        tmin = tymin;
    }
    
    if tymax < tmax {
        tmax = tymax;
    }
    
    let mut tzmin = (box_min.z - ray_origin.z) / ray_dir.z;
    let mut tzmax = (box_max.z - ray_origin.z) / ray_dir.z;
    
    if tzmin > tzmax {
        std::mem::swap(&mut tzmin, &mut tzmax);
    }
    
    if tmin > tzmax || tzmin > tmax {
        return None;
    }
    
    if tzmin > tmin {
        tmin = tzmin;
    }
    
    if tzmax < tmax {
        tmax = tzmax;
    }
    
    // Return the closest intersection point
    if tmin > 0.0 {
        Some(tmin)
    } else if tmax > 0.0 {
        Some(tmax)
    } else {
        None
    }
}

// Generate a random light position that provides good illumination
fn generate_random_light_position() -> Vec3 {
    let mut rng = rand::thread_rng();
    
    // Generate random spherical coordinates around the cube
    let distance: f32 = rng.gen_range(6.0..12.0); // Distance from center
    let azimuth: f32 = rng.gen_range(0.0..std::f32::consts::TAU); // Rotation around Y axis
    let elevation: f32 = rng.gen_range(0.3..1.2); // Angle from horizontal (avoid too low or too high)
    
    // Convert spherical to cartesian coordinates
    let x = distance * elevation.cos() * azimuth.cos();
    let y = distance * elevation.sin() + rng.gen_range(2.0..6.0) as f32; // Add some height bias
    let z = distance * elevation.cos() * azimuth.sin();
    
    Vec3::new(x, y, z)
}

// Generate a random light color with slight warm/cool variations
fn generate_random_light_color() -> Color {
    let mut rng = rand::thread_rng();
    
    // Create subtle color variations - mostly white but with slight tints
    let base_intensity: f32 = 0.95;
    let variation: f32 = 0.1;
    
    let r = (base_intensity + rng.gen_range(-variation..variation)).clamp(0.0, 1.0);
    let g = (base_intensity + rng.gen_range(-variation..variation)).clamp(0.0, 1.0);
    let b = (base_intensity + rng.gen_range(-variation..variation)).clamp(0.0, 1.0);
    
    Color::srgb(r, g, b)
}

#[derive(Component)]
pub struct CubeMarker {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

#[derive(Component)]
pub struct HoveredCube;

#[derive(Component)]
pub struct GameLight;

#[derive(Component)]
pub struct MoveAnimation {
    pub timer: f32,
    pub duration: f32,
    pub initial_scale: f32,
    pub target_scale: f32,
    pub rotation_speed: f32,
}

impl MoveAnimation {
    pub fn new() -> Self {
        Self {
            timer: 0.0,
            duration: 0.5, // Animation duration in seconds
            initial_scale: 0.1,
            target_scale: 1.0,
            rotation_speed: 8.0, // Rotations per second
        }
    }
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
    pub hovered: Handle<StandardMaterial>,
}

#[derive(Resource)]
pub struct GameMeshes {
    pub cube: Handle<Mesh>,
}

#[derive(Event)]
pub enum SoundEvent {
    MovePlace,
    Hover,
    Win,
    Lose,
    Reset,
}

#[derive(Resource)]
pub struct GameSounds {
    pub enabled: bool,
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
        hovered: materials.add(StandardMaterial {
            base_color: Color::srgba(0.6, 0.6, 0.6, 0.8),
            alpha_mode: AlphaMode::Blend,
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

    // Random light position and color for variety
    let light_position = generate_random_light_position();
    let light_color = generate_random_light_color();
    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                color: light_color,
                illuminance: 3000.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_translation(light_position).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        GameLight,
    ));

    // Ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 300.0,
    });

    // UI Text
    commands.spawn(
        TextBundle::from_section(
            "3D Tic-Tac-Toe\nHover over cubes to highlight them\nClick highlighted cubes to play!\nWASD + Mouse to rotate camera\nR to reset game + randomize lighting",
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
    
    // Initialize sound system
    let game_sounds = GameSounds {
        enabled: true,
    };
    commands.insert_resource(game_sounds);
}

#[derive(Component)]
pub struct GameStatusText;

pub fn handle_hover(
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    cubes_query: Query<(Entity, &GlobalTransform, &CubeMarker), Without<HoveredCube>>,
    hovered_cubes: Query<Entity, With<HoveredCube>>,
    mut commands: Commands,
    game_state: Res<GameState>,
    mut sound_events: EventWriter<SoundEvent>,
) {
    if game_state.game_over || game_state.current_player != Player::Human {
        // Remove all hover highlights when it's not the player's turn
        for entity in hovered_cubes.iter() {
            commands.entity(entity).remove::<HoveredCube>();
        }
        return;
    }

    let window = windows.single();
    if let Some(cursor_position) = window.cursor_position() {
        let (camera, camera_transform) = camera_query.single();
        
        // Convert screen coordinates to world ray
        if let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) {
            let ray_origin = ray.origin;
            let ray_dir = *ray.direction;
            
            let mut closest_cube = None;
            let mut closest_distance = f32::INFINITY;
            
            // Check intersection with all cubes
            for (entity, cube_transform, cube_marker) in cubes_query.iter() {
                // Only check empty cubes
                if game_state.board[cube_marker.x][cube_marker.y][cube_marker.z] != CellState::Empty {
                    continue;
                }
                
                let cube_pos = cube_transform.translation();
                let cube_size = 0.4; // Half the cube size (0.8 / 2)
                let box_min = cube_pos - Vec3::splat(cube_size);
                let box_max = cube_pos + Vec3::splat(cube_size);
                
                if let Some(distance) = ray_box_intersection(ray_origin, ray_dir, box_min, box_max) {
                    if distance < closest_distance {
                        closest_distance = distance;
                        closest_cube = Some(entity);
                    }
                }
            }
            
            // Remove hover from all cubes
            for entity in hovered_cubes.iter() {
                commands.entity(entity).remove::<HoveredCube>();
            }
            
            // Add hover to the closest cube
            if let Some(entity) = closest_cube {
                commands.entity(entity).insert(HoveredCube);
                
                // Play hover sound (only if no cube was previously hovered)
                if hovered_cubes.is_empty() {
                    sound_events.send(SoundEvent::Hover);
                }
            }
        }
    } else {
        // Remove all hover highlights when cursor is not over the window
        for entity in hovered_cubes.iter() {
            commands.entity(entity).remove::<HoveredCube>();
        }
    }
}

pub fn handle_input(
    buttons: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    hovered_cubes: Query<&CubeMarker, With<HoveredCube>>,
    mut game_state: ResMut<GameState>,
    mut sound_events: EventWriter<SoundEvent>,
) {
    if keyboard.just_pressed(KeyCode::KeyR) {
        game_state.reset();
        sound_events.send(SoundEvent::Reset);
        return;
    }

    if game_state.game_over || game_state.current_player != Player::Human {
        return;
    }

    if buttons.just_pressed(MouseButton::Left) {
        // Only allow selection of hovered cubes for accurate hit detection
        for cube_marker in hovered_cubes.iter() {
            // Make the move on the hovered cube
            game_state.make_move(cube_marker.x, cube_marker.y, cube_marker.z);
            break; // Only one cube can be hovered at a time
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
    }
}

pub fn trigger_move_animations(
    mut commands: Commands,
    mut cube_query: Query<(Entity, &mut Transform, &CubeMarker), Without<MoveAnimation>>,
    game_state: Res<GameState>,
    mut sound_events: EventWriter<SoundEvent>,
) {
    if !game_state.is_changed() {
        return;
    }
    
    // Check all cubes for newly placed pieces
    for (entity, mut transform, cube_marker) in cube_query.iter_mut() {
        let cell_state = game_state.board[cube_marker.x][cube_marker.y][cube_marker.z];
        
        // If this cube was just placed (not empty and game state changed), start animation
        if cell_state != CellState::Empty {
            // Check if this cube was the last move made
            if let Some(last_move) = game_state.last_move {
                if (cube_marker.x, cube_marker.y, cube_marker.z) == last_move {
                    // Start animation from small scale
                    transform.scale = Vec3::splat(0.1);
                    transform.rotation = Quat::IDENTITY;
                    
                    // Add animation component
                    commands.entity(entity).insert(MoveAnimation::new());
                    
                    // Play move sound
                    sound_events.send(SoundEvent::MovePlace);
                }
            }
        }
    }
}

pub fn update_cube_materials(
    mut cube_query: Query<(&mut Handle<StandardMaterial>, &CubeMarker, Option<&HoveredCube>, Option<&MoveAnimation>)>,
    game_state: Res<GameState>,
    materials: Res<CubeMaterials>,
) {
    for (mut material, cube_marker, hovered, _animating) in cube_query.iter_mut() {
        let cell_state = game_state.board[cube_marker.x][cube_marker.y][cube_marker.z];
        
        *material = match cell_state {
            CellState::Empty => {
                if hovered.is_some() && game_state.current_player == Player::Human && !game_state.game_over {
                    materials.hovered.clone()
                } else if Some((cube_marker.x, cube_marker.y, cube_marker.z)) == game_state.selected_cube {
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
    mut sound_events: EventWriter<SoundEvent>,
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
                    sound_events.send(SoundEvent::Win);
                }
                Some(Player::AI) => {
                    text.sections[0].value = "AI wins! Press R to restart".to_string();
                    text.sections[0].style.color = Color::srgb(0.7, 0.2, 0.2);
                    sound_events.send(SoundEvent::Lose);
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
                    text.sections[0].value = "Smart AI calculating...".to_string();
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

    // AI delay
    static mut AI_TIMER: f32 = 0.0;
    unsafe {
        AI_TIMER += time.delta_seconds();
        if AI_TIMER < 1.5 {
            return;
        }
        AI_TIMER = 0.0;
    }

    if let Some((x, y, z)) = game_state.ai.get_best_move(&game_state) {
        game_state.make_move(x, y, z);
    }
}

pub fn animate_moves(
    mut commands: Commands,
    time: Res<Time>,
    mut cube_query: Query<(Entity, &mut Transform, &mut MoveAnimation, &CubeMarker)>,
) {
    for (entity, mut transform, mut animation, _cube_marker) in cube_query.iter_mut() {
        animation.timer += time.delta_seconds();
        
        if animation.timer >= animation.duration {
            // Animation complete - set final scale and rotation, then remove animation component
            transform.scale = Vec3::splat(animation.target_scale);
            transform.rotation = Quat::IDENTITY; // Return to original orientation
            commands.entity(entity).remove::<MoveAnimation>();
        } else {
            // Calculate animation progress (0.0 to 1.0)
            let progress = animation.timer / animation.duration;
            
            // Smooth easing function (ease-out cubic)
            let eased_progress = 1.0 - (1.0 - progress).powi(3);
            
            // Interpolate scale
            let current_scale = animation.initial_scale + 
                (animation.target_scale - animation.initial_scale) * eased_progress;
            transform.scale = Vec3::splat(current_scale);
            
            // Spin during animation but end at original position
            // Use a function that spins multiple times but always ends at 0
            let total_spins = 2.0; // Exactly 2 full rotations
            let spin_factor = 1.0 - eased_progress; // Decreases from 1 to 0
            let rotation_amount = progress * total_spins * std::f32::consts::TAU * spin_factor;
            
            transform.rotation = Quat::from_rotation_y(rotation_amount);
        }
    }
}

pub fn clear_animations_on_reset(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut cube_query: Query<(Entity, &mut Transform, &CubeMarker), With<MoveAnimation>>,
) {
    if keyboard.just_pressed(KeyCode::KeyR) {
        // Clear all animations and reset transforms
        for (entity, mut transform, _) in cube_query.iter_mut() {
            transform.scale = Vec3::ONE;
            transform.rotation = Quat::IDENTITY;
            commands.entity(entity).remove::<MoveAnimation>();
        }
    }
}

pub fn play_sound_effects(
    mut sound_events: EventReader<SoundEvent>,
    sounds: Res<GameSounds>,
) {
    if !sounds.enabled {
        return;
    }
    
    for event in sound_events.read() {
        match event {
            SoundEvent::MovePlace => {
                // Play a pleasant "place" sound (mid-high frequency)
                info!("🔊 Playing move place sound");
                // In a real implementation, you'd load and play an actual audio file
                // For now, we'll just log the sound event
            }
            SoundEvent::Hover => {
                // Play a subtle hover sound (high frequency, quiet)
                info!("🔊 Playing hover sound");
            }
            SoundEvent::Win => {
                // Play a victory sound (ascending notes)
                info!("🎉 Playing win sound");
            }
            SoundEvent::Lose => {
                // Play a defeat sound (descending notes)
                info!("😞 Playing lose sound");
            }
            SoundEvent::Reset => {
                // Play a reset sound (neutral beep)
                info!("🔄 Playing reset sound");
            }
        }
    }
}

pub fn randomize_light_on_reset(
    _game_state: Res<GameState>,
    mut light_query: Query<(&mut Transform, &mut DirectionalLight), With<GameLight>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    // Check if the game was just reset
    if keyboard.just_pressed(KeyCode::KeyR) {
        // Randomize light position and color
        let new_position = generate_random_light_position();
        let new_color = generate_random_light_color();
        
        for (mut light_transform, mut directional_light) in light_query.iter_mut() {
            light_transform.translation = new_position;
            light_transform.look_at(Vec3::ZERO, Vec3::Y);
            directional_light.color = new_color;
        }
        
        info!("Light randomized - Position: {:?}, Color: {:?}", new_position, new_color);
    }
} 