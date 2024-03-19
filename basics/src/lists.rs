use std::cell::{RefCell, RefMut};
use std::rc::Rc;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct StackNode<T> {
    value: T,
    previous_node: Option<Box<StackNode<T>>>,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Stack<T> {
    top: Option<Box<StackNode<T>>>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { top: None }
    }
    pub fn push(&mut self, val: T) {
        let mut new_node: StackNode<T> = StackNode {
            value: val,
            previous_node: None,
        };

        match self.top.take() {
            Some(s) => {
                new_node.previous_node = Some(Box::new(*s));
                self.top = Some(Box::new(new_node));
            }
            None => {
                self.top = Some(Box::new(new_node));
            }
        }
    }
    pub fn pop(&mut self) -> Option<T> {
        // self.top will now be None
        let mut tippy_top = self.top.take();
        match tippy_top {
            None => {
                self.top = None;
                return None;
            }
            Some(box_data) => {
                // return tippytop value and store it's prev_node as Stack
                let mut raw_data: StackNode<T> = *box_data;
                self.top = raw_data.previous_node.take();
                return Some(raw_data.value);
            }
        }
    }
}

// -----------------------------------------------
// --------------- Queues ------------------------
// -----------------------------------------------

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct NonPriorityQueueNode<T> {
    pub value: T,
    pub next: Option<Rc<RefCell<NonPriorityQueueNode<T>>>>,
}

impl<T> NonPriorityQueueNode<T> {
    fn new(val: T) -> Self {
        Self {
            value: val,
            next: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct NonPriorityQueue<T> {
    pub head: Option<Rc<RefCell<NonPriorityQueueNode<T>>>>,
    pub tail: Option<Rc<RefCell<NonPriorityQueueNode<T>>>>,
}

impl<'a, T> NonPriorityQueue<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        NonPriorityQueue {
            head: None,
            tail: None,
        }
    }
    pub fn enqueue(&mut self, val: T) {
        let new_node = Rc::new(RefCell::new(NonPriorityQueueNode::new(val)));
        match self.tail.take() {
            None => {
                self.tail = Some(Rc::clone(&new_node));
                self.head = Some(Rc::clone(&new_node));
            }
            Some(ref_cnt) => {
                let mut end_elm: RefMut<'_, NonPriorityQueueNode<T>> = (*ref_cnt).borrow_mut();
                (*end_elm).next = Some(Rc::clone(&new_node));
                self.tail = Some(Rc::clone(&new_node));
            }
        }
    }
    pub fn dequeue(&mut self) -> Option<T> {
        match self.head.take() {
            None => {
                return None;
            }
            Some(old_head) => {
                self.head = (*old_head).borrow_mut().next.take();
                if let None = self.head {
                    self.tail = None;
                }
                let rv: T = (*old_head).borrow().value.clone();
                Some(rv)
            }
        }
    }
}

// -----------------------------------------------
// ---------------- Tests ------------------------
// -----------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_stack() {
        let actual: Stack<i8> = Stack::new();
        let excpected: Stack<i8> = Stack { top: None };
        assert!(actual == excpected);
    }
    #[test]
    fn test_push_to_stack() {
        let obj1: StackNode<i32> = StackNode {
            value: 69,
            previous_node: None,
        };
        let obj2: StackNode<i32> = StackNode {
            value: 42,
            previous_node: Some(Box::new(obj1)),
        };
        let expected: Stack<i32> = Stack {
            top: Some(Box::new(obj2)),
        };
        let mut actual: Stack<i32> = Stack::new();
        actual.push(69);
        actual.push(42);
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_pop_from_stack() {
        // test value poped and stack after
        let obj1: StackNode<i32> = StackNode {
            value: 69,
            previous_node: None,
        };
        let obj2: StackNode<i32> = StackNode {
            value: 42,
            previous_node: Some(Box::new(obj1.clone())),
        };
        let mut actual_stack: Stack<i32> = Stack {
            top: Some(Box::new(obj2)),
        };
        let actual_value: Option<i32> = actual_stack.pop();
        let expected_value: Option<i32> = Some(42);
        let expected_stack: Stack<i32> = Stack {
            top: Some(Box::new(obj1.clone())),
        };
        assert_eq!(actual_value, expected_value);
        assert_eq!(actual_stack, expected_stack);
    }
    #[test]
    fn test_make_non_priority_queue() {
        let actual: NonPriorityQueue<i8> = NonPriorityQueue::new();
        let excpected: NonPriorityQueue<i8> = NonPriorityQueue {
            head: None,
            tail: None,
        };
        assert!(actual == excpected);
    }
    #[test]
    fn test_npq_enqueue() {
        let val: i32 = 42;
        let new_node = Rc::new(RefCell::new(NonPriorityQueueNode::new(val)));
        let mut actual: NonPriorityQueue<i32> = NonPriorityQueue::new();
        actual.enqueue(val);
        let expected: NonPriorityQueue<i32> = NonPriorityQueue {
            head: Some(Rc::clone(&new_node)),
            tail: Some(Rc::clone(&new_node)),
        };
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_npq_dequeue() {
        let mut actual: NonPriorityQueue<i32> = NonPriorityQueue::new();
        actual.enqueue(42);
        let actual_value = actual.dequeue().unwrap();
        let expected_val: i32 = 42;
        let expected_q: NonPriorityQueue<i32> = NonPriorityQueue {
            head: None,
            tail: None,
        };
        assert_eq!(actual_value, expected_val);
        assert_eq!(actual, expected_q);
    }
}
