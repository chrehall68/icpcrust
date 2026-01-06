mod minimum_subgraph;
use minimum_subgraph::Solution;
pub fn main() {
    let edges = vec![
        vec![0, 1, 2],
        vec![1, 2, 3],
        vec![1, 3, 5],
        vec![1, 4, 4],
        vec![2, 5, 6],
    ];
    let queries = vec![vec![2, 3, 4], vec![0, 2, 5]];
    println!("{:?}", Solution::minimum_weight(edges, queries));
}
