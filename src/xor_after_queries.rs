pub struct Solution;
const C: i64 = 1_000_000_007;
fn modinv(v: i64) -> i64 {
    if v <= 1 {
        return v;
    }
    C - C / v * modinv(C % v) % C
}
impl Solution {
    pub fn xor_after_queries(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        // i think we need the actual numbers
        // because there's no properties of xor that say i can multiply after
        // line scan?
        // maybe line scan and combine periods
        // since there can be at most n distinct periods
        // and if we have those periods 1,2,3,... that leads to nlgn work
        // oh but we have offsets too; those mess up our guarantees
        // ok but if we do this strategy up to sqrt(n), then that leads to
        // n sqrt(n) work, which is acceptable
        // then for everything else, we do simulation; that works since
        // simulating with an increment > sqrt(n) leads to at most q sqrt(n) work
        // so sqrt(n) (n+q), which fits
        let rt = (nums.len() as f64).sqrt().ceil() as i32;
        let mut queries_by_k = vec![vec![]; rt as usize + 1];
        for q in queries {
            let (l, r, k, v) = (q[0], q[1], q[2], q[3]);
            if k > rt {
                // simulate
                let mut idx = l as usize;
                while idx <= r as usize {
                    nums[idx] = ((nums[idx] as i64 * v as i64) % C) as i32;
                    idx += k as usize;
                }
            } else {
                // process later
                queries_by_k[k as usize].push((l, r, v));
            }
        }
        // now process the buckets
        for period in 1..=rt as usize {
            let mut to_process = vec![1; nums.len()];
            let mut multipliers = vec![1; period];
            for &(l, r, v) in queries_by_k[period].iter() {
                to_process[l as usize] = (to_process[l as usize] * v as i64) % C;
                // make r be periodic
                let num_periods = (r - l) as usize / period + 1;
                let end = l as usize + period * num_periods;
                if end < nums.len() {
                    to_process[end] = (to_process[end] * modinv(v as i64)) % C;
                }
            }
            // then sweep
            for start in (0..nums.len()).step_by(period) {
                for j in 0..period {
                    if start + j < nums.len() {
                        multipliers[j] = (multipliers[j] * to_process[start + j]) % C;
                        nums[start + j] = ((nums[start + j] as i64 * multipliers[j]) % C) as i32;
                    }
                }
            }
        }
        // now xor
        nums.into_iter().reduce(|accum, x| accum ^ x).unwrap()
    }
}
