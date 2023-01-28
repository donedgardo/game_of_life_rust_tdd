use std::time::Duration;
use crate::game::Game;
use crate::grid::Grid;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::render::camera::RenderTarget;
use bevy::sprite::Mesh2dHandle;
use crate::node::Node;

mod node;
mod game;
mod box_boundary;
mod grid;

fn main() {
    App::new()
        .insert_resource(GameState {
            game: Game::new(),
            grid: Grid::new(1000, 10),
            status: GameStatus::Pause,
        })
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup_game)
        .add_system(my_cursor_system)
        .add_system(my_game_play_pause_system)
        .add_system(evolve_game)
        .run();
}

#[derive(PartialEq)]
enum GameStatus {
    Play,
    Pause,
}

#[derive(Resource)]
struct GameState {
    game: Game,
    status: GameStatus,
    grid: Grid,
}

#[derive(Resource)]
struct EvolutionTimer {
    timer: Timer,
}

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct Cell {
    x: i32,
    y: i32,
}

#[derive(Bundle)]
struct CellBundle {
    #[bundle]
    mesh_2d_bundle: MaterialMesh2dBundle<ColorMaterial>,
    cell: Cell,
}

impl CellBundle {
    fn new(mesh: Mesh2dHandle, material: Handle<ColorMaterial>, node: &Node, cell_size: i32) -> Self {
        Self {
            mesh_2d_bundle: MaterialMesh2dBundle {
                mesh,
                material,
                transform: Transform::from_translation(
                    Vec3::new(
                        (node.x * cell_size) as f32,
                        (node.y * cell_size) as f32, 0.)
                ),
                ..default()
            },
            cell: Cell {
                x: node.x,
                y: node.y,
            },
        }
    }
}

fn setup_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_state: Res<GameState>,
) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
    for node in game_state.grid.get_cells() {
        if game_state.game.is_node_alive(node.x, node.y) {
            let mesh = meshes.add(shape::Cube::new(game_state.grid.cell_size as f32).into()).into();
            let material = materials.add(ColorMaterial::from(Color::BLUE));
            commands.spawn(CellBundle::new(mesh, material, node, game_state.grid.cell_size));
        }
    }
    commands.insert_resource(EvolutionTimer {
        timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
    });
}

fn evolve_game(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut config: ResMut<EvolutionTimer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    cell_q: Query<Entity, With<Cell>>,
    time: Res<Time>,
) {
    if game_state.status == GameStatus::Pause { return; };
    config.timer.tick(time.delta());
    if !config.timer.finished() { return; }
    for cell in cell_q.iter() {
        let mut entity = commands.entity(cell);
        entity.despawn();
    }
    game_state.game.evolve();
    for node in &game_state.game.live_nodes {
        if node.x < -game_state.grid.radius ||
            node.x >= game_state.grid.radius ||
            node.y < -game_state.grid.radius ||
            node.y >= game_state.grid.radius { continue; };
        if game_state.game.is_node_alive(node.x, node.y) {
            let mesh = meshes.add(shape::Cube::new(game_state.grid.cell_size as f32).into()).into();
            let material = materials.add(ColorMaterial::from(Color::BLUE));
            commands.spawn(CellBundle::new(mesh, material, node, game_state.grid.cell_size));
        }
    }
}

fn my_game_play_pause_system(
    mut game_state: ResMut<GameState>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        if game_state.status == GameStatus::Pause {
            game_state.status = GameStatus::Play;
        } else {
            game_state.status = GameStatus::Pause;
        }
    }
}

fn my_cursor_system(
    windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    buttons: Res<Input<MouseButton>>,
    mut game_state: ResMut<GameState>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    cell_q: Query<(&Cell, Entity)>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }
    let world_pos_res = get_cursor_world_pos(windows, q_camera);
    if world_pos_res.is_none() { return; }
    let world_pos = world_pos_res.unwrap();
    let node = Grid::get_node_from_world_pos(world_pos.x, world_pos.y, game_state.grid.cell_size as u32);
    if game_state.game.is_node_alive(node.x, node.y) {
        for (cell, entity) in cell_q.iter() {
            if node.x == cell.x && node.y == cell.y {
                commands.entity(entity).despawn();
            }
        }
    } else {
        let mesh = meshes.add(shape::Cube::new(game_state.grid.cell_size as f32).into()).into();
        let material = materials.add(ColorMaterial::from(Color::BLUE));
        commands.spawn(CellBundle::new(mesh, material, &node, game_state.grid.cell_size));
    }
    game_state.game.toggle(&node);
}

fn get_cursor_world_pos(windows: Res<Windows>, q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>) -> Option<Vec2> {
    let (camera, camera_transform) = q_camera.single();
    let wnd = if let RenderTarget::Window(id) = camera.target {
        windows.get(id).unwrap()
    } else {
        windows.get_primary().unwrap()
    };
    let wind_pos = wnd.cursor_position();
    return match wind_pos {
        Some(screen_pos) => {
            let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
            let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
            let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
            let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
            Option::from(world_pos.truncate())
        }
        None => {
            None
        }
    };
}

