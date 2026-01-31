// https://leetcode.com/problems/minimum-cost-to-divide-array-into-subarrays/description/
pub struct Solution;
// view https://codeforces.com/blog/entry/63823
use std::collections::VecDeque;
impl Solution {
    fn eval(line: (i64, i64), x: i64) -> i64 {
        let (slope, intercept) = line;
        intercept + slope * x
    }
    fn intersection_time(a: (i64, i64), b: (i64, i64)) -> i64 {
        let (s1, i1) = a;
        let (s2, i2) = b;
        // s1*x + i1 = s2*x + i2
        // -> i1-i2 = (s2-s1)*x
        // -> (i1-i2)/(s2-s1)
        (i1 - i2) / (s2 - s1)
    }
    pub fn minimum_cost(nums: Vec<i32>, cost: Vec<i32>, k: i32) -> i64 {
        // so if we were to "brute force this"
        // we have j <= 1000 positions
        // and each could be the start of the i <= 1000'th subarray
        // and to calculate the best cost for (j,i), we would just
        // iterate through everything after it and take min
        // for all r >= j of (sum(nums[..=r]) + k*i) * sum(cost[j..=r]) + prev_dp[r+1]
        // so we know that k*i is a constant w.r.t i
        // ideally we would do it n**2 or n**2 lgn
        // like for every i
        // then for every j backwards
        // let's treat j as r, then we know prev_dp[r+1], and we know sum(nums..=r), and we know k*i
        // we don't know sum(cost[other_j..=r])
        // but we know sum(cost[other_j..=r]) = sum(cost[..=r]) - sum(cost[..other_j])
        // so if we're minimizing the above equation, we're minimizing
        // (sum(nums[..=r]) + k*i) * (sum(cost[..=r]) - sum(cost[..other_j])) + prev_dp[r+1]
        // = (sum(nums[..=r]) + k*i) * sum(cost[..=r]) + prev_dp[r+1]
        //   - (sum(nums[..=r]) + k*i) * sum(cost[..other_j])
        // which we'll say is intercept - slope * sum(cost[..other_j])
        // hmmm so we know that sum(nums[..=r]) + k*i decreases as r -> 0
        // so that means basically everything will have a smaller slope
        let mut nums_psums = vec![0; nums.len() + 1];
        let mut cost_psums = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            nums_psums[i + 1] = nums_psums[i] + nums[i] as i64;
            cost_psums[i + 1] = cost_psums[i] + cost[i] as i64;
        }
        let mut prev_dp = vec![i64::MAX; nums.len() + 1];
        prev_dp[nums.len()] = 0;
        for i in (1..=nums.len()).rev() {
            let mut cur_level = vec![0; nums.len() + 1];
            // front will be largest number, back will be smallest number
            // we want to maintain some collection of lines
            // where the front is the current minimum (will have max intercept and max slope)
            // and as we go to the back, intercept goes down and so does slope
            // then getting the next max is just popping the top
            let mut lines = VecDeque::new();
            let extra = k as i64 * i as i64;
            // (slope, intercept)
            let initial_line = (
                nums_psums[nums.len()] + extra,
                -((nums_psums[nums.len()] + extra) * cost_psums[nums.len()]),
            );
            lines.push_back(initial_line);

            // so we have lines inserted with decreasing slope
            // and our queries (cost_psum) are also decreasing
            // so we can use cht
            // and we want mins, so we negate both slope and intercept (and re-negate when getting our answer)
            // but then that would put us in increasing order of slopes, so we negate the slope again
            for j in (0..nums.len()).rev() {
                // see if there's a new best line
                let cost_psum = cost_psums[j];
                while lines.len() > 1
                    && Self::eval(lines[lines.len() - 1], cost_psum)
                        < Self::eval(lines[lines.len() - 2], cost_psum)
                {
                    lines.pop_back();
                }
                // then set my dp
                cur_level[j] = -Self::eval(lines[lines.len() - 1], cost_psum);
                // and add this line
                if prev_dp[j] != i64::MAX {
                    let my_slope = nums_psums[j] + extra;
                    let my_intercept = prev_dp[j] + my_slope * cost_psum;
                    let line = (my_slope, -my_intercept);
                    // new lowest slope
                    assert!(lines.len() > 0);
                    assert!(lines[0].0 > my_slope);
                    while lines.len() > 1
                        && Self::intersection_time(line, lines[0])
                            > Self::intersection_time(lines[0], lines[1])
                    {
                        lines.pop_front();
                    }
                    lines.push_front(line);
                }
            }
            prev_dp = cur_level;
        }
        prev_dp[0]
    }
}
