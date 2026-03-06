// 2791
// technically, could've been solved with just the hashmap
// since the xor'ing ensure that nodes cancel out
// but this is a good exercise in small to large implementation
// so I've included it here.
pub struct Solution;
use std::collections::HashMap;

const NUM_BITS: usize = 27;
#[derive(Debug)]
struct HashCounts {
    counts: HashMap<i32, i64>,
    offset: i32,
}
impl HashCounts {
    fn count_palindromes_helper(&self, offset: i32) -> i64 {
        // count how many are even length palindromes
        let mut total = *self.counts.get(&offset).unwrap_or(&0);
        // and how many are odd length palindromes
        for set_bit in 0..NUM_BITS {
            let search = offset ^ (1 << set_bit);
            total += *self.counts.get(&search).unwrap_or(&0);
        }
        total
    }
    pub fn count_palindromes(&self) -> i64 {
        self.count_palindromes_helper(self.offset)
    }
    pub fn count_palindromes_with(&mut self, extra_offset: i32) -> i64 {
        self.count_palindromes_helper(self.offset ^ extra_offset)
    }
    pub fn new() -> Self {
        HashCounts {
            counts: HashMap::new(),
            offset: 0,
        }
    }
    pub fn len(&self) -> usize {
        self.counts.len()
    }
}
impl Solution {
    fn recurse(g: &[Vec<(u8, usize)>], i: usize) -> (i64, HashCounts) {
        // base case:
        if g[i].len() == 0 {
            let mut my_counts = HashCounts::new();
            my_counts.counts.insert(0, 1);
            (0, my_counts)
        } else {
            // recursive case
            // we need to ask all the children
            let mut total = 0;
            let mut my_counts = HashCounts::new();
            for &(letter, child) in g[i].iter() {
                // so first we recursively solve + count
                let (child_total, mut child_counts) = Self::recurse(g, child);
                total += child_total;
                // and then we add the letter
                let shift = letter - b'a';
                let extra_offset = 1 << shift;
                child_counts.offset ^= extra_offset;
                total += child_counts.count_palindromes();
                // and then small to large merge
                let (small, mut large);
                if my_counts.len() < child_counts.len() {
                    small = my_counts;
                    large = child_counts;
                } else {
                    small = child_counts;
                    large = my_counts;
                }
                // count first
                for (&key, &count) in small.counts.iter() {
                    // actual key is the key after applying small's lazy offset
                    let actual_key = key ^ small.offset;
                    total += large.count_palindromes_with(actual_key) * count;
                }
                // then actually merge
                for (&key, &count) in small.counts.iter() {
                    // actual key is the key after applying small's lazy offset
                    let actual_key = key ^ small.offset;
                    let large_key = actual_key ^ large.offset;
                    *large.counts.entry(large_key).or_default() += count;
                }

                // finish merge
                my_counts = large;
            }
            // and insert a 0 here too
            *my_counts.counts.entry(my_counts.offset).or_default() += 1;

            (total, my_counts)
        }
    }
    pub fn count_palindrome_paths(parent: Vec<i32>, s: String) -> i64 {
        // i think small to large merging with lazy
        // hahmap counting works
        // at every node, we will:
        // - count how many paths form palindromes in just its children
        //  (this gives us their ending counts too)
        // - count how many go through this node (using the small to large merging)
        let s = s.into_bytes();
        let mut g = vec![vec![]; parent.len()];
        for i in 1..parent.len() {
            let p = parent[i];
            g[p as usize].push((s[i], i));
        }
        Self::recurse(&g, 0).0
    }
}
