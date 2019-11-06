/*
 * @lc app=leetcode id=155 lang=rust
 *
 * [155] Min Stack
 */

// @lc code=start
// use std::i32;

struct MinStack {
    pub stack: Vec<i32>,
    pub min: i32
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    /** initialize your data structure here. */
    fn new() -> Self {
        MinStack { stack: vec![], min: std::i32::MAX }
    }
    
    fn push(&mut self, x: i32) {
        if x <= self.min {
            self.stack.push(self.min);
            self.min = x;
        }
        self.stack.push(x);

    }
    
    fn pop(&mut self) {
        match self.stack.pop() {
            None => None,
            Some(val) => {
                if val == self.min {
                    match self.stack.pop() {
                        None => { self.min = std::i32::MAX; },
                        Some(val1) => { self.min = val1; }
                    };
                }
                Some(val)
            }
        };
    }
    
    fn top(&self) -> i32 {
        self.stack[self.stack.len()-1]
    }
    
    fn get_min(&self) -> i32 {
        self.min
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(x);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
// @lc code=end

