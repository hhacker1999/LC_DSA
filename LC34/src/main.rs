fn main() {
    println!("vec is {:?}", search_range(vec![1], 0));
}
fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut ans: Vec<i32> = vec![-1, -1];
    if nums.is_empty() {
        return ans;
    }
    let mut start: usize = 0;
    let mut end: usize = nums.len() - 1;
    while start <= end {
        if nums[start] == target {
            ans[0] = start as i32;
        }
        if nums[end] == target {
            ans[1] = end as i32;
        }
        if ans[0] != -1 && ans[1] != -1 {
            break;
        } else if ans[0] != -1 {
            end -= 1;
        } else if ans[1] != -1 {
            start += 1;
        } else {
            start += 1;
            end = {
                if end == 0 {
                    0
                } else {
                    end - 1
                }
            };
        }
    }
    return ans;
}
