fn main() {
    println!("FizzBuzz is {:?}", fizz_buzz(15));
}

fn fizz_buzz(n: i32) -> Vec<String> {
    let mut ans: Vec<String> = Vec::new();
    for i in 1..=n {
        if i % 3 == 0 && i % 5 == 0 {
            ans.push(String::from("FizzBuzz"));
            continue;
        }
        if i % 3 == 0 {
            ans.push(String::from("Fizz"));
            continue;
        }
        if i % 5 == 0 {
            ans.push(String::from("Buzz"));
            continue;
        }
        ans.push(i.to_string());
    }
    return ans;
}
