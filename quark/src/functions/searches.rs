use super::sorts::bubble_sort_ascending;

pub fn binary_search<T: PartialOrd + Copy>(lst: Vec<T>, val: T) -> bool {
    let mut subj = bubble_sort_ascending(lst);
    
    let mut select;
    loop {
        
        select = subj[subj.len()/2];
        if select == val {
            return true
        } else {
            if subj.len() <= 1 {return false;}
            subj = remove_half(subj, select < val);
        }
    }
}

fn remove_half<T>(mut lst: Vec<T>, side: bool) -> Vec<T> {
    for i in 
        if side {0..lst.len()/2} else {lst.len()/2..lst.len()} {
            lst.remove(i);
    }
    return lst;

}