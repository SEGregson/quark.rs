struct ListNode<T> {
    value: T,
    next: Box<Option<ListNode<T>>>
}

impl<T> Clone for ListNode<T> where T: Clone {
    fn clone(&self) -> Self {
        Self { 
            value: self.value.clone(), 
            next: self.next.clone() 
        }
    }
}

impl<T> ListNode<T> 
    where T: Clone {
    pub fn new(val: T) -> ListNode<T> {
        ListNode {
            value: val,
            next: Box::new(None),
        }
    }

    pub fn set_next(&mut self, next: T) {
        self.next = Box::new(Some(ListNode::new(next)));
    }

    pub fn get_next(&self) -> ListNode<T> {

        return self.next.clone().unwrap();
    }
}

pub struct LinkedList<T> {
    start: ListNode<T>
}

impl<T> LinkedList<T> where T: Clone{
    

    pub fn insert_first(&mut self, mut val: T) {
        let mut node = ListNode::new(val);
        node.set_next(self.start.value.clone());
        self.start = node;
    }

    pub fn insert_last(&self, mut node: ListNode<T>) {

    }
}