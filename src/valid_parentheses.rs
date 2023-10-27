pub fn is_valid(s: String) -> bool {
    if s.len() & 1 == 1 { return false; }
    let mut stack:Vec<char> = vec![];

    for char in s.chars() {
        match char {
            '(' | '{' | '[' => stack.push(char),
            _ => match stack.pop() {
                Some('(') if char == ')' => (),
                Some('{') if char == '}' => (),
                Some('[') if char == ']' => (),
                _ => return false
                }
        }
    }
    stack.is_empty()
}
