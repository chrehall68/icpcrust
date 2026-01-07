use std::mem::swap;

pub struct Solution;
impl Solution {
    fn sort_cyclic_shifts(s: &str) -> Vec<usize> {
        let n = s.len();
        let alphabet_size = 256;
        // p[i] = idx of start of the i'th smallest substring of length 2**x
        // c[i] is the equivalency classes; smaller equivalence class -> smaller
        let mut p = vec![0; n];
        let mut c = vec![0; n];
        let mut counts = vec![0; n.max(alphabet_size)];
        // initial counting sort
        for i in 0..n {
            counts[s.as_bytes()[i] as usize] += 1;
        }
        // psum everything
        for i in 1..alphabet_size {
            counts[i] += counts[i - 1];
        }
        // then insert into place
        for i in (0..n).rev() {
            let idx = &mut counts[s.as_bytes()[i] as usize];
            *idx -= 1;
            p[*idx as usize] = i;
        }
        // finally, populate equivalency classes
        let mut num_classes = 0;
        c[p[0]] = num_classes;
        for i in 1..n {
            if s.as_bytes()[p[i]] != s.as_bytes()[p[i - 1]] {
                num_classes += 1;
            }
            c[p[i]] = num_classes;
        }
        // now do maintenance
        let mut next_p = vec![0; n];
        let mut next_c = vec![0; n];
        let mut pow = 1;
        while pow < n {
            // combine the length pow strings to make length pow*2 strings
            // sort them by (class_first, class_second)
            // if we take any element to be class_Second, they're already sorted by their second
            // so, we can keep that order and then sort by their first
            // get firsts
            for i in 0..n {
                p[i] = (p[i] + (n - pow % n)) % n;
            }
            // then sort by those firsts using counting sort
            counts.fill(0);
            // first count
            for i in 0..n {
                counts[c[p[i]]] += 1;
            }
            // then psum
            for i in 1..n {
                counts[i] += counts[i - 1];
            }
            // fill in next p
            for i in (0..n).rev() {
                let idx = &mut counts[c[p[i]]];
                *idx -= 1;
                next_p[*idx as usize] = p[i];
            }
            // fill in new equivalency classes
            num_classes = 0;
            next_c[next_p[0]] = num_classes;
            for i in 1..n {
                if (c[next_p[i - 1]], c[(next_p[i - 1] + pow) % n])
                    != (c[next_p[i]], c[(next_p[i] + pow) % n])
                {
                    num_classes += 1;
                }
                next_c[next_p[i]] = num_classes;
            }
            // advance
            swap(&mut next_p, &mut p);
            swap(&mut next_c, &mut c);
            pow *= 2;
        }

        p
    }
    fn make_suffix_array(s: &mut String) -> Vec<usize> {
        s.push('\0');
        let result = Self::sort_cyclic_shifts(&s);
        s.pop();
        result.into_iter().skip(1).collect()
    }
    pub fn last_substring(mut s: String) -> String {
        // it will be a suffix
        // suppose it wasn't. Then, it ends somewhere. You can make it lexicographically
        // greater by adding any letter to it
        let sorted_suffixes = Self::make_suffix_array(&mut s);
        s[*sorted_suffixes.last().unwrap()..].to_owned()
    }
}
