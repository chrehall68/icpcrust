use std::ops::{Add, Mul, Sub};

// ==============================
// ModNum section
// ==============================
type T = [i64; 3];
const MODS: T = [1_000_000_007, 830_258_441, 852_069_347];
#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
struct ModNum {
    vals: T,
}
impl ModNum {
    pub fn new(val: i64) -> Self {
        let mut vals = MODS.clone();
        for i in 0..vals.len() {
            vals[i] = val % vals[i];
        }
        ModNum { vals }
    }
    pub fn pow(&self, mut exp: usize) -> Self {
        // binary exponentiation
        let mut result = ModNum::new(1);
        let mut cur_pow = self.clone();
        while exp > 0 {
            if exp & 1 == 1 {
                result = result * cur_pow;
            }
            cur_pow = cur_pow * cur_pow;
            exp >>= 1;
        }
        result
    }
    fn modinv_helper(val: i64, c: i64) -> i64 {
        if val <= 1 {
            return val;
        }
        c - c / val * Self::modinv_helper(c % val, c) % c
    }
    pub fn modinv(&self) -> Self {
        let mut new_vals = self.vals.clone();
        for i in 0..new_vals.len() {
            new_vals[i] = Self::modinv_helper(new_vals[i], MODS[i]);
        }
        ModNum { vals: new_vals }
    }
}
impl Mul for ModNum {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut new_vals: T = self.vals.clone();
        for i in 0..new_vals.len() {
            new_vals[i] *= rhs.vals[i];
            new_vals[i] %= MODS[i];
        }
        ModNum { vals: new_vals }
    }
}
impl Mul<i64> for ModNum {
    type Output = Self;
    fn mul(self, rhs: i64) -> Self::Output {
        let mut new_vals: T = self.vals.clone();
        for i in 0..new_vals.len() {
            new_vals[i] *= rhs;
            new_vals[i] %= MODS[i];
        }
        ModNum { vals: new_vals }
    }
}
impl Add<i64> for ModNum {
    type Output = Self;
    fn add(self, rhs: i64) -> Self::Output {
        let mut new_vals: T = self.vals.clone();
        for i in 0..new_vals.len() {
            new_vals[i] += rhs;
            new_vals[i] %= MODS[i];
        }
        ModNum { vals: new_vals }
    }
}
impl Sub for ModNum {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut new_vals = self.vals.clone();
        for i in 0..new_vals.len() {
            new_vals[i] -= rhs.vals[i];
            new_vals[i] += MODS[i];
            new_vals[i] %= MODS[i];
        }
        ModNum { vals: new_vals }
    }
}
// ==============================
// Trie section
// ==============================
type NodeId = usize;
struct TrieNode {
    to: [Option<NodeId>; 26],
    idxs: Vec<usize>,
}
impl TrieNode {
    pub fn new() -> Self {
        TrieNode {
            to: [None; 26],
            idxs: Vec::new(),
        }
    }
}
struct Trie {
    // rooted at 0
    nodes: Vec<TrieNode>,
}
impl Trie {
    const ROOT: usize = 0;
    pub fn new() -> Self {
        let root = TrieNode::new();
        Trie { nodes: vec![root] }
    }

    pub fn insert(&mut self, word: &str, word_idx: usize) {
        let mut cur = Trie::ROOT;
        for b in word.as_bytes() {
            let key = (*b - b'a') as usize;
            if let None = self.nodes[cur].to[key] {
                self.nodes[cur].to[key] = Some(self.nodes.len());
                self.nodes.push(TrieNode::new());
            }
            cur = self.nodes[cur].to[key].unwrap();
        }
        self.nodes[cur].idxs.push(word_idx);
    }
}

// ==============================
// Algorithm section
// ==============================
pub struct Solution;
impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        // build tries
        let mut root = Trie::new();
        let mut rev_root = Trie::new();
        for (i, word) in words.iter().enumerate() {
            root.insert(&word, i);
            rev_root.insert(&word.chars().rev().collect::<String>(), i);
        }
        // consider each word
        let mut result = Vec::new();
        const RADIX: i64 = 27;
        for (word_idx, word) in words.into_iter().enumerate() {
            // rolling hash to compare sections in O(1) time
            // prefix_hash[i] = hash of word[0..i] exclusive
            let mut prefix_hash = vec![ModNum::new(0); word.len() + 1];
            for (i, b) in word.as_bytes().iter().enumerate() {
                // value from 1-26
                let key = (*b - b'a') as i64 + 1;
                prefix_hash[i + 1] = prefix_hash[i] * RADIX + key;
            }
            // suffix_rev_hash[i] = hash of word[i..]
            let mut suffix_rev_hash = vec![ModNum::new(0); word.len() + 1];
            for (i, b) in word.as_bytes().iter().enumerate().rev() {
                let key = (*b - b'a') as i64 + 1;
                suffix_rev_hash[i] = suffix_rev_hash[i + 1] * RADIX + key;
            }
            // so now check if this word can act as the first part
            // ie if this word is L + R where L_rev in words and R is a palindrome
            let mut rev_idx = Some(0);
            let mut shifter = ModNum::new(RADIX).pow(word.len());
            let inv = ModNum::new(RADIX).modinv();
            for r_start in 0..=word.len() {
                let r_prefix_hash = prefix_hash[word.len()] - prefix_hash[r_start] * shifter;
                let r_suffix_rev_hash = suffix_rev_hash[r_start];
                let r_is_palindrome = r_prefix_hash == r_suffix_rev_hash;
                // move forward in the rev tree
                if r_start > 0
                    && let Some(actual_idx) = rev_idx
                {
                    let key = (word.as_bytes()[r_start - 1] - b'a') as usize;
                    rev_idx = rev_root.nodes[actual_idx].to[key];
                }
                // output
                if let Some(actual_idx) = rev_idx
                    && r_is_palindrome
                {
                    for other_idx in rev_root.nodes[actual_idx].idxs.iter() {
                        // this word + other_word = palindrome
                        if *other_idx != word_idx {
                            result.push(vec![word_idx as i32, *other_idx as i32]);
                        }
                    }
                }
                // advance
                shifter = shifter * inv;
            }
            // mirror case for if word = L + R and there is R_rev in words
            let mut root_idx = Some(0);
            shifter = ModNum::new(RADIX).pow(word.len());
            for l_end in (1..=word.len()).rev() {
                let l_prefix_hash = prefix_hash[l_end];
                let l_suffix_rev_hash = suffix_rev_hash[0] - suffix_rev_hash[l_end] * shifter;
                let l_is_palindrome = l_prefix_hash == l_suffix_rev_hash;
                // move forward in the regular tree
                if l_end < word.len()
                    && let Some(actual_idx) = root_idx
                {
                    let key = (word.as_bytes()[l_end] - b'a') as usize;
                    root_idx = root.nodes[actual_idx].to[key];
                }
                // output
                if let Some(actual_idx) = root_idx
                    && l_is_palindrome
                {
                    for other_idx in root.nodes[actual_idx].idxs.iter() {
                        // this other_word + this_word = palindrome
                        result.push(vec![*other_idx as i32, word_idx as i32]);
                    }
                }
                // advance
                shifter = shifter * inv;
            }
        }

        result
    }
}
