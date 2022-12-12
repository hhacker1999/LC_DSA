#![warn(unused_variables)]
#![warn(dead_code)]

use std::rc::Rc;

fn main() {
    let new_tree: Btree<i32> = Btree::<i32>::new(0);
    println!("{:?}", &new_tree);
}

#[derive(Debug, Clone)]
struct Node<T> {
    value: T,
    left: Option<Rc<Node<T>>>,
    right: Option<Rc<Node<T>>>,
    parent: Option<Rc<Node<T>>>,
}

#[derive(Debug)]
struct Btree<T> {
    root: Rc<Node<T>>,
    last: Rc<Node<T>>,
    depth: usize,
}

impl<T> Btree<T> {
    fn new(value: T) -> Btree<T> {
        let node = Rc::new(Node::<T> {
            value,
            left: None,
            right: None,
            parent: None,
        });

        Btree {
            root: node.clone(),
            last: node,
            depth: 0,
        }
    }

    fn insert(&mut self, node: Node<T>) {
        let root_node = self.root.clone();
    }
}
