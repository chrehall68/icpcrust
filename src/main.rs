mod add_operators;
use add_operators::Solution;
pub fn main() {
    println!("{:?}", Solution::add_operators("232".to_owned(), 8));
}
