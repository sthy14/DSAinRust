use std::rc::Rc;
use std::cell::RefCell;

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug, Clone)]
struct Node{
    value: String,
    next: Link,
    prev: Link
}

#[derive(Debug, Clone)]
struct BetterTransactionLog {
    head: Link,
    tail: Link,
    pub length: u64
}

fn main() {
    println!("Hello, world!");
}
