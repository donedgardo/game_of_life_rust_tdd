use crate::box_boundary::BoxBoundary;
use crate::node::Node;

pub struct Game {
    pub live_nodes: Vec<Node>,
}

impl Game {
    pub fn new() -> Self {
        Game { live_nodes: Vec::new() }
    }

    pub fn evolve(&mut self) {
        let mut new_live_nodes: Vec<Node> = vec![];
        let boundary = self.get_boundary();
        for x in boundary.lower.x..=boundary.upper.x {
            for y in boundary.lower.y..=boundary.upper.y {
                let live_neighbors = self.get_live_neighbors(&Node { x, y });
                if self.should_node_live(x, y, &live_neighbors) {
                    new_live_nodes.push(Node { x, y });
                }
            }
        }
        self.live_nodes = new_live_nodes;
    }

    pub fn is_node_alive(&self, x: i32, y: i32) -> bool {
        self.live_nodes.contains(&Node { x, y })
    }

    pub fn get_live_neighbors(&self, node: &Node) -> Vec<Node> {
        let neighbors = node.get_neighbors();
        let live_neighbors = neighbors
            .into_iter()
            .filter(|node| self.live_nodes.contains(node))
            .collect();
        live_neighbors
    }

    pub fn live_node_should_die(&self, live_neighbors: &Vec<Node>) -> bool {
        live_neighbors.len() < 2 || live_neighbors.len() > 3
    }

    pub fn dead_node_should_live(&self, live_neighbors: &Vec<Node>) -> bool {
        live_neighbors.len() == 3
    }

    pub fn get_boundary(&self) -> BoxBoundary {
        let mut box_boundary = BoxBoundary::new();
        if self.live_nodes.is_empty() {
            box_boundary
        } else {
            let x_min = self.live_nodes.clone().into_iter()
                .map(|node| node.x).min().unwrap();
            let x_max = self.live_nodes.clone().into_iter()
                .map(|node| node.x).max().unwrap();
            let y_min = self.live_nodes.clone().into_iter()
                .map(|node| node.y).min().unwrap();
            let y_max = self.live_nodes.clone().into_iter()
                .map(|node| node.y).max().unwrap();
            box_boundary.lower.x = x_min - 1;
            box_boundary.upper.x = x_max + 1;
            box_boundary.lower.y = y_min - 1;
            box_boundary.upper.y = y_max + 1;
            box_boundary
        }
    }

    fn should_node_live(&mut self, x: i32, y: i32, live_neighbors: &Vec<Node>) -> bool {
        (self.is_node_alive(x, y) && !self.live_node_should_die(&live_neighbors))
            || (!self.is_node_alive(x, y) && self.dead_node_should_live(&live_neighbors))
    }

