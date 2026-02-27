mod min_reverse_operations;
use min_reverse_operations::Solution;
fn main() {
    let banned = vec![1, 2];
    println!("{:?}", Solution::min_reverse_operations(4, 0, banned, 4));
}
