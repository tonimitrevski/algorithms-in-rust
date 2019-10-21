fn main() {
    let mut vec = vec![5, 2, 8, 12, 7];

    for x in 0..vec.len() {
        if x == 0 { continue; }

        let current = vec[x];
        let mut previous_index_checker = x - 1;
        insertion_sort_from_prev_index(&mut vec, current, &mut previous_index_checker);
        add_previous_index_value(&mut vec, current, previous_index_checker)
    }

    println!("Insertion Sort! {}", format!("{:?}", vec));
}

fn insertion_sort_from_prev_index(vec: &mut Vec<i32>, current: i32, previous_index_checker: &mut usize) {
    let mut dereference_index = *previous_index_checker;
    while vec[dereference_index] > current {
        vec[dereference_index + 1] = vec[dereference_index];
        if dereference_index == 0 {
            break;
        }
        dereference_index -= 1;
    }
}

fn add_previous_index_value(vec: &mut Vec<i32>, current: i32, previous_index_checker: usize) -> () {
    if previous_index_checker == 0 {
        vec[0] = current;
        return;
    }

    vec[previous_index_checker + 1] = current;
}