use crate::node::Node;

pub struct BoxBoundary {
    pub lower: Node,
    pub upper: Node,
}

impl BoxBoundary {
    pub fn new() -> Self {
        BoxBoundary {
            lower: Node { x: 0, y: 0 },
            upper: Node { x: 0, y: 0 },
        }
    }
}
