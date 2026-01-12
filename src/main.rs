mod maximum_robots;
use maximum_robots::Solution;
pub fn main() {
    let charge_times = vec![3, 6, 1, 3, 4];
    let running_costs = vec![2, 1, 3, 4, 5];
    println!(
        "{}",
        Solution::maximum_robots(charge_times, running_costs, 25)
    );
}
