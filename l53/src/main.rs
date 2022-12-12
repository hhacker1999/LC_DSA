#![allow(unused_variables)]
#![allow(unused_mut)]

fn main() {
    println!("{}", trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]));
}

fn trap(height: Vec<i32>) -> i32 {
    let mut ans: i32 = 0;
    let mut start: usize = 0;
    let mut end: usize = height.len() - 1;
    let mut last_big: i32 = 0;
    while start < end - 1 {
        let min: i32 = if height[start] < height[end] {
            height[start]
        } else {
            height[end]
        };
        if min > last_big {
            let mut max_possible: i32 = min * ((end - start) as i32 - 1);
            for i in start + 1..end {
                if height[i] >= min {
                    max_possible -= min;
                } else {
                    if height[i] < last_big {
                        max_possible -= last_big;
                    } else {
                        max_possible -= height[i];
                    }
                }
            }
            last_big = min;
            ans += max_possible;
        }
        if height[start] < height[end] {
            start += 1;
        } else {
            end -= 1;
        }
    }
    ans
}

// [3, -1, -4]
fn max_product(nums: Vec<i32>) -> i32 {
    let mut last_max: i32 = *nums.first().unwrap();
    let mut ans: i32 = 1;
    for i in nums.iter() {
        ans = ans * *i;
        // println!("{}", &ans);
        if ans > last_max {
            last_max = ans;
        }
        if ans <= 0 {
            ans = 1;
        }
    }
    return last_max;
}

fn max(arr: &Vec<i32>) -> i32 {
    let mut ans: i32 = *arr.first().unwrap();
    for i in arr.iter() {
        if *i > ans {
            ans = *i;
        }
    }
    ans
}

fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut last_max: i32 = *nums.first().unwrap();
    let mut ans: i32 = 0;
    for i in nums.iter() {
        let curr: i32 = ans + *i;
        ans += *i;
        if last_max < ans {
            last_max = ans;
        }
        if curr < 0 {
            ans = 0;
        }
    }
    last_max
    // for i in nums.iter() {
    //     if *i > 0 {
    //         ans_arr.push(*i);
    //     }
    // }
    // if ans_arr.is_empty() {
    //     return max_negative(&nums);
    // }
    // return sum(&ans_arr);
}

fn sum(arr: &Vec<i32>) -> i32 {
    let mut ans: i32 = 0;
    for i in arr {
        println!("reached here");
        ans += *i;
    }
    ans
}

fn max_negative(arr: &Vec<i32>) -> i32 {
    let mut ans: i32 = *arr.first().unwrap();
    for i in arr {
        if *i > ans {
            ans = *i;
        }
    }
    ans
}
