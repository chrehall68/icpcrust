// interesting application of bfs
// by "removing" the implicit connections
// to avoid re-visiting nodes
use std::collections::{BTreeSet, VecDeque};
pub struct Solution;
impl Solution {
    pub fn min_operations(s: String, k: i32) -> i32 {
        let mut counts = vec![0, 0];
        for b in s.into_bytes() {
            counts[(b - b'0') as usize] += 1;
        }
        // we have x ones and y 0s
        // if k | y, then obviously just takes y / k moves
        // otherwise, we need to do some logic
        // suppose we have y < k 0s
        // we have the options:
        // - flip k 1's -> net k
        // - flip k-1 1's and 1 0's -> net k-2
        // - flip k-2 1's and 2 0's -> net k-4
        // - flip k-3 1's and 3 0's -> net k-6
        // ...
        // - flip k-y 1's and y 0's -> net k-2*y
        // we can do this in a bfs brute force, but that would be
        // O(Nk) which is at most O(N**2)
        // so instead we optimize by "removing" links
        // specifically, we take advantage of the fact that
        // k, k-2, k-4, k-6, ... all have the same parity
        // so we store unvisited nodes of parity 0, 1
        // and will remove those once we relax an edge
        // to prevent re-visiting them
        let (zeros, ones) = (counts[0], counts[1]);
        let n = zeros + ones;
        let mut parity_sets = [BTreeSet::new(), BTreeSet::new()];
        for i in 0..n {
            parity_sets[(i % 2) as usize].insert(i);
        }
        let mut distances = vec![-1; n as usize];
        let mut q = VecDeque::new();
        q.push_back(zeros);
        distances[zeros as usize] = 0;
        parity_sets[zeros as usize % 2].remove(&zeros);
        while distances[0] == -1
            && let Some(num_zeros) = q.pop_front()
        {
            let num_ones = n - num_zeros;
            // calculate max zeros after
            // to maximize zeros after, we flip as many 1's as possible
            let max_ones_flipped = num_ones.min(k);
            let consequent_zero_flips = k - max_ones_flipped;
            assert!(consequent_zero_flips <= num_zeros);
            let max_num_zeros_after = num_zeros + max_ones_flipped - consequent_zero_flips;
            // calculate min num zeros after
            // to minimize zeros, we flip as many 0's as possible
            let max_zeros_flipped = num_zeros.min(k);
            let consequent_one_flips = k - max_zeros_flipped;
            assert!(consequent_one_flips <= num_ones);
            let min_num_zeros_after = num_zeros - max_zeros_flipped + consequent_one_flips;
            // so now we have that range
            assert!(max_num_zeros_after % 2 == min_num_zeros_after % 2);
            let parity = max_num_zeros_after % 2;
            // so add any new nodes in that range
            let mut to_remove = Vec::new();
            for new_node in
                parity_sets[parity as usize].range(min_num_zeros_after..=max_num_zeros_after)
            {
                q.push_back(*new_node);
                to_remove.push(*new_node);
                distances[*new_node as usize] = distances[num_zeros as usize] + 1;
            }
            for node in to_remove {
                parity_sets[parity as usize].remove(&node);
            }
        }

        distances[0]
    }
}
