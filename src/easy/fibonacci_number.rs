use std::collections::HashMap;
// 509. Fibonacci Number

pub fn fib(n: i32) -> i32 {
    let mut memo: HashMap<i32, i32> = HashMap::new();
    memo.insert(0, 0);
    memo.insert(1, 1);
    return fib_memo(n, &mut memo);
}

pub fn fib_memo(n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
    if memo.contains_key(&n) {
        return *memo.get(&n).unwrap();
    }
    let new_n: i32 = fib_memo(n - 1, memo) + fib_memo(n - 2, memo);
    memo.insert(n, new_n);
    new_n
}
