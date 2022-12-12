fn main() {
    let mut array: Vec<i32> = vec![20, 40, 34, 146, 646, 1, 0, 64, 34, 88, 73];
    println!("Original array is {:?}", &array);
    let size: usize = array.len();
    for i in (0..=size / 2).rev() {
        heapify(&mut array, i, None);
    }
    println!("Heapyfied array is {:?}", &array);
    sort_heap(&mut array);
    println!("Sorted Heapyfied array is {:?}", &array);
}

#[derive(Debug)]
enum HeapType {
    MinHeap,
    MaxHeap,
}

impl HeapType {
    fn new(is_max: bool) -> HeapType {
        if is_max {
            return HeapType::MaxHeap;
        }
        return HeapType::MinHeap;
    }
}

#[derive(Debug)]
struct Heap {
    value: Vec<i32>,
    heap_type: HeapType,
}

impl Heap {
    fn new(heap_type: HeapType) -> Heap {
        Heap {
            value: vec![0],
            heap_type,
        }
    }

    fn insert(&mut self, value: i32) {
        self.value.push(value);
        let length: usize = self.value.len();
        let mut value_index: usize = length - 1;
        while value_index > 1 {
            let parent_index: usize = value_index / 2;
            let parent: i32 = self.value[parent_index];
            let value: i32 = self.value[value_index];
            match self.heap_type {
                HeapType::MaxHeap => {
                    if parent < value {
                        self.value.swap(parent_index, value_index);
                        value_index = parent_index;
                    } else {
                        return;
                    }
                }
                HeapType::MinHeap => {
                    if parent > value {
                        self.value.swap(parent_index, value_index);
                        value_index = parent_index;
                    } else {
                        return;
                    }
                }
            }
        }
    }

    fn delete(&mut self) {
        if self.value.len() == 1 || self.value.len() == 2 {
            return;
        }
        let size: usize = self.value.len() - 2;
        self.value[1] = self.value.pop().unwrap();
        let mut index: usize = 1;
        while index <= size {
            let left_child_index: usize = index * 2;
            let right_child_index: usize = index * (2 + 1);
            match self.heap_type {
                HeapType::MaxHeap => {
                    if left_child_index <= size && self.value[index] < self.value[left_child_index]
                    {
                        self.value.swap(left_child_index, index);
                        index = left_child_index;
                    } else if right_child_index <= size
                        && self.value[index] < self.value[right_child_index]
                    {
                        self.value.swap(index, right_child_index);
                        index = right_child_index
                    } else {
                        break;
                    }
                }
                HeapType::MinHeap => {
                    if left_child_index <= size && self.value[index] > self.value[left_child_index]
                    {
                        self.value.swap(left_child_index, index);
                        index = left_child_index;
                    } else if right_child_index <= size
                        && self.value[index] > self.value[right_child_index]
                    {
                        self.value.swap(index, right_child_index);
                        index = right_child_index;
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

fn heapify(arr: &mut Vec<i32>, index: usize, new_size: Option<usize>) {
    let size: usize = new_size.unwrap_or(arr.len());
    let mut largest: usize = index;
    let left_child_index: usize = (2 * index) + 1;
    let right_child_index: usize = left_child_index + 1;
    if left_child_index < size && arr[left_child_index] > arr[largest] {
        largest = left_child_index;
    }
    if right_child_index < size && arr[right_child_index] > arr[largest] {
        largest = right_child_index;
    }
    if largest != index {
        arr.swap(index, largest);
        heapify(arr, largest, new_size);
    } else {
        return;
    }
}

fn sort_heap(arr: &mut Vec<i32>) {
    let size: usize = arr.len();
    let mut swap_index: usize = size - 1;
    while swap_index > 0 {
        arr.swap(0, swap_index);
        swap_index -= 1;
        heapify(arr, 0, Some(swap_index + 1));
    }
}
