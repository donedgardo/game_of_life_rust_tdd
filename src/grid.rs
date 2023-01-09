use crate::node::Node;

pub struct Grid {
    radius: i32,
    cells: Vec<Node>,
}

impl Grid {
    pub fn new(radius: i32) -> Self {
        let mut cells = Vec::new();
        for y in -radius..radius {
            for x in -radius..radius {
                cells.push(Node {x, y});
            }
        }
        Self {
            radius,
            cells,
        }
    }
    pub fn get_node(&self, index: usize) -> &Node {
        &self.cells[index]
    }

    pub fn get_index(&self, node: &Node) -> usize {
        (((node.y + self.radius) * (self.radius *2)) + (node.x + self.radius)) as usize
    }

    pub(crate) fn get_cells(&self) -> &Vec<Node>  {
        &self.cells
    }
}

#[cfg(test)]
mod test {
    use crate::grid::Grid;
    use crate::node::Node;

    #[test]
    fn can_index_a_cartesian_index() {
        let radius = 8;
        let grid = Grid::new(radius);
        assert_eq!(grid.get_cells().len() as i32, (radius * 2) * (radius * 2));
    }

    #[test]
    fn returns_zero_node_index() {
        let radius = 8;
        let grid = Grid::new(radius);
        assert_eq!(grid.get_node(0), &Node {x: -8, y: -8});
        assert_eq!(grid.get_index(&Node {x: -8, y: -8}), 0);
        assert_eq!(grid.get_node(1), &Node {x: -7, y: -8});
        assert_eq!(grid.get_index(&Node {x: -7, y: -8}), 1);
        assert_eq!(grid.get_node(2), &Node {x: -6, y: -8});
        assert_eq!(grid.get_index(&Node {x: -6, y: -8}), 2);
        assert_eq!(grid.get_node(16), &Node {x: -8, y: -7});
        assert_eq!(grid.get_index(&Node {x: -8, y: -7}), 16);
        assert_eq!(grid.get_node(grid.cells.len() - 1), &Node {x: 7, y: 7});
        assert_eq!(grid.get_index(&Node {x: 7, y: 7}), grid.cells.len() - 1);
    }
}
