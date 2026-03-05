mod almost_equal;
use almost_equal::Solution;
fn main() {
    let s = "deeeeddde".to_owned();
    let p = "eddd".to_owned();
    println!("{}", Solution::min_starting_index(s, p));
}
