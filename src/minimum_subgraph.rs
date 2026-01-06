pub struct Solution;

const TIMES: usize = 20;
impl Solution {
    fn lca(depths: &Vec<i32>, lifts: &Vec<Vec<(usize, i64)>>, a: usize, b: usize) -> i64 {
        let (mut lower, mut higher) = if depths[a] > depths[b] {
            (a, b)
        } else {
            (b, a)
        };
        // move lower up until it's on the same level as higher
        let mut result = 0;
        for i in (0..TIMES).rev() {
            if depths[lower] - (1 << i) >= depths[higher] {
                // worth it to move up
                let info = lifts[lower][i];
                lower = info.0;
                result += info.1;
            }
        }
        // now they're either the same node, or they're not
        if lower == higher {
            // done
            return result;
        }
        // otherwise, we need to find their lca by moving up
        for i in (0..TIMES).rev() {
            if depths[lower] - (1 << i) >= 0 {
                let info_a = lifts[lower][i];
                let info_b = lifts[higher][i];
                if info_a.0 != info_b.0 {
                    // mismatch, so move up
                    lower = info_a.0;
                    higher = info_b.0;
                    result += info_a.1 + info_b.1;
                }
            }
        }
        // now the result is their parent
        assert_ne!(lower, higher);
        assert_eq!(lifts[lower][0].0, lifts[higher][0].0);
        result + lifts[lower][0].1 + lifts[higher][0].1
    }

    pub fn minimum_weight(edges: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        // construct graph
        let n = edges.len() + 1;
        // g[from] = (to, cost)
        let mut g = vec![vec![]; n];
        for e in edges {
            g[e[0] as usize].push((e[1] as usize, e[2] as i64));
            g[e[1] as usize].push((e[0] as usize, e[2] as i64));
        }
        // arbitrarily root the tree at node 0 and then figure out parents from that
        let mut cur_level = vec![0];
        let mut depths = vec![0; n];
        let mut parents = vec![n; n];
        // lifts is (node, cost)
        let mut lifts = vec![vec![]; n];
        while !cur_level.is_empty() {
            let mut next_level = Vec::new();
            for node in cur_level {
                for &(conn, weight) in g[node].iter() {
                    if conn != parents[node] {
                        // then node is parent of conn
                        parents[conn] = node;
                        lifts[conn].push((node, weight));
                        depths[conn] = depths[node] + 1;
                        next_level.push(conn);
                    }
                }
            }
            cur_level = next_level;
        }
        // now compute the 2**i'th parent using the 2**(i-1)'th parents
        for i in 1..TIMES {
            for node in 0..n {
                if lifts[node].len() >= i && lifts[lifts[node][i - 1].0].len() >= i {
                    let my_info = lifts[node][i - 1];
                    let parent_info = lifts[my_info.0][i - 1];
                    let new_info = (parent_info.0, my_info.1 + parent_info.1);
                    lifts[node].push(new_info);
                }
            }
        }
        // now handle queries
        let mut result = Vec::new();
        for query in queries {
            // just need to go from a->b, b->c, and then c->a
            let q: Vec<_> = query.into_iter().map(|v| v as usize).collect();
            let ab = Self::lca(&depths, &lifts, q[0], q[1]);
            let bc = Self::lca(&depths, &lifts, q[1], q[2]);
            let ca = Self::lca(&depths, &lifts, q[2], q[0]);
            result.push(((ab + bc + ca) / 2) as i32)
        }

        result
    }
}
