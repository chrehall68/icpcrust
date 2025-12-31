use std::{cmp::Reverse, collections::BinaryHeap};
pub struct Solution;
impl Solution {
    fn dijkstra(g: &Vec<Vec<(usize, i64)>>, source: usize) -> Vec<i64> {
        let mut min_cost = vec![i64::MAX; g.len()];
        min_cost[source] = 0;
        // pq is max heap, so have to use reverse
        let mut pq = BinaryHeap::new();
        pq.push((Reverse(0), source));
        while let Some((Reverse(cost), node)) = pq.pop() {
            if cost <= min_cost[node] && cost != i64::MAX {
                // then we explore this
                for &(conn, extra_cost) in g[node].iter() {
                    let new_cost = cost + extra_cost;
                    if new_cost < min_cost[conn] {
                        min_cost[conn] = new_cost;
                        pq.push((Reverse(new_cost), conn));
                    }
                }
            }
        }

        min_cost
    }
    pub fn minimum_weight(n: i32, edges: Vec<Vec<i32>>, src1: i32, src2: i32, dest: i32) -> i64 {
        // the resulting subgraph will be acyclic
        // suppose it was cyclic. Then, there are 2 ways, so we can remove an edge
        // ok so dest should be a sink; only should have edges into it
        // and at least one of s1, s2 should be sources (only have edges out)
        // and also we should only have one edge out of each node in the path
        // ok so they share a common point
        // from that common point, they take the shortest path to the destination
        // construct graph and graph reverse
        let mut g = vec![Vec::new(); n as usize];
        let mut gr = vec![Vec::new(); n as usize];
        for edge in edges {
            let (from, to, cost) = (edge[0], edge[1], edge[2]);
            g[from as usize].push((to as usize, cost as i64));
            gr[to as usize].push((from as usize, cost as i64));
        }
        let to_dest = Self::dijkstra(&gr, dest as usize);
        let from_src1 = Self::dijkstra(&g, src1 as usize);
        let from_src2 = Self::dijkstra(&g, src2 as usize);
        // so now they need to come together at some point. Just check all
        let best = (0..n as usize)
            .map(|together| {
                to_dest[together]
                    .checked_add(from_src1[together])
                    .and_then(|v| v.checked_add(from_src2[together]))
                    .unwrap_or(i64::MAX)
            })
            .min()
            .unwrap();
        if best == i64::MAX { -1 } else { best }
    }
}
