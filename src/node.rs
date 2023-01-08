#[derive(Debug, Clone)]
pub struct Node {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Node {
    pub fn get_neighbors(&self) -> Vec<Node> {
        let mut neighbors = Vec::new();
        for x in self.x..=(self.x + 2) {
            for y in self.y..=(self.y + 2) {
                neighbors.push(Node { x: x - 1, y: y - 1 })
            }
        }
        neighbors.remove(4);
        neighbors
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn get_neighbors_amount() {
        let node = Node { x: 0, y: 0 };
        let neighbors = node.get_neighbors();
        assert_eq!(neighbors.len(), 8);
    }

    #[test]
    fn get_neighbors_for_node_0_0_contains_neighbors() {
        let node = Node { x: 0, y: 0 };
        let neighbors = node.get_neighbors();
        assert!(neighbors.contains(&Node { x: -1, y: -1 }));
        assert!(neighbors.contains(&Node { x: -1, y: 0 }));
        assert!(neighbors.contains(&Node { x: -1, y: 1 }));
        assert!(neighbors.contains(&Node { x: 0, y: -1 }));
        assert!(!neighbors.contains(&Node { x: 0, y: 0 }));
        assert!(neighbors.contains(&Node { x: 0, y: 1 }));
        assert!(neighbors.contains(&Node { x: 1, y: -1 }));
        assert!(neighbors.contains(&Node { x: 1, y: 0 }));
        assert!(neighbors.contains(&Node { x: 1, y: 1 }));
    }

    #[test]
    fn get_neighbors_for_node_1_1_contains_neighbors() {
        let node = Node { x: 1, y: 1 };
        let neighbors = node.get_neighbors();
        assert!(neighbors.contains(&Node { x: 0, y: 0 }));
        assert!(neighbors.contains(&Node { x: 0, y: 1 }));
        assert!(neighbors.contains(&Node { x: 0, y: 2 }));
        assert!(neighbors.contains(&Node { x: 1, y: 0 }));
        assert!(!neighbors.contains(&Node { x: 1, y: 1 }));
        assert!(neighbors.contains(&Node { x: 1, y: 2 }));
        assert!(neighbors.contains(&Node { x: 2, y: 0 }));
        assert!(neighbors.contains(&Node { x: 2, y: 1 }));
        assert!(neighbors.contains(&Node { x: 2, y: 2 }));
    }

    #[test]
    fn get_neighbors_for_node_negative_2_2_contains_neighbors() {
        let node = Node { x: -2, y: -2 };
        let neighbors = node.get_neighbors();
        assert!(neighbors.contains(&Node {x: -3, y: -3}));
        assert!(neighbors.contains(&Node {x: -3, y: -2}));
        assert!(neighbors.contains(&Node {x: -3, y: -1}));
        assert!(neighbors.contains(&Node {x: -2, y: -3}));
        assert!(!neighbors.contains(&Node {x: -2, y: -2}));
        assert!(neighbors.contains(&Node {x: -3, y: -1}));
        assert!(neighbors.contains(&Node {x: -1, y: -3}));
        assert!(neighbors.contains(&Node {x: -1, y: -2}));
        assert!(neighbors.contains(&Node {x: -1, y: -1}));
    }
}
