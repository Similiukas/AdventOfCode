use regex::Captures;

#[derive(Clone)]
pub struct Stack<T> {
    stack: Vec<T>
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        self.stack.push(item)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    pub fn peek(&mut self) -> Option<&T> {
        self.stack.last()
    }

    // pub fn copy(&mut self) -> Stack<T> {
    //     self.stack.clone();
    // }
}

// impl Copy for Stack<char> {
//     fn copy(&self) -> Stack<T> {
//         self.stack.clone();
//     }
// }

pub fn re_to_int(captures: &Captures, group_index: usize) -> usize {
    captures.get(group_index).unwrap().as_str().parse::<usize>().unwrap_or(0)
}