use std::fmt::Debug;

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
    where T: Clone,
    T: Debug {
    fn new(val: T) -> ListNode<T> {
        ListNode {
            value: val,
            next: Box::new(None),
        }
    }

    fn set_next(&mut self, next: T) {
        self.next = Box::new(Some(ListNode::new(next)));
    }

    fn set_next_node(&mut self, next: ListNode<T>) {
        self.next = Box::new(Some(next));
    }

    fn get_next(&self) -> ListNode<T> {
        return self.next.clone().unwrap();
    }

    fn to_string(&self) -> String {format!("{:?}", self.value)}

    fn drop_tail(&mut self) {
        self.next = Box::new(None);
    }


}

pub struct LinkedList<T> {
    start: ListNode<T>
}

impl<T> LinkedList<T> where T: Clone, T: Debug{
    pub fn get_start(&self) -> T {
        self.start.value.to_owned()
    }

    pub fn insert_first(&mut self, val: T) {
        let mut node = ListNode::new(val);
        node.set_next(self.start.value.clone());
        self.start = node;
    }

    pub fn insert_last(&self, node: T) {
        let mut select = self.start.next.clone();

        while !select.is_none() {
             select = select.unwrap().next;

        }
        select.unwrap().set_next(node);
    }

    pub fn to_string(&self) -> String {
        let mut select = self.start.next.clone().unwrap();
        let mut out = "".to_string();
        while !select.next.is_none() {
            out = format!("{out} -> {:?}", select.value);

            select = select.next.unwrap();
       };
       return out;
    }

    pub fn delete_value(&mut self, value: T) -> bool {
        let mut select = self.start.next.clone().unwrap();
        let mut prev = select.clone();

        while (!select.next.is_none()) && (select.to_string() != format!("{:?}", value)) {
            prev = select.clone();
            select = select.next.unwrap()
        }

        if !select.next.is_none() && select.to_string() == format!("{:?}", value) {
            prev.set_next_node(select.next.unwrap());
            return true;
        } else if select.next.is_none() && select.to_string() == format!("{:?}", value) {
            prev.drop_tail();
            return true;
        } else {false}
        
        
    }


}