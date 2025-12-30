mod concatenated_words;
use concatenated_words::Solution;
pub fn main() {
    let ref_words = vec!["rfgve", "gve", "v", "e", "g"];
    let words = ref_words.into_iter().map(|s| s.to_owned()).collect();
    println!(
        "{:?}",
        Solution::find_all_concatenated_words_in_a_dict(words)
    );
}
