use bevy::{diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, prelude::*, sprite::MaterialMesh2dBundle};
use crate::game::Game;
use crate::node::Node;

mod node;
mod game;
mod box_boundary;
mod grid;

fn main() {
    let mut game = Game {
        live_nodes: vec![
            Node { x: 5, y: 0 },
            Node { x: 3, y: 1 },
            Node { x: 4, y: 1 },
            Node { x: 4, y: 2 },
        ]
    };
    App::new()
        .insert_resource(GameState { game, node_entities: vec![] })
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup_cells)
        .run();
}

#[derive(Resource)]
struct GameState {
    game: Game,
    node_entities: Vec<Entity>,
}

fn setup_cells(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut game_state: ResMut<GameState>,
) {
    commands.spawn(Camera2dBundle::default());
    // Node
    let cell_size = 10;
    let board_size = 50;
    let mut nodes = vec![];
    for y in -board_size..=board_size {
        for x in -board_size..=board_size {
            let color = if game_state.game.is_node_alive(x, y) { Color::YELLOW } else { Color::BLACK };
            nodes.push(commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Cube::new(cell_size as f32).into()).into(),
                    material: materials.add(ColorMaterial::from(color)),
                    transform: Transform::from_translation(Vec3::new((x * cell_size) as f32, (y * cell_size) as f32, 0.)),
                    ..default()
                },
            )).id());
        }
    }
    game_state.node_entities = nodes;
}

