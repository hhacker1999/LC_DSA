// Arrays approach
fn get_first_recurring_element(array: &Vec<i32>) -> Option<i32> {
    let mut unique_vec: Vec<i32> = Vec::new();
    for i in array {
        if unique_vec.contains(i) {
            return Some(*i);
        } else {
            unique_vec.push(*i);
        }
    }
    None
}


// Hash map approach
fn first_recurrin_element_using_hash_map(array: &Vec<i32>) -> Option<i32> {
    use::std::collections::HashMap;
    let mut hash_map: HashMap<i32, i32> = HashMap::new();
    for i in array {
        if let Some(0) = hash_map.get(i) {
            return Some(*i);
        } else {
            hash_map.insert(*i, 0);
        }
    }
    None
}
