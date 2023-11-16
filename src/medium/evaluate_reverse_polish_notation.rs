// 150. Evaluate Reverse Polish Notation

pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack: Vec<i32> = Vec::new();
    for token in tokens {
        if let Ok(num) = token.parse::<i32>() {
            stack.push(num);
        } else {
            let b = stack.pop().unwrap();
            let a = stack.pop().unwrap();
            stack.push(match token.as_str() {
                "+" => a + b,
                "-" => a - b,
                "*" => a * b,
                "/" => a / b,
                err => panic!["bad stack item {}", err],
            })
        }
    }
    stack.pop().unwrap()
}
