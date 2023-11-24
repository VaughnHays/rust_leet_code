use crate::medium::generate_parentheses::generate_parenthesis;

mod easy;
mod hard;
mod medium;
mod non_leetcode;

fn main() {
    let res = generate_parenthesis(3);
    println!["{:?}", res]
}
