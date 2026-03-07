// repeated substrings
// kattis problem
// https://open.kattis.com/problems/substrings
// suffix array and lcp array

use std::io::stdin;
pub struct Solution;

fn sort_cyclic_shifts(s: &[u8]) -> Vec<usize> {
    // counting sort by the length-1 strings
    // (ie by the characters themselves)
    let classes: usize = s.len().max(256);
    let mut counts = vec![0; classes];
    let mut eq_cls = vec![0; s.len()];
    for i in 0..s.len() {
        eq_cls[i] = s[i] as usize;
        counts[eq_cls[i]] += 1;
    }
    for i in 1..classes {
        counts[i] += counts[i - 1];
    }
    // order[order_idx] = original_idx
    let mut order = vec![0; s.len()];
    for i in (0..s.len()).rev() {
        let cls = eq_cls[i];
        let idx = &mut counts[cls];
        *idx -= 1;
        order[*idx] = i;
    }
    // now dp based off of that
    let mut prev_l = 1;
    let mut next_eq_class = vec![0; s.len()];
    let mut next_order = vec![0; s.len()];
    while prev_l < s.len() {
        // so now, we have, for every index i
        // the tuple (my_order_idx, order_idx_of(i+prev_l))
        // so we stably sort by the latter, then the former
        // sort by order_idx_of(i+prev_l)
        // we can do that by just shifting all the indices
        for i in 0..s.len() {
            let original_idx = order[i];
            let start = (original_idx + s.len() - prev_l) % s.len();
            next_order[i] = start;
        }
        // then sort by my_order_idx
        // we'll do this using the eq_classes and counting sort
        counts.fill(0);
        for i in 0..s.len() {
            let original_idx = next_order[i];
            let cls = eq_cls[original_idx];
            counts[cls] += 1;
        }
        for i in 1..classes {
            counts[i] += counts[i - 1];
        }
        for i in (0..s.len()).rev() {
            let original_idx = next_order[i];
            let cls = eq_cls[original_idx];
            let idx = &mut counts[cls];
            *idx -= 1;
            order[*idx] = original_idx;
        }
        // now calculate new eq classes
        let mut num_classes = 0;
        let mut prev_cls = (eq_cls[order[0]], eq_cls[(order[0] + prev_l) % s.len()]);
        next_eq_class[order[0]] = 0;
        for i in 1..s.len() {
            let original_idx = order[i];
            let my_cls = (
                eq_cls[original_idx],
                eq_cls[(original_idx + prev_l) % s.len()],
            );
            if prev_cls != my_cls {
                num_classes += 1;
            }
            prev_cls = my_cls;
            next_eq_class[original_idx] = num_classes;
        }
        // advance
        std::mem::swap(&mut eq_cls, &mut next_eq_class);
        prev_l *= 2;
    }
    order
}
fn suffix_array(s: &mut String) -> Vec<usize> {
    s.push('\0');
    let mut v = sort_cyclic_shifts(s.as_bytes());
    v.remove(0);
    s.pop();
    v
}
fn lcp_array(s: &[u8], suffix_array: &[usize]) -> Vec<usize> {
    // rank stores index in the suffix array
    // that way we can quickly figure out what's the next suffix
    let mut rank = vec![0; s.len()];
    for i in 0..s.len() {
        rank[suffix_array[i]] = i;
    }
    // k is current lcp
    let mut k = 0;
    let mut lcp = vec![0; s.len() - 1];
    for i in 0..s.len() {
        // case of no lcp since it's the last in the suffix array
        if rank[i] == s.len() - 1 {
            k = 0;
            continue;
        }
        // advance (compare against next item in suffix array)
        let j = suffix_array[rank[i] + 1];
        while i + k < s.len() && j + k < s.len() && s[i + k] == s[j + k] {
            k += 1;
        }
        lcp[rank[i]] = k;
        // as we move forward, we know that the suffix can't have
        // this current letter, so we decrement by one
        if k > 0 {
            k -= 1;
        }
    }

    lcp
}

impl Solution {
    pub fn main() {
        let mut inp = String::new();
        let _ = stdin().read_line(&mut inp).unwrap();
        let t: usize = inp.trim().parse().unwrap();
        for _ in 0..t {
            inp.clear();
            let _ = stdin().read_line(&mut inp).unwrap();
            let mut s = inp.trim().to_owned();
            let suffix_arr = suffix_array(&mut s);
            let lcp_arr = lcp_array(s.as_bytes(), &suffix_arr);
            let mut prev = 0;
            let mut total = 0;
            for lcp in lcp_arr {
                if lcp >= prev {
                    total += lcp - prev;
                }
                prev = lcp;
            }
            println!("{total}");
        }
    }
}
