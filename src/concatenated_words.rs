use std::collections::VecDeque;
const ALPHABET: usize = 26;

type NodeId = usize;

#[derive(Debug)]
struct TrieNode {
    depth: usize,
    is_end: bool,
    to: [Option<NodeId>; 26],
    output_link: Option<NodeId>,
    suffix_link: Option<NodeId>,
}
// root is always node 0
#[derive(Debug)]
struct AhoCorasick {
    nodes: Vec<TrieNode>,
    cur: NodeId,
}

impl TrieNode {
    pub fn new(depth: usize) -> Self {
        TrieNode {
            depth,
            is_end: false,
            to: [const { None }; ALPHABET],
            output_link: None,
            suffix_link: None,
        }
    }
}
impl AhoCorasick {
    const ROOT: usize = 0;
    pub fn new() -> Self {
        AhoCorasick {
            nodes: vec![TrieNode::new(0)],
            cur: Self::ROOT,
        }
    }
    pub fn insert(&mut self, word: &str) {
        let mut cur = Self::ROOT;
        for c in word.as_bytes() {
            let key = (c - b'a') as usize;
            if let None = self.nodes[cur].to[key] {
                self.nodes[cur].to[key] = Some(self.nodes.len());
                self.nodes.push(TrieNode::new(self.nodes[cur].depth + 1));
            }
            cur = self.nodes[cur].to[key].unwrap();
        }
        self.nodes[cur].is_end = true;
    }
    pub fn construct_automaton(&mut self) {
        // explore initial connections
        let mut to_explore = VecDeque::new();
        for i in 0..ALPHABET {
            if let Some(conn) = self.nodes[Self::ROOT].to[i] {
                to_explore.push_back(conn);
                self.nodes[conn].suffix_link = Some(Self::ROOT);
            }
        }
        // then bfs out from there
        while let Some(top) = to_explore.pop_front() {
            for i in 0..ALPHABET {
                if let Some(conn) = self.nodes[top].to[i] {
                    // then find an appropriate suffix
                    let mut suffix = self.nodes[top].suffix_link.unwrap();
                    let mut suffix_conn = self.nodes[suffix].to[i];
                    while let None = suffix_conn
                        && suffix != Self::ROOT
                    {
                        suffix = self.nodes[suffix].suffix_link.unwrap();
                        suffix_conn = self.nodes[suffix].to[i];
                    }
                    // add connection
                    if let Some(next) = suffix_conn {
                        self.nodes[conn].suffix_link = Some(next);
                        if self.nodes[next].is_end {
                            self.nodes[conn].output_link = Some(next);
                        } else if let Some(output_link) = self.nodes[next].output_link {
                            self.nodes[conn].output_link = Some(output_link);
                        }
                    } else {
                        // suffix is just the root
                        self.nodes[conn].suffix_link = Some(Self::ROOT);
                    }
                    // keep bfs'ing
                    to_explore.push_back(conn);
                }
            }
        }
    }
    pub fn reset(&mut self) {
        self.cur = Self::ROOT;
    }
    pub fn advance(&mut self, c: u8) -> Vec<usize> {
        // returns the depths
        let mut result = Vec::new();
        let key = (c - b'a') as usize;

        // advance to either connection, or root (none)
        let mut next = self.nodes[self.cur].to[key];
        while let None = next
            && self.cur != Self::ROOT
        {
            self.cur = self.nodes[self.cur].suffix_link.unwrap();
            next = self.nodes[self.cur].to[key];
        }
        if let Some(val) = next {
            self.cur = val;
        }

        // output
        if self.nodes[self.cur].is_end {
            result.push(self.nodes[self.cur].depth);
        }
        let mut outputter_option = self.nodes[self.cur].output_link;
        while let Some(outputter) = outputter_option {
            result.push(self.nodes[outputter].depth);
            outputter_option = self.nodes[outputter].output_link;
        }

        result
    }
}

pub struct Solution;
impl Solution {
    pub fn find_all_concatenated_words_in_a_dict(words: Vec<String>) -> Vec<String> {
        // so we build an aho corasic automaton
        // fill it with the vocabulary
        // then just match it
        // this scales as long as the vocab size is small enough
        let mut root = AhoCorasick::new();
        for word in words.iter() {
            root.insert(&word);
        }
        root.construct_automaton();
        let mut result = Vec::new();
        let total_words = words.len() as i32;
        for word in words {
            // possible[i] = max # of concatenated things to make [0, i)
            let mut possible = vec![-total_words; word.len() + 1];
            possible[0] = 0;
            root.reset();
            for (i, c) in word.as_bytes().iter().enumerate() {
                let lengths = root.advance(*c);
                for l in lengths {
                    possible[i + 1] = possible[i + 1].max(possible[i + 1 - l] + 1);
                }
            }
            // now just count how many words we can use to make it to the end of the string
            if possible[word.len()] >= 2 {
                result.push(word);
            }
        }
        result
    }
}
