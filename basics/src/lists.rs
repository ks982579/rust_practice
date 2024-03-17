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
}
