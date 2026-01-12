use std::collections::VecDeque;
pub struct Solution;
impl Solution {
    pub fn count_non_decreasing_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let mut total = 0;
        let mut cur_cost = 0;
        let mut right = nums.len() - 1;
        // max_deque stores indices, and front is smallest, back is largest
        let mut max_deque = VecDeque::new();
        for i in (0..nums.len()).rev() {
            while let Some(&idx) = max_deque.front()
                && nums[idx] < nums[i]
            {
                // need to pop it off first
                max_deque.pop_front();
                // and also need to increase our cur cost
                // by the difference between our num and nums[idx]
                let dif = nums[i] - nums[idx];
                // dif gets applied however many times there are
                let next_idx = match max_deque.front() {
                    Some(next_idx) => *next_idx,
                    None => right + 1,
                };
                cur_cost += (next_idx - idx) as i64 * dif as i64;
            }
            max_deque.push_front(i);
            // then need to move our right backward
            while cur_cost > k as i64 {
                let cost_right = nums[*max_deque.back().unwrap()] - nums[right];
                cur_cost -= cost_right as i64;
                if *max_deque.back().unwrap() == right {
                    max_deque.pop_back();
                }
                right -= 1;
            }
            // now we have some span
            // and any of the spans starting here work
            let span_len = right - i + 1;
            total += span_len as i64;
        }
        total
    }
}
