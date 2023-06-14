pub mod functions;
pub mod io;
mod structures;

#[cfg(test)]
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
        fn get_index() {
            let mut list = LinkedList::new(3);
            list.insert_last(4);
            list.insert_last(3);
            list.insert_last(7);
            assert_eq!(list.get(1), Some(4));
        }

        #[test]
        fn size() {
            let mut list = LinkedList::new(3);
            list.insert_last(4);
            list.insert_last(3);
            list.insert_last(7);
            assert_eq!(4, list.get_size());
        }

        #[test]
        fn insert_last() {
            let mut list = LinkedList::new(3);
            list.insert_last(4);
            // println!("{}", list.to_string());
            assert_eq!(" -> 3 -> 4", list.to_string());
        }

        

        #[test]
        fn insert_first() {
            let mut list = LinkedList::new(3);
            list.insert_first(4);
            // println!("{}", list.to_string());
            assert_eq!(" -> 4 -> 3", list.to_string());
        }

        #[test]        
        fn delete_index() {
            let mut list = LinkedList::new(3);
            list.insert_last(4);
            list.insert_last(3);
            list.insert_last(7);
            list.delete(0);
            assert_eq!(" -> 4 -> 3 -> 7", list.to_string());
        }

        #[test]
        fn delete_after() {
            let mut list = LinkedList::new(3);
            list.insert_last(4);
            list.insert_last(3);
            list.insert_last(7);
            list.delete_after(2);
            assert_eq!(" -> 3 -> 4", list.to_string());
        }

        
    }

    #[cfg(test)]
    mod sort_tests {
        use crate::functions::sorts::{bubble_sort_decending, bubble_sort_ascending};

        #[test]
        fn bubble_sort_decending_test() {
            let vec = vec![1,2,3,4];
            assert_eq!(vec![4,3,2,1], bubble_sort_decending(vec))
        }

        #[test]
        fn bubble_sort_ascending_test() {
            let vec = vec![4,3,2,1];
            assert_eq!(vec![1,2,3,4], bubble_sort_ascending(vec))
        }
    }
    
}
