mod palindromic_tree;
use palindromic_tree::Solution;
fn main() {
    let parent = vec![-1, 0, 0, 1, 1, 2];
    let s = "acaabc".to_owned();
    println!("{}", Solution::count_palindrome_paths(parent, s));
}
