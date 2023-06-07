use std::{fmt::Debug, ops::Deref};

struct ListNode<'a, T> {
    value: T,
    next: Option<&'a ListNode<'a, T>>
}

impl<T> Clone for ListNode<'_, T> where T: Clone {
    fn clone(&self) -> Self {
        Self { 
            value: self.value.clone(), 
            next: self.next 
        }
    }
}

impl<'a, T> AsRef<ListNode<'a, T>> for ListNode<'a, T> {
    fn as_ref(&self) -> &ListNode<'a, T> {
        self
    }
}

impl<'a, T> AsMut<ListNode<'a, T>> for ListNode<'a, T> {
    fn as_mut(&mut self) -> &mut ListNode<'a, T> {
        self
    }
}

impl<T> ListNode<'_, T> 
    where T: Clone + 'static,
    T: Debug {
    fn new(val: T) -> ListNode<'static, T> {
        ListNode {
            value: val,
            next: None,
        }
    }

    fn set_next<'a>(&mut self, next: T) {
        self.next = Some(&ListNode::new(next));
    }

    fn set_next_node(&mut self, next_node: &ListNode<T>) {
        self.next = Some(next_node);
    }

    fn get_next(&self) -> Option<&ListNode<T>> {
        return self.next;
    }

    fn to_string(&self) -> String {format!("{:?}", self.value)}

    fn drop_tail(&mut self) {
        self.next = None;
    }


}

pub struct LinkedList<'a, T> {
    start: &'a ListNode<'a, T>
}

impl<T> LinkedList<'_, T> where T: Clone, T: Debug + 'static{
    pub fn new(start: T) -> LinkedList<'static, T> {
        LinkedList { 
            start: &ListNode { 
                value: start, 
                next: None 
            }
        }
    }


    pub fn get_start(&self) -> T {
        self.start.value.to_owned()
    }

    pub fn get_size(&self) -> usize {
        let mut select = self.start.next;
        let mut count = 0;

        while !select.is_none() {
            select = select.unwrap().next;
            count += 1;
        }

        return count;
    }

    pub fn insert_first(&mut self, val: T) {
        let mut node = ListNode::new(val);
        node.set_next(self.start.value.clone());
        self.start = &node;
    }

    pub fn insert_last(&mut self, node: T) {
        let mut select = self.start;

        loop {
            match select.next.is_none() {
                true => break,
                false => select = select.get_next().unwrap(),
            }
        }
        select.set_next(node);
    }

    pub fn to_string(&self) -> String {
        let mut select = self.start;
        let mut out = "".to_string();
        loop {
            out = format!("{out} -> {:?}", select.value);
            println!("{}", out);

            match select.next.is_none() {
                true => break,
                false => select = select.next.unwrap().as_ref(),
            }
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

    pub fn delete_after(&mut self, len: usize) -> bool {
        let mut select = self.start;
        if len > self.get_size() {return false;}

        if len > 0 {
            for i in 0..len-1 {
                select = select.next.unwrap();
            }
        }
        select.drop_tail();
        return true;
    }


}