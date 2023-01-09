use crate::node::Node;

pub struct Grid {
    pub radius: i32,
    pub cell_size: i32,
    cells: Vec<Node>,
}

impl Grid {}

impl Grid {
    pub fn new(radius: i32, cell_size: i32) -> Self {
        let mut cells = Vec::new();
        for y in -radius..radius {
            for x in -radius..radius {
                cells.push(Node { x, y });
            }
        }
        Self {
            radius,
            cell_size,
            cells,
        }
    }
    pub fn get_node(&self, index: usize) -> &Node {
        &self.cells[index]
    }

    pub fn get_index(&self, node: &Node) -> usize {
        (((node.y + self.radius) * (self.radius * 2)) + (node.x + self.radius)) as usize
    }

    pub fn get_cells(&self) -> &Vec<Node> {
        &self.cells
    }

    pub fn get_node_from_world_pos(world_x: f32, world_y: f32, cell_size: u32) -> Node {
        let offset = (cell_size / 2) as f32;
        Node {
            x: { ((world_x + offset) / cell_size as f32).floor() as i32 },
            y: { ((world_y + offset) / cell_size as f32).floor() as i32 },
        }
    }
}

#[cfg(test)]
mod test {
    use crate::grid::Grid;
    use crate::node::Node;

    #[test]
    fn can_index_a_cartesian_index() {
        let radius = 8;
        let grid = Grid::new(radius, 10);
        assert_eq!(grid.get_cells().len() as i32, (radius * 2) * (radius * 2));
    }

    #[test]
    fn returns_zero_node_index() {
        let radius = 8;
        let grid = Grid::new(radius, 10);
        assert_eq!(grid.get_node(0), &Node { x: -8, y: -8 });
        assert_eq!(grid.get_index(&Node { x: -8, y: -8 }), 0);
        assert_eq!(grid.get_node(1), &Node { x: -7, y: -8 });
        assert_eq!(grid.get_index(&Node { x: -7, y: -8 }), 1);
        assert_eq!(grid.get_node(2), &Node { x: -6, y: -8 });
        assert_eq!(grid.get_index(&Node { x: -6, y: -8 }), 2);
        assert_eq!(grid.get_node(16), &Node { x: -8, y: -7 });
        assert_eq!(grid.get_index(&Node { x: -8, y: -7 }), 16);
        assert_eq!(grid.get_node(grid.cells.len() - 1), &Node { x: 7, y: 7 });
        assert_eq!(grid.get_index(&Node { x: 7, y: 7 }), grid.cells.len() - 1);
    }

    #[test]
    fn world_position_to_node() {
        let cell_size = 10;
        let mut world_x = 0.1;
        let mut world_y = 0.1;
        //0, 0
        assert_eq!(Grid::get_node_from_world_pos(world_x, world_y, cell_size), Node { x: 0, y: 0 });
        world_x = -0.1;
        world_y = -0.1;
        // 0, 0
        assert_eq!(Grid::get_node_from_world_pos(world_x, world_y, cell_size), Node { x: 0, y: 0 });
        world_x = 5.0;
        world_y = 5.0;
        // 1, 1
        assert_eq!(Grid::get_node_from_world_pos(world_x, world_y, cell_size), Node { x: 1, y: 1 });
        world_x = -5.1;
        world_y = -5.1;
        // -1, -1
        assert_eq!(Grid::get_node_from_world_pos(world_x, world_y, cell_size), Node { x: -1, y: -1 });
        world_x = 5.1;
        world_y = 5.1;
        // 1, 1
        assert_eq!(Grid::get_node_from_world_pos(world_x, world_y, cell_size), Node { x: 1, y: 1 });
        world_x = 11.0;
        world_y = 12.0;
        // 1, 1
        assert_eq!(Grid::get_node_from_world_pos(world_x, world_y, cell_size), Node { x: 1, y: 1 });
    }
}
