pub struct MinStack {
    items: Vec<i32>,
    min_vals: Vec<i32>,
}

impl MinStack {
    pub fn new() -> Self {
        MinStack {
            items: vec![],
            min_vals: vec![],
        }
    }

    pub fn push(&mut self, val: i32) {
        self.items.push(val);
        if val <= self.min_vals.last().cloned().unwrap_or(i32::MAX) {
            self.min_vals.push(val);
        }
    }

    pub fn pop(&mut self) {
        if self.items.pop().unwrap() == self.get_min() {
            self.min_vals.pop();
        }
    }

    pub fn top(&self) -> i32 {
        self.items.last().cloned().unwrap()
    }

    pub fn get_min(&self) -> i32 {
        self.min_vals.last().cloned().unwrap()
    }
}
