pub mod io;
mod structures;


mod tests {
    #[cfg(test)]
    mod linked_lists_tests {
        use crate::structures::linked_list::LinkedList;

        #[test]
        fn declaration() {
            let list = LinkedList::new(3);
            assert_eq!(3, list.get_start());
        }

        #[test]
        fn insert_last() {
            let mut list = LinkedList::new(3);
            list.insert_last(4);
            // println!("{}", list.to_string());
            assert_eq!(" -> 3 -> 4", list.to_string());
        }

        #[test]
        fn size() {
            let mut list = LinkedList::new(3);
            list.insert_last(4);
            list.insert_last(3);
            list.insert_last(7);
            assert_eq!(4, list.get_size());
        }
    }

    //#[test]
    
}
