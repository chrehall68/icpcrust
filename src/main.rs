mod minimum_subgraph_path;
use minimum_subgraph_path::Solution;
pub fn main() {
    let edges = vec![
        vec![0, 2, 2],
        vec![0, 5, 6],
        vec![1, 0, 3],
        vec![1, 4, 5],
        vec![2, 1, 1],
        vec![2, 3, 3],
        vec![2, 3, 4],
        vec![3, 4, 2],
        vec![4, 5, 1],
    ];
    println!("{}", Solution::minimum_weight(6, edges, 0, 1, 5));
}
