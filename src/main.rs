mod maximum_cyclic_partition_score;
use maximum_cyclic_partition_score::Solution;
fn main() {
    let nums = vec![7, 2, 4, 5, 11, 6, 1, 0];
    println!("{}", Solution::maximum_score(nums, 3));
}
