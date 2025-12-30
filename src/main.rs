mod rectangle_area;
use rectangle_area::Solution;
pub fn main() {
    let rectangles = vec![vec![0, 0, 2, 2], vec![1, 0, 2, 3], vec![1, 0, 3, 1]];
    // let rectangles = vec![vec![0, 0, 1000000000, 1000000000]];
    let res = Solution::rectangle_area(rectangles);
    println!("{}", res);
}
