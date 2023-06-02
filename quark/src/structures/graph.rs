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

pub struct UnweightedGraph<T> {
    entry: GraphNode<T>,
}

impl<T> UnweightedGraph<T> {
    pub fn insert(&mut self, value: T, parent: T) {todo!()}

    pub fn add_undirected_arc(&self, link_a: T, link_b: T) {todo!()}

    pub fn add_directed_arc(&self, from: T, to: T) {todo!()}

    pub fn contains(&self, value: T) -> bool {todo!()}

    pub fn remove(&self, value: T) -> bool {todo!()}

    pub fn update(&self, old: T, new: T) -> bool {todo!()}

}