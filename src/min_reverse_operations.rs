// 2612 Minimum Reverse Operations
// another bfs with implicit edges
pub struct Solution;
use std::collections::{BTreeSet, VecDeque};
impl Solution {
    pub fn min_reverse_operations(n: i32, p: i32, banned: Vec<i32>, k: i32) -> Vec<i32> {
        // just bfs from p
        let mut parity_sets = [BTreeSet::new(), BTreeSet::new()];
        let banned: BTreeSet<_> = banned.into_iter().collect();
        for i in 0..n {
            if !banned.contains(&i) {
                parity_sets[(i % 2) as usize].insert(i);
            }
        }
        let mut distances = vec![-1; n as usize];
        distances[p as usize] = 0;
        let mut q = VecDeque::new();
        q.push_back(p);
        parity_sets[p as usize % 2].remove(&p);
        while let Some(i) = q.pop_front() {
            // yeah so everything reachable by placing the window
            // has the same parity
            let window_leftmost = (i - (k - 1)).max(0);
            let window_rightmost = i.min(n - 1 - (k - 1));
            let dist_leftmost = window_leftmost + (k - 1) - i;
            let dist_rightmost = i - window_rightmost;
            let left_reflection = window_leftmost + dist_leftmost;
            let right_reflection = window_rightmost + (k - 1) - dist_rightmost;
            let parity = left_reflection % 2;
            let mut to_remove = Vec::new();
            for &j in parity_sets[parity as usize].range(left_reflection..=right_reflection) {
                to_remove.push(j);
                distances[j as usize] = distances[i as usize] + 1;
                q.push_back(j);
            }
            for node in to_remove {
                parity_sets[parity as usize].remove(&node);
            }
        }

        distances
    }
}
