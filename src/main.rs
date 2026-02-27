mod equalize_string;
use equalize_string::Solution;
fn main() {
    let s = "11010".to_owned();
    let k = 4;
    println!("{}", Solution::min_operations(s, k));
}
