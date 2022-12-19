fn main() {
    println!("ans is {:?}", letter_combinations("23".to_string()));
}

struct CharsFromNum<usize> {
    number: usize,
}

impl CharsFromNum<usize> {
    fn get_chars(&self) -> Vec<char> {
        match self.number {
            2 => vec!['a', 'b', 'c'],
            3 => vec!['d', 'e', 'f'],
            4 => vec!['g', 'h', 'i'],
            5 => vec!['j', 'k', 'l'],
            6 => vec!['m', 'n', 'o'],
            7 => vec!['p', 'q', 'r', 's'],
            8 => vec!['t', 'u', 'v'],
            9 => vec!['w', 'x', 'y', 'z'],
            _ => Vec::new(),
        }
    }
}

fn letter_combinations(digits: String) -> Vec<String> {
    let mut ans: Vec<String> = Vec::new();
    if digits.is_empty() {
        return ans;
    }
    ans.push(String::new());
    let digit_arr: Vec<char> = digits.chars().collect();
    let char_num_array: Vec<CharsFromNum<usize>> = digit_arr
        .into_iter()
        .map(|char| CharsFromNum {
            number: ((char as i32 - 0x30) as usize),
        })
        .collect();
    for i in char_num_array.iter() {
        let mut tmp_str: Vec<String> = Vec::new();
        for j in i.get_chars() {
            for k in ans.iter() {
                let mut some = String::from(k);
                some.push(j);
                tmp_str.push(some);
            }
        }
        std::mem::swap(&mut tmp_str, &mut ans);
    }
    return ans;
}
