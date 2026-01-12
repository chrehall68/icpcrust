use std::collections::VecDeque;
pub struct Solution;
impl Solution {
    pub fn maximum_robots(charge_times: Vec<i32>, running_costs: Vec<i32>, budget: i64) -> i32 {
        // we can use a deque to keep track of the maximimum in a sliding window
        // where we have the latest largest at the front, and decreasing as go to back
        // and have another variable to keep track of sum of running costs
        // sliding window vars
        let mut left = 0;
        let mut right_exclusive = 0;
        // problem vars
        let mut best = 0;
        let mut charge_max_deque = VecDeque::new();
        let mut running_sum = 0;
        while right_exclusive <= charge_times.len() {
            let cost = if right_exclusive == left {
                0
            } else {
                (right_exclusive - left) as i64 * running_sum
                    + charge_times[charge_max_deque[0]] as i64
            };
            if cost <= budget {
                // good, and can try to expand
                best = best.max(right_exclusive - left);
                // now move right forward
                if right_exclusive < charge_times.len() {
                    running_sum += running_costs[right_exclusive] as i64;
                    while let Some(&idx) = charge_max_deque.back()
                        && charge_times[idx] < charge_times[right_exclusive]
                    {
                        charge_max_deque.pop_back();
                    }
                    charge_max_deque.push_back(right_exclusive);
                }
                right_exclusive += 1;
            } else {
                // bad, need to shrink by moving left forward
                running_sum -= running_costs[left] as i64;
                if charge_max_deque[0] == left {
                    charge_max_deque.pop_front();
                }
                left += 1;
            }
        }

        best as i32
    }
}
