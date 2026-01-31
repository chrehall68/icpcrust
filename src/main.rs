mod special_cht;
use special_cht::Solution;
fn main() {
    let nums = vec![20, 41, 13, 32];
    let cost = vec![14, 12, 7, 31];
    println!("{}", Solution::minimum_cost(nums, cost, 15));
}
