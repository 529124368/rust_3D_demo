#![windows_subsystem = "windows"]
use bevy::{app::ScheduleRunnerSettings, utils::Duration};
use bevy::{prelude::*, window::PresentMode};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Playing,
}

#[derive(Default)]
struct Player {
    entity: Option<Entity>,
    i: f32,
    j: f32,
    state: usize,
    old_state: usize,
}

#[derive(Default)]
struct Game {
    player: Player,
}
fn main() {
    App::new()
        .init_resource::<Game>()
        //初期状态
        .add_state(GameState::Playing)
        .insert_resource(WindowDescriptor {
            title: "rust game demo".to_string(),
            width: 1000.,
            height: 600.,
            present_mode: PresentMode::Fifo,
            ..default()
        })
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        )))
        .add_plugins(DefaultPlugins)
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0,
        })
        .add_startup_system(setup)
        .add_system(setup_scene_once_loade)
        .add_system(keyboard_animation_control)
        .add_system(move_player_by_mouse)
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(move_player))
        //.add_system(bevy::window::close_on_esc)
        .run();
}

struct Animations(Vec<Handle<AnimationClip>>);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut game: ResMut<Game>,
) {
    // reset the game state
    game.player.i = 0.0;
    game.player.j = 0.0;
    game.player.state = 1;
    game.player.old_state = 1;
    // Insert a resource with the current scene information
    commands.insert_resource(Animations(vec![
        asset_server.load("ba.gltf#Animation2"),
        asset_server.load("ba.gltf#Animation1"),
        asset_server.load("ba.gltf#Animation0"),
        asset_server.load("ba.gltf#Animation3"),
    ]));

    // Camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(1.0, 1.7, 1.5).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..default()
    });

    // Plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 500000.0 })),
        material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
        ..default()
    });

    // Light
    const HALF_SIZE: f32 = 1.0;
    commands.spawn_bundle(DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            1.0,
            -std::f32::consts::FRAC_PI_4,
        )),
        directional_light: DirectionalLight {
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..default()
            },
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });

    //player
    game.player.entity = Some(
        commands
            // .spawn_bundle(SceneBundle {
            //     scene: asset_server.load("ba.gltf#Scene0"),
            //     transform: Transform::default(),
            //     ..default()
            // })
            // .id(),
            .spawn_bundle(TransformBundle::from(Transform {
                //rotation: Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2),
                ..default()
            }))
            .with_children(|cell| {
                cell.spawn_scene(asset_server.load("ba.gltf#Scene0"));
            })
            .id(),
    );
}

// Once the scene is loaded, start the animation
fn setup_scene_once_loade(
    animations: Res<Animations>,
    mut player: Query<&mut AnimationPlayer>,
    mut done: Local<bool>,
    game: Res<Game>,
) {
    if !*done {
        if let Ok(mut player) = player.get_single_mut() {
            player
                .play(animations.0[game.player.state].clone_weak())
                .repeat();
            *done = true;
        }
    }
}

fn keyboard_animation_control(
    mut animation_player: Query<&mut AnimationPlayer>,
    animations: Res<Animations>,
    mut game: ResMut<Game>,
) {
    if game.player.old_state != game.player.state {
        if let Ok(mut player) = animation_player.get_single_mut() {
            player
                .play(animations.0[game.player.state].clone_weak())
                .repeat();
            game.player.old_state = game.player.state;
        }
    }
}

// control the game character
fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut game: ResMut<Game>,
    mut transforms: Query<&mut Transform>,
    time: Res<Time>,
) {
    let speed = 0.5 * time.delta_seconds();
    let mut moved = false;

    if keyboard_input.pressed(KeyCode::Up) {
        game.player.i -= speed;
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        game.player.i += speed;
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        game.player.j -= speed;
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::Left) {
        game.player.j += speed;
        moved = true;
    }
    // move on the board
    if moved {
        game.player.state = 3;
        *transforms.get_mut(game.player.entity.unwrap()).unwrap() = Transform {
            translation: Vec3::new(game.player.i as f32, 0.0, game.player.j as f32),
            ..default()
        };
    } else if game.player.state != 2 {
        game.player.state = 1;
    }
    //}
}

// control the game character
fn move_player_by_mouse(
    mouse_input: Res<Input<MouseButton>>,
    mut game: ResMut<Game>,
    windows: Res<Windows>,
    // mut transforms: Query<&mut Transform>,
    // time: Res<Time>,
) {
    // let speed = 0.5 * time.delta_seconds();
    // let mut moved = false;
    if mouse_input.pressed(MouseButton::Left) {
        let window = windows.get_primary().unwrap();

        if let Some(_position) = window.cursor_position() {
            // cursor is inside the window, position given
            println!("{}:{}", _position.x, _position.y);
        } else {
            // cursor is not inside the window
            println!("不在窗口内");
        }
    }
    if mouse_input.pressed(MouseButton::Right) {
        game.player.state = 2;
    }
}
