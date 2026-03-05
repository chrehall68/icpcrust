// maximum cyclic partition score - 3743
// https://leetcode.com/problems/maximize-cyclic-partition-score/description/
// interesting cyclic array problem using dp

pub struct Solution;
impl Solution {
    fn solve(arr: Vec<i32>, k: i32) -> i64 {
        // so we either use the current number as the min, or the max
        // or skip it
        // 3 things:
        // - resolved[k] = max score partitioning arr[..cur_idx] using k or less
        // - missingMax[k] = max score parititioning arr[..cur_idx] using k or less
        //   complete, but with the start of a new subarray that's missing its max
        // - missingMin[k] = similar thing
        // if we use the current number as the min, then either:
        // - it is the min of a new subarray
        //   - in this case, simply update the best missingMax
        // - it is the min of an existing subarray
        //   - update resolved
        // similar for max
        assert!(k > 0);
        let k = k as usize;
        let mut resolved = vec![0; k + 1];
        let mut missing_max = vec![i32::MIN as i64; k + 1];
        let mut missing_min = vec![i32::MIN as i64; k + 1];
        for num in arr {
            let mut next_resolved = resolved.clone();
            let mut next_missing_max = missing_max.clone();
            let mut next_missing_min = missing_min.clone();
            for cur_k in 0..k {
                // maintain the "using k or less"
                next_resolved[cur_k + 1] = next_resolved[cur_k + 1].max(next_resolved[cur_k]);
                // try using this as the min
                // - of an existing
                next_resolved[cur_k + 1] =
                    next_resolved[cur_k + 1].max(missing_min[cur_k] - num as i64);
                // - of a new
                next_missing_max[cur_k] = next_missing_max[cur_k].max(resolved[cur_k] - num as i64);
                // try using this as the max
                // - of an existing
                next_resolved[cur_k + 1] =
                    next_resolved[cur_k + 1].max(missing_max[cur_k] + num as i64);
                // - of a new
                next_missing_min[cur_k] = next_missing_min[cur_k].max(resolved[cur_k] + num as i64);
            }
            resolved = next_resolved;
            missing_max = next_missing_max;
            missing_min = next_missing_min;
        }

        resolved[k]
    }
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i64 {
        // dp[start][end][k'] = maximum score for
        // the range [start, end] using at most k' subarrays
        // then for every possible end' in [start, end], try
        // dividing there
        // that gives n**4 time
        // ok so we can think of every subarray as being defined by its min and its max
        // and only the first and last subarrays will not have that
        // (since they'll have some extra numbers leading or trailing)
        // so, we can say that the smallest number gets paired
        // with either:
        // - something on its left
        // - something on its right
        // if it's paired with something on its right, we can think of this as
        // an array starting at index i and ending at index i-1
        // if it's paired with something on its left, it's an array
        // starting at index i+1 and ending at index i
        // so then now our solution just needs to handle the leading/trailing unneeded numbers
        // so now the question is just how to solve those
        let mut min_idx = 0;
        for i in 0..nums.len() {
            if nums[i] < nums[min_idx] {
                min_idx = i;
            }
        }
        let mut pair_right = nums[min_idx..].to_vec();
        pair_right.extend_from_slice(&nums[..min_idx]);
        let mut pair_left = nums[min_idx + 1..].to_vec();
        pair_left.extend_from_slice(&nums[..=min_idx]);
        Self::solve(pair_right, k).max(Self::solve(pair_left, k))
    }
}
