// 125. Valid Palindrome
pub fn is_palindrome(s: String) -> bool {
    let s_format: String = s
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() && c != &' ')
        .collect();

    s_format
        .bytes()
        .take(s_format.len() / 2)
        .eq(s_format.bytes().rev().take(s_format.len() / 2))
}
