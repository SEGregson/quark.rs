pub fn bubble_sort_decending<T: PartialOrd + Copy>(mut lst: Vec<T>) -> Vec<T> {
    if lst.len() <= 1 {
        return lst;
    }
    let mut sorted = false;
    while !sorted {
        sorted = true;

        for i in 0..lst.len()-1 {
            if lst[i] < lst[i+1] {
                (lst[i], lst[i+1]) = (lst[i+1], lst[i]);
                sorted = false;
            }
        }
    }
    return lst
}

pub fn bubble_sort_ascending<T: PartialOrd + Copy>(mut lst: Vec<T>) -> Vec<T> {
    if lst.len() <= 1 {
        return lst;
    }
    let mut sorted = false;
    while !sorted {
        sorted = true;

        for i in 0..lst.len()-1 {
            if lst[i] > lst[i+1] {
                (lst[i], lst[i+1]) = (lst[i+1], lst[i]);
                sorted = false;
            }
        }
    }
    return lst
}