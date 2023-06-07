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

impl<T> Clone for GraphNode<T> where T: Clone {
    fn clone(&self) -> Self {
        Self { value: self.value.clone(), arcs: self.arcs.clone() }
    }
}

impl<T> PartialEq for GraphNode<T> where T:PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.arcs == other.arcs
    }

    
}

pub struct UnweightedGraph<T> {
    entry: GraphNode<T>,
}

impl<T> UnweightedGraph<T> where T: Clone, T: PartialEq {
    fn get_node(&self, value: T) -> Option<GraphNode<T>> {
        let mut queue = self.entry.arcs.clone();
        let mut checked: Vec<GraphNode<T>> = vec![];

        while queue.len() > 0 {
            let mut node = queue.pop().unwrap();
            if node.value == value && !checked.contains(&node) {
                return Some(node);
            }
            checked.push(node.clone());
            queue.append(&mut node.arcs)
            
        }

        return None;
    }

    pub fn insert(&mut self, value: T, parent: T) {
        
    }

    pub fn add_undirected_arc(&self, link_a: T, link_b: T) {todo!()}

    pub fn add_directed_arc(&self, from: T, to: T) {todo!()}

    pub fn contains(&self, value: T) -> bool {todo!()}

    pub fn remove(&self, value: T) -> bool {todo!()}

    pub fn update(&self, old: T, new: T) -> bool {todo!()}

}