    pub fn toggle(&mut self, node: &Node) {
        if !self.live_nodes.contains(&node) {
            self.live_nodes.push(node.clone());
        } else {
            if let Some(index) = self.live_nodes
                .iter()
                .position(|n| n.eq(&node)) {
                self.live_nodes.remove(index);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::game::Game;
    use super::*;

    #[test]
    fn new_game_has_no_live_nodes() {
        let game = Game::new();
        assert!(game.live_nodes.is_empty());
    }

    #[test]
    fn game_with_no_live_nodes_evolves_with_0_live_nodes() {
        let mut game = Game::new();
        game.evolve();
        assert_eq!(game.live_nodes.len(), 0);
    }

    #[test]
    fn game_with_no_live_nodes_returns_0_0_boundary() {
        let game = Game::new();
        let boundary = game.get_boundary();
        assert_eq!(boundary.lower, Node { x: 0, y: 0 });
        assert_eq!(boundary.upper, Node { x: 0, y: 0 });
    }

    #[test]
    fn game_with_live_nodes_returns_neighbor_boundary() {
        let mut game = Game {
            live_nodes: vec![Node { x: 0, y: 0 }]
        };
        let mut boundary = game.get_boundary();
        assert_eq!(boundary.lower, Node { x: -1, y: -1 });
        assert_eq!(boundary.upper, Node { x: 1, y: 1 });

        game.live_nodes = vec![Node { x: 0, y: 0 }, Node { x: 2, y: 3 }];
        boundary = game.get_boundary();
        assert_eq!(boundary.lower, Node { x: -1, y: -1 });
        assert_eq!(boundary.upper, Node { x: 3, y: 4 });

        game.live_nodes = vec![Node { x: 0, y: 0 }, Node { x: 2, y: 3 }, Node { x: -1, y: -1 }];
        boundary = game.get_boundary();
        assert_eq!(boundary.lower, Node { x: -2, y: -2 });
        assert_eq!(boundary.upper, Node { x: 3, y: 4 });
    }

    #[test]
    fn game_example_1_passes() {
        let mut game = Game {
            live_nodes: vec![
                Node { x: 5, y: 0 },
                Node { x: 3, y: 1 },
                Node { x: 4, y: 1 },
                Node { x: 4, y: 2 },
            ]
        };
        game.evolve();
        assert_eq!(game.live_nodes.len(), 6);
        assert!(game.live_nodes.contains(&Node { x: 4, y: 0 }));
        assert!(game.live_nodes.contains(&Node { x: 3, y: 1 }));
        assert!(game.live_nodes.contains(&Node { x: 4, y: 1 }));
        assert!(game.live_nodes.contains(&Node { x: 5, y: 1 }));
        assert!(game.live_nodes.contains(&Node { x: 3, y: 2 }));
        assert!(game.live_nodes.contains(&Node { x: 4, y: 2 }));
    }

    #[test]
    fn a_live_node_with_0_live_neighbors_should_die() {
        let game = Game {
            live_nodes: vec![Node { x: 0, y: 0 }]
        };
        let live_neighbors = game.get_live_neighbors(&Node { x: 0, y: 0 });
        assert!(game.live_node_should_die(&live_neighbors));
    }

    #[test]
    fn a_live_node_with_1_live_neighbors_should_die() {
        let game = Game {
            live_nodes: vec![Node { x: 0, y: 0 }, Node { x: 1, y: 1 }]
        };
        let live_neighbors = game.get_live_neighbors(&Node { x: 0, y: 0 });
        assert!(game.live_node_should_die(&live_neighbors));
    }

    #[test]
    fn a_live_node_with_2_live_neighbors_should_not_die() {
        let game = Game {
            live_nodes: vec![Node { x: 0, y: 0 }, Node { x: 1, y: 1 }, Node { x: -1, y: 0 }]
        };

        let live_neighbors = game.get_live_neighbors(&Node { x: 0, y: 0 });
        assert!(!game.live_node_should_die(&live_neighbors));
    }

    #[test]
    fn a_live_node_with_3_live_neighbors_should_not_die() {
        let game = Game {
            live_nodes: vec![
                Node { x: 0, y: 0 },
                Node { x: 1, y: 1 },
                Node { x: -1, y: 0 },
                Node { x: -1, y: -1 }]
        };
        let live_neighbors = game.get_live_neighbors(&Node { x: 0, y: 0 });
        assert!(!game.live_node_should_die(&live_neighbors));
    }

    #[test]
    fn a_live_node_with_4_live_neighbors_should_die() {
        let game = Game {
            live_nodes: vec![
                Node { x: 0, y: 0 },
                Node { x: 1, y: 1 },
                Node { x: -1, y: 0 },
                Node { x: 0, y: -1 },
                Node { x: -1, y: -1 }]
        };
        let live_neighbors = game.get_live_neighbors(&Node { x: 0, y: 0 });
        assert!(game.live_node_should_die(&live_neighbors));
    }

    #[test]
    fn a_dead_cell_with_1_live_neighbours_should_not_live() {
        let game = Game {
            live_nodes: vec![Node { x: 1, y: 1 }]
        };
        let live_neighbors = game.get_live_neighbors(&Node { x: 0, y: 0 });
        assert!(!game.dead_node_should_live(&live_neighbors));
    }

    #[test]
    fn a_dead_cell_with_2_live_neighbours_should_not_live() {
        let game = Game {
            live_nodes: vec![Node { x: 1, y: 1 }, Node { x: -1, y: -1 }]
        };
        let live_neighbors = game.get_live_neighbors(&Node { x: 0, y: 0 });
        assert!(!game.dead_node_should_live(&live_neighbors));
    }

    #[test]
    fn a_dead_cell_with_3_live_neighbours_should_live() {
        let game = Game {
            live_nodes: vec![Node { x: 1, y: 1 }, Node { x: -1, y: -1 }, Node { x: 1, y: -1 }]
        };
        let live_neighbors = game.get_live_neighbors(&Node { x: 0, y: 0 });
        assert!(game.dead_node_should_live(&live_neighbors));
    }

    #[test]
    fn a_dead_cell_with_4_live_neighbours_should_not_live() {
        let game = Game {
            live_nodes: vec![
                Node { x: 1, y: 1 },
                Node { x: -1, y: -1 },
                Node { x: 1, y: -1 },
                Node { x: 0, y: 1 }]
        };
        let live_neighbors = game.get_live_neighbors(&Node { x: 0, y: 0 });
        assert!(!game.dead_node_should_live(&live_neighbors));
    }

    #[test]
    fn game_with_no_live_nodes_has_no_live_neighbors_at_0_0() {
        let game = Game::new();
        let node_zero = Node { x: 0, y: 0 };
        let live_neighbors = game.get_live_neighbors(&node_zero);
        assert_eq!(live_neighbors.len(), 0);
    }

    #[test]
    fn game_with_live_nodes_near_zero_node_returns_live_neighbors() {
        let game = Game {
            live_nodes: vec![Node { x: -1, y: -1 }, Node { x: 1, y: 1 }]
        };
        let node_zero = Node { x: 0, y: 0 };
        let live_neighbors = game.get_live_neighbors(&node_zero);
        assert_eq!(live_neighbors.len(), 2);
        assert_eq!(live_neighbors[0], game.live_nodes[0]);
        assert_eq!(live_neighbors[1], game.live_nodes[1]);
    }

    #[test]
    fn game_with_live_nodes_not_near_zero_node_returns_0_live_neighbors() {
        let game = Game {
            live_nodes: vec![Node { x: -3, y: -3 }, Node { x: -3, y: 3 }]
        };
        let node_zero = Node { x: 0, y: 0 };
        let live_neighbors = game.get_live_neighbors(&node_zero);
        assert_eq!(live_neighbors.len(), 0);
    }

    #[test]
    fn can_toggle_node_from_dead_to_alive() {
        let mut game = Game {
            live_nodes: vec![]
        };
        let node_zero = Node { x: 0, y: 0 };
        game.toggle(&node_zero);
        assert_eq!(game.live_nodes.len(), 1);
    }

    #[test]
    fn can_toggle_node_from_alive_to_dead() {
        let mut game = Game {
            live_nodes: vec![Node { x: 0, y: 0 }]
        };
        let node_zero = Node { x: 0, y: 0 };
        game.toggle(&node_zero);
        assert_eq!(game.live_nodes.len(), 0);
    }
}