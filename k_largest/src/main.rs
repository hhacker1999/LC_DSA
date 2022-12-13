use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(PartialEq, Debug)]
struct MinNonNan(i32);

impl Eq for MinNonNan {}

impl PartialOrd for MinNonNan {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl Ord for MinNonNan {
    fn cmp(&self, other: &MinNonNan) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn main() {
    println!(
        "ans is {:?}, ",
        find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4)
    );
}

fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let mut min_heap: BinaryHeap<MinNonNan> = BinaryHeap::new();
    let size: usize = nums.len();
    for i in 0..k {
        min_heap.push(MinNonNan(nums[i as usize]));
    }
    for i in k as usize..size {
        if nums[i as usize] > min_heap.peek().unwrap().0 {
            min_heap.pop();
            min_heap.push(MinNonNan(nums[i]));
        }
    }
    println!("min heap is {:?}", &min_heap);
    return min_heap.pop().unwrap().0;
}
