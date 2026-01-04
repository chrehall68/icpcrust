mod palindromic_pairs;
use palindromic_pairs::Solution;
pub fn main() {
    let words = vec!["abcd", "dcba", "lls", "s", "sssll"]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    println!("{:?}", Solution::palindrome_pairs(words));
}
