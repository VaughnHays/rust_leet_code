// 9. Palindrome Number
pub fn is_palindrome(x: i32) -> bool {
    let a = x.to_string();
    a.bytes()
        .take(a.len() / 2)
        .eq(a.bytes().rev().take(a.len() / 2))
}
