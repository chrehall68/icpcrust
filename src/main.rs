mod self_crossing;
use self_crossing::Solution;
pub fn main() {
    let distance = vec![2, 1, 1, 2];
    println!("{}", Solution::is_self_crossing(distance));
}
