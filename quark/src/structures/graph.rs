struct GraphNode<T> {
    value: T,
    arcs: Vec<GraphNode<T>>
}

impl<T> GraphNode<T> {
    pub fn new(val: T) -> GraphNode<T> {
        GraphNode { 
            value: val, 
            arcs: vec![]
        }
    }
}

