use std::collections::HashSet;

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let num_set: HashSet<i32> = nums.into_iter().collect();
    let mut longest_cons: i32 = -1;
    let mut current = 1;

    for num in &num_set {
        if !num_set.contains(&(num - 1)) {
            let mut current_num = num.clone();
            while num_set.contains(&(current_num + 1)) {
                current += 1;
                current_num += 1;
            }
        }
        if current > longest_cons {
            longest_cons = current;
        }
        current = 1;
    }
    longest_cons
}
