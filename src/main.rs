mod min_merge_cost;
use min_merge_cost::Solution;
pub fn main() {
    // let v = vec![vec![1, 3, 5], vec![2, 4], vec![6, 7, 8]];
    let n = 12;
    let mut v = vec![vec![]; n];
    for i in 0..2000 {
        let belonging = i % n;
        v[belonging].push(i as i32);
    }
    println!("{}", Solution::min_merge_cost(v));
}
