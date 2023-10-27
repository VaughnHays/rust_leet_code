use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut num_hash: HashMap<i32, i32> = HashMap::new();
    // key = nums[i], value = i

    for i in 0..nums.len() {
        if num_hash.contains_key(&(target - nums[i])) {
            return vec![num_hash[&(target - nums[i])], i as i32];
        }
        num_hash.insert(nums[i], i as i32);
    }
    vec![-1]
}
