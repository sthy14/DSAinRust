//a list should contain a value and the pointer to next
// the next node could be null, rust doesn't have null. so need to use Option instead
/*
NOTE: the following def won't work:
struct Node {
    value: i32,
    next: Option<Node> !! recursive def without indirection, need to use box/rc/&
}
*/
// according to Hands on data structure, we are using Rc<RefCell>
//Storing each node item in a Rc<RefCell<T>> provides the ability to retrieve and
//replace data as needed (the internal mutability pattern)—crucial when executing
//operations on the list.
use std::cell::RefCell;
use std::rc::Rc;
type SingleLink = Option<Rc<RefCell<Node>>>; // good practice.
#[derive(Clone)]
struct Node {
    value: String,
    next: SingleLink
}

impl Node {
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new( Node {
            value: value,
            next: None
        }))
    }
}


// Alternatively, leetcode has the following def, this one doesn't have share control of head/tail:
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

struct TransactionLog {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
    pub length: i64
}

impl TransactionLog {
    pub fn new_empty() -> TransactionLog {
        TransactionLog {
            head: None,
            tail: None,
            length: 0
        }
    }

    pub fn append(&mut self, value: String) {
        let new = Node::new(value);
        match self.tail.take() {
            Some(old) => old.borrow_mut().next = Some(new.clone()),
            None => self.head = Some(new.clone())
        }
        self.length += 1;
        self.tail = Some(new);
    }

    pub fn pop(&mut self) -> Option<String> {
        self.head.take().map(|head| { // the head here is different from self.head
            // use map here to convert Option<Rc<RefCell<Node>>> to Option<String>
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take(); // I think take a Rc would just reduce one in refcount
            }
            // the previous iflet is equivalent to something like:
            /*
            match head.borrow_mut().next.take() {
                Some(next) => self.head = Some(next); // if head is linked to sth.
                _ => self.tail.take(); // this includes non Some(next) and None
                // this case means length = 1
            }
            */
            self.length -= 1;
            Rc::try_unwrap(head) // Rc<RefCell<Node>> => Result<RefCell<Node>, Rc<RefCell<Node>>>
                .ok()   //Result<RefCell<Node>> => Option<RefCell<Node>>
                .expect("Something is terribly wrong") // Option<RefCell<Node>> => RefCell<Node>
                .into_inner() // RefCell<Node> => Node
                .value // Node => Node.value
        })
    }
}

fn main() {
    println!("Hello, world!");
}
