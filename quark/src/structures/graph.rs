use super::thead_data::ThreadData;

struct GraphNode<T> {
    value: T,
    arcs: Vec<ThreadData<GraphNode<T>>>
}

impl<T> GraphNode<T> {
    pub fn new(val: T) -> GraphNode<T> {
        GraphNode { 
            value: val, 
            arcs: vec![]
        }
    }

    fn new_struct(val: T) -> ThreadData<GraphNode<T>> {
        ThreadData::new(GraphNode::new(val))
    }

    pub fn add_arc(&mut self, val: T) {
        self.arcs.push(GraphNode::new_struct(val))
    } 

    pub fn get_node(&self, val: T){todo!()}

}


pub struct UnweightedGraph<T> {
    entry: ThreadData<GraphNode<T>>,
}

impl<T> UnweightedGraph<T>  {
    fn get_node(&self, val: T) -> Option<GraphNode<T>> {
        let mut select = self.entry.clone();

        loop {
            match select.try_read_access() {
                Some(read) => {
                    todo!()
                },
                None => todo!(),
            }
        }
    }

    pub fn insert(&mut self, value: T, parent: T) {todo!()}

    pub fn add_undirected_arc(&self, link_a: T, link_b: T) {todo!()}

    pub fn add_directed_arc(&self, from: T, to: T) {todo!()}

    pub fn contains(&self, value: T) -> bool {todo!()}

    pub fn remove(&self, value: T) -> bool {todo!()}

    pub fn update(&self, old: T, new: T) -> bool {todo!()}

}