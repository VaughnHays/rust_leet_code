use crate::non_leetcode::fizzbuzz_one_liner::fizzbuzz;

mod easy;
mod hard;
mod medium;
mod non_leetcode;

fn main() {
    let res = fizzbuzz(100);
    println!["{:?}", res]
}
