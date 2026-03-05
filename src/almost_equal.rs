// uses z_function (z function)
// which, for every i, gives the largest j
// such that s[:j] == s[i:i+j]
// https://leetcode.com/problems/find-the-occurrence-of-first-almost-equal-substring/
pub struct Solution;
impl Solution {
    fn zf(s: &[u8], pattern: &[u8]) -> Vec<usize> {
        let mut z_p = vec![0; pattern.len()];
        // left/right of the rightmost match
        let mut l = 0;
        let mut r = 0;
        for i in 1..pattern.len() {
            if i < r {
                // initialize with min
                z_p[i] = (r - i).min(z_p[i - l]);
            }
            // move forward
            while i + z_p[i] < pattern.len() && pattern[i + z_p[i]] == pattern[z_p[i]] {
                z_p[i] += 1;
            }
            // update l and r
            if i + z_p[i] > r {
                r = i + z_p[i];
                l = i;
            }
        }
        // now match the text
        let mut z = vec![0; s.len()];
        l = 0;
        r = 0;
        for i in 0..s.len() {
            if i < r {
                // initialize
                z[i] = (r - i).min(z_p[i - l]);
            }
            // move forward
            while i + z[i] < s.len() && z[i] < pattern.len() && pattern[z[i]] == s[i + z[i]] {
                z[i] += 1;
            }
            // update l and r
            if i + z[i] > r {
                l = i;
                r = i + z[i];
            }
        }
        z
    }
    pub fn min_starting_index(s: String, pattern: String) -> i32 {
        // what's the largest match starting here
        // + what's the largest match ending at end
        let s_rev: String = s.chars().rev().collect();
        let p_rev: String = pattern.chars().rev().collect();
        let mut ending_here: Vec<_> = Self::zf(s_rev.as_bytes(), p_rev.as_bytes());
        ending_here.reverse();
        let starting_here = Self::zf(s.as_bytes(), pattern.as_bytes());

        for start in 0..=s.len() - pattern.len() {
            // total match
            if starting_here[start] == pattern.len() {
                return start as i32;
            }
            // otherwise, partial match
            let covered = starting_here[start] + ending_here[start + pattern.len() - 1];
            // +1 bc we're allowed that 1 missing char
            if covered + 1 >= pattern.len() {
                return start as i32;
            }
        }

        -1
    }
}
