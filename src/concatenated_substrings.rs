use std::collections::VecDeque;

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};
const ALPHABET: usize = 26;

type Node = Rc<RefCell<TrieNode>>;

#[derive(Debug)]
struct TrieNode {
    depth: usize,
    is_end: bool,
    to: [Option<Node>; 26],
    output_link: Option<Weak<RefCell<TrieNode>>>,
    suffix_link: Option<Weak<RefCell<TrieNode>>>,
}
struct AhoCorasick {
    root: Node,
    cur: Node,
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
    pub fn new() -> Self {
        let root = Rc::new(RefCell::new(TrieNode::new(0)));
        AhoCorasick {
            root: root.clone(),
            cur: root,
        }
    }
    pub fn insert(&mut self, word: &str) {
        let mut cur = self.root.clone();
        for c in word.as_bytes() {
            let key = (c - b'a') as usize;
            let mut to = cur.borrow().to[key].clone();
            if let None = to {
                let to_insert = Some(Rc::new(RefCell::new(TrieNode::new(cur.borrow().depth + 1))));
                cur.borrow_mut().to[key] = to_insert
            }
            to = cur.borrow().to[key].clone();
            let next = to.unwrap();
            cur = next;
        }
        cur.borrow_mut().is_end = true;
    }
    pub fn construct_automaton(&mut self) {
        // explore initial connections
        let mut to_explore = VecDeque::new();
        for i in 0..ALPHABET {
            if let Some(val) = self.root.borrow().to[i].clone() {
                to_explore.push_back(val.clone());
                val.borrow_mut().suffix_link = Some(Rc::downgrade(&self.root));
            }
        }
        // then bfs out from there
        while let Some(top) = to_explore.pop_front() {
            for i in 0..ALPHABET {
                if let Some(connection) = top.borrow().to[i].clone() {
                    // then find an appropriate suffix
                    let mut suffix = top
                        .borrow()
                        .suffix_link
                        .as_ref()
                        .unwrap()
                        .upgrade()
                        .unwrap();
                    let mut suffix_conn = suffix.borrow().to[i].clone();
                    while let None = suffix_conn
                        && suffix.as_ptr() != self.root.as_ptr()
                    {
                        let next = suffix
                            .borrow()
                            .suffix_link
                            .as_ref()
                            .unwrap()
                            .upgrade()
                            .unwrap();
                        suffix = next;
                        suffix_conn = suffix.borrow().to[i].clone();
                    }
                    // add connection
                    if let Some(next) = suffix_conn {
                        connection.borrow_mut().suffix_link = Some(Rc::downgrade(&next));
                        if next.borrow().is_end {
                            connection.borrow_mut().output_link = Some(Rc::downgrade(&next));
                        } else if let Some(output_link) = next.borrow().output_link.clone() {
                            connection.borrow_mut().output_link = Some(output_link);
                        }
                    } else {
                        // suffix is just the root
                        connection.borrow_mut().suffix_link = Some(Rc::downgrade(&self.root));
                    }
                    // keep bfs'ing
                    to_explore.push_back(connection);
                }
            }
        }
    }
    pub fn reset(&mut self) {
        self.cur = self.root.clone();
    }
    pub fn advance(&mut self, c: u8) -> Vec<usize> {
        // returns the depths
        let mut result = Vec::new();
        let key = (c - b'a') as usize;

        // advance to either connection, or root (none)
        let mut next = self.cur.borrow().to[key].clone();
        while let None = next
            && self.cur.as_ptr() != self.root.as_ptr()
        {
            let temp = self
                .cur
                .borrow()
                .suffix_link
                .as_ref()
                .unwrap()
                .upgrade()
                .unwrap();
            self.cur = temp;
            next = self.cur.borrow().to[key].clone();
        }
        if let Some(val) = next {
            self.cur = val;
        }

        // output
        if self.cur.borrow().is_end {
            result.push(self.cur.borrow().depth);
        }
        let mut outputter_option = self.cur.borrow().output_link.clone();
        while let Some(outputter) = outputter_option {
            result.push(outputter.upgrade().unwrap().borrow().depth);
            outputter_option = outputter.upgrade().unwrap().borrow().output_link.clone();
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
