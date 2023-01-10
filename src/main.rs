use std::time::Duration;
use crate::game::Game;
use crate::grid::Grid;
use crate::node::Node;
use bevy::{diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, prelude::*, sprite::MaterialMesh2dBundle};
use bevy::render::camera::RenderTarget;

mod node;
mod game;
mod box_boundary;
mod grid;

fn main() {
    App::new()
        .insert_resource(GameState {
            game: Game::new(),
            node_entities: vec![],
            grid: Grid::new(50, 10),
            status: GameStatus::Pause,
        })
        .add_plugins(DefaultPlugins)
        //.add_plugin(LogDiagnosticsPlugin::default())
        //.add_plugin(FrameTimeDiagnosticsPlugin::default())
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
    node_entities: Vec<Entity>,
}

#[derive(Resource)]
struct EvolutionTimer {
    timer: Timer,
}

#[derive(Component)]
struct MainCamera;

fn setup_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut game_state: ResMut<GameState>,
) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
    let mut entity_ids = vec![];
    for node in game_state.grid.get_cells() {
        let color = if game_state.game.is_node_alive(node.x, node.y) { Color::YELLOW } else { Color::BLACK };
        let entity_id = commands.spawn(
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Cube::new(game_state.grid.cell_size as f32).into()).into(),
                material: materials.add(ColorMaterial::from(color)),
                transform: Transform::from_translation(Vec3::new((node.x * game_state.grid.cell_size) as f32, (node.y * game_state.grid.cell_size) as f32, 0.)),
                ..default()
            }
        ).id();
        entity_ids.push(entity_id);
    }
    game_state.node_entities = entity_ids;
    commands.insert_resource(EvolutionTimer {
        // create the repeating timer
        timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
    });
}

fn evolve_game(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut config: ResMut<EvolutionTimer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    time: Res<Time>,
) {
    if game_state.status == GameStatus::Pause { return; };
    config.timer.tick(time.delta());
    if !config.timer.finished() { return; }
    for node in &game_state.game.live_nodes {
        if node.x < -game_state.grid.radius ||
            node.x >= game_state.grid.radius ||
            node.y < -game_state.grid.radius ||
            node.y >= game_state.grid.radius { continue; };
        let entity_index = game_state.grid.get_index(&node);
        let mut entity = commands.entity(game_state.node_entities[entity_index]);
        entity.insert((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Cube::new(game_state.grid.cell_size as f32).into()).into(),
                material: materials.add(ColorMaterial::from(Color::BLACK)),
                transform: Transform::from_translation(Vec3::new((node.x * game_state.grid.cell_size) as f32, (node.y * game_state.grid.cell_size) as f32, 0.)),
                ..default()
            },
        ));
    }
    game_state.game.evolve();
    for node in &game_state.game.live_nodes {
        if node.x < -game_state.grid.radius ||
            node.x >= game_state.grid.radius ||
            node.y < -game_state.grid.radius ||
            node.y >= game_state.grid.radius { continue; };
        let entity_index = game_state.grid.get_index(&node);
        let mut entity = commands.entity(game_state.node_entities[entity_index]);
        entity.insert((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Cube::new(game_state.grid.cell_size as f32).into()).into(),
                material: materials.add(ColorMaterial::from(Color::YELLOW)),
                transform: Transform::from_translation(Vec3::new((node.x * game_state.grid.cell_size) as f32, (node.y * game_state.grid.cell_size) as f32, 0.)),
                ..default()
            },
        ));
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
    wnds: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    buttons: Res<Input<MouseButton>>,
    mut game_state: ResMut<GameState>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }
    if let Some(world_pos) = get_cursor_world_pos(wnds, q_camera) {
        let node = Grid::get_node_from_world_pos(world_pos.x, world_pos.y, game_state.grid.cell_size as u32);
        game_state.game.toggle(&node);
        let mut entity = commands.entity(game_state.node_entities[game_state.grid.get_index(&node)]);
        let color = if game_state.game.is_node_alive(node.x, node.y) { Color::YELLOW } else { Color::BLACK };
        entity.insert(
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Cube::new(game_state.grid.cell_size as f32).into()).into(),
                material: materials.add(ColorMaterial::from(color)),
                transform: Transform::from_translation(Vec3::new((node.x * game_state.grid.cell_size) as f32, (node.y * game_state.grid.cell_size) as f32, 0.)),
                ..default()
            },
        );
    }
}

fn get_cursor_world_pos(wnds: Res<Windows>, q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>) -> Option<Vec2> {
    let (camera, camera_transform) = q_camera.single();
    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
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

