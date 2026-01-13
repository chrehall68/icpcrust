mod xor_after_queries;
use xor_after_queries::Solution;
pub fn main() {
    let nums = vec![551, 78, 927];
    let queries = vec![
        vec![2, 2, 3, 10],
        vec![2, 2, 3, 1],
        vec![0, 0, 1, 7],
        vec![2, 2, 2, 9],
        vec![2, 2, 3, 12],
        vec![1, 2, 2, 1],
        vec![0, 1, 3, 6],
        vec![1, 1, 1, 8],
        vec![2, 2, 3, 6],
        vec![0, 1, 2, 6],
        vec![0, 2, 2, 14],
        vec![2, 2, 2, 20],
        vec![2, 2, 1, 5],
        vec![2, 2, 2, 20],
        vec![0, 2, 2, 2],
    ];
    println!("{}", Solution::xor_after_queries(nums, queries));
}
