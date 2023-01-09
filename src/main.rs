use std::time::Duration;
use bevy::{diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, prelude::*, sprite::MaterialMesh2dBundle};
use crate::game::Game;
use crate::grid::Grid;
use crate::node::Node;

mod node;
mod game;
mod box_boundary;
mod grid;

fn main() {
    let game = Game {
        live_nodes: vec![
            Node { x: -20, y: -20 },
            Node { x: -21, y: -19 },
            Node { x: -21, y: -18 },
            Node { x: -20, y: -18 },
            Node { x: -19, y: -18 },
        ]
    };
    App::new()
        .insert_resource(GameState { game, node_entities: vec![], grid: Grid::new(50), cell_size: 10 })
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        //.add_startup_system(setup_game_batched)
        .add_startup_system(setup_game)
        .add_system(evolve_game)
        .run();
}

#[derive(Resource)]
struct GameState {
    game: Game,
    grid: Grid,
    node_entities: Vec<Entity>,
    cell_size: i32,
}

#[derive(Resource)]
struct EvolutionTimer {
    timer: Timer,
}

fn setup_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut game_state: ResMut<GameState>,
) {
    commands.spawn(Camera2dBundle::default());
    let nodes = game_state.grid.get_cells().into_iter().map(|node| {
        let color = if game_state.game.is_node_alive(node.x, node.y) { Color::YELLOW } else { Color::BLACK };
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Cube::new(game_state.cell_size as f32).into()).into(),
                material: materials.add(ColorMaterial::from(color)),
                transform: Transform::from_translation(Vec3::new((node.x * game_state.cell_size) as f32, (node.y * game_state.cell_size) as f32, 0.)),
                ..default()
            },
        )).id()
    }).collect();

    commands.insert_resource(EvolutionTimer {
        // create the repeating timer
        timer: Timer::new(Duration::from_micros(5), TimerMode::Repeating),
    });
    game_state.node_entities = nodes;
}

fn setup_game_batched(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut game_state: ResMut<GameState>,
) {
    commands.spawn(Camera2dBundle::default());
    let mut bundles = vec![];
    for node in game_state.grid.get_cells() {
        let color = if game_state.game.is_node_alive(node.x, node.y) { Color::YELLOW } else { Color::BLACK };
        bundles.push(
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Cube::new(game_state.cell_size as f32).into()).into(),
                material: materials.add(ColorMaterial::from(color)),
                transform: Transform::from_translation(Vec3::new((node.x * game_state.cell_size) as f32, (node.y * game_state.cell_size) as f32, 0.)),
                ..default()
            },
        )
    }

    commands.spawn_batch(bundles);
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
    config.timer.tick(time.delta());
    if config.timer.finished() {
        for node in &game_state.game.live_nodes {
            let entity_index = game_state.grid.get_index(&node);
            let mut entity = commands.entity(game_state.node_entities[entity_index]);
            entity.insert((
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Cube::new(game_state.cell_size as f32).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::BLACK)),
                    transform: Transform::from_translation(Vec3::new((node.x * game_state.cell_size) as f32, (node.y * game_state.cell_size) as f32, 0.)),
                    ..default()
                },
            ));
        }
        game_state.game.evolve();
        for node in &game_state.game.live_nodes {
            let entity_index = game_state.grid.get_index(&node);
            let mut entity = commands.entity(game_state.node_entities[entity_index]);
            entity.insert((
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Cube::new(game_state.cell_size as f32).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::YELLOW)),
                    transform: Transform::from_translation(Vec3::new((node.x * game_state.cell_size) as f32, (node.y * game_state.cell_size) as f32, 0.)),
                    ..default()
                },
            ));
        }
    }
}
