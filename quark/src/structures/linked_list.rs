use std::{fmt::Debug, sync::{Arc, RwLock}};


struct ListNode<T> {
    value: T,
    next: Option<Arc<RwLock<ListNode<T>>>>
}




impl<T> ListNode<T> 
    where T: Debug {
    fn new(val: T) -> ListNode<T> {
        ListNode {
            value: val,
            next: None,
        }
    }

    fn new_struct(val: T) -> Arc<RwLock<ListNode<T>>> {
        Arc::new(RwLock::new(ListNode::new(val)))
    }


    fn set_next<'a>(&mut self, next: T) {
        self.next = Some(ListNode::new_struct(next));
    }

    fn set_next_node(&mut self, next_node: Option<Arc<RwLock<ListNode<T>>>>) {
        self.next = next_node;
    }

    fn get_next(&self) -> Option<Arc<RwLock<ListNode<T>>>> {
        return self.next.clone();
    }

    fn to_string(&self) -> String {format!("{:?}", self.value)}

    fn drop_tail(&mut self) {
        self.next = None;
    }


}

pub struct LinkedList<T: 'static> {
    start: Arc<RwLock<ListNode<T>>>
}

impl<T> LinkedList<T> where T: Clone, T: Debug {
    pub fn new(start: T) -> LinkedList<T> {
        LinkedList { 
            start: ListNode::new_struct(start)
        }
    }


    pub fn get_start(&self) -> T {
        self.start.clone().read().unwrap().value.clone()
    }

    pub fn get_size(&self) -> usize {
        let mut select = self.start.read().unwrap().get_next();
        let mut count = 1;

        while !select.is_none() {
            select = select.unwrap().read().unwrap().get_next();
            count += 1;
        }

        return count;
    }

    pub fn insert_first(&mut self, val: T) {
        let node = ListNode::new_struct(val);
        node.write().unwrap().set_next_node(Some(self.start.clone()));
        self.start = node;
    }

    pub fn insert_last(&mut self, node: T) {
        let mut select = self.start.clone();

        loop {
            match select.clone().read().unwrap().get_next() {
                Some(next) => {
                    select = next;
                },
                None => break,
            };
        }
        select.write().unwrap().set_next(node);
    }

    pub fn to_string(&self) -> String {
        let mut select = self.start.clone();
        let mut out = "".to_string();
        loop {
            out = format!("{out} -> {:?}", select.read().unwrap().value);
            println!("{}", out);

            match select.clone().read().unwrap().get_next() {
                Some(next) => select = next,
                None => break,
            }
       };
       return out;
    }

    pub fn delete_value(&mut self, value: T) -> bool {
        let mut select = self.start.read().unwrap().get_next().unwrap();
        let mut prev = select.clone();

        while (!select.clone().read().unwrap().next.is_none()) && (select.clone().read().unwrap().to_string() != format!("{:?}", value)) {
            prev = select.clone();
            select = select.clone().read().unwrap().next.clone().unwrap();
        }

        if !select.read().unwrap().next.is_none() && select.read().unwrap().to_string() == format!("{:?}", value) {
            prev.write().unwrap().set_next_node(Some(select.read().unwrap().next.clone().unwrap().clone()));
            return true;
        } else if select.read().unwrap().next.is_none() && select.read().unwrap().to_string() == format!("{:?}", value) {
            prev.write().unwrap().drop_tail();
            return true;
        } else {false}
        
    }

    pub fn delete_after(&mut self, len: usize) -> bool {
        let mut select = self.start.clone();
        if len >= self.get_size() {return false;}

        for _ in 0..len-1 {
            select = select.clone().read().unwrap().next.clone().unwrap();
        }
        select.write().unwrap().drop_tail();
        return true;
    }


}