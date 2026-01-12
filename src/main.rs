mod nondecreasing_subarray_operations;
use nondecreasing_subarray_operations::Solution;
pub fn main() {
    let nums = vec![14, 9, 4];
    println!("{}", Solution::count_non_decreasing_subarrays(nums, 1));
}
