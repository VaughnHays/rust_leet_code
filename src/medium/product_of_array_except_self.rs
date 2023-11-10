// 238. Product of Array Except Self
pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let size = nums.len();
    let mut prefix: i32 = 1;
    let mut postfix: i32 = 1;
    let mut res: Vec<i32> = vec![1; size];

    for i in 0..size {
        res[i] *= prefix;
        prefix *= nums[i];
        res[size - (i + 1)] *= postfix;
        postfix *= nums[size - (i + 1)];
    }
    res
}
