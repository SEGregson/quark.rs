/*
    assumes vector is ordered into ascending order
 */
pub fn binary_search<T: PartialOrd + Copy>(mut lst: Vec<T>, val: T) -> Option<usize> {
    let mut select;
    let mut count;
    loop {
        count = lst.len()/2;
        select = lst[count];
        if select == val {
            return Some(count);
        } else {
            if lst.len() <= 1 {return None;}
            lst = remove_half(lst, select < val);
        }
    }
    fn remove_half<T>(mut lst: Vec<T>, side: bool) -> Vec<T> {
        for i in 
            if side {0..lst.len()/2} else {lst.len()/2..lst.len()} {
                lst.remove(i);
        }
        return lst;
    
    }
}

