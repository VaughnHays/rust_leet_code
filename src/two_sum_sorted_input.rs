use std::cmp::Ordering;

pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut idx_low: usize = 0;
    let mut idx_high: usize = numbers.len() - 1;

    while idx_low < idx_high {
        match (numbers[idx_low] + numbers[idx_high]).cmp(&target) {
            Ordering::Less => idx_low += 1,
            Ordering::Greater => idx_high -= 1,
            Ordering::Equal => return vec![idx_low as i32 + 1, idx_high as i32 + 1],
        }
    }
    unreachable!()
}
