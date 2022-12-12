use std::collections::HashMap;

fn main() {
    println!(
        "is anagram = {}",
        is_anagram(String::from("harsh"), String::from("hrahs"))
    );
}

fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut map1: HashMap<char, i32> = HashMap::new();
    let mut map2: HashMap<char, i32> = HashMap::new();
    for i in s.chars() {
        if map1.contains_key(&i) {
            let mut value: i32 = *map1.get(&i).unwrap();
            value += 1;
            map1.insert(i, value);
        } else {
            map1.insert(i, 0);
        }
    }
    for i in t.chars() {
        if map2.contains_key(&i) {
            let mut value: i32 = *map2.get(&i).unwrap();
            value += 1;
            map2.insert(i, value);
        } else {
            map2.insert(i, 0);
        }
    }

    for i in map1.iter() {
        println!("{}", i.0);
        if map1.get(i.0).unwrap_or(&0) != map2.get(i.0).unwrap_or(&0) {
            return false;
        }
    }
    return true;
}
