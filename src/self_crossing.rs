use std::{
    cmp::Reverse,
    collections::{BTreeMap, BTreeSet, BinaryHeap},
};

pub struct Solution;
impl Solution {
    pub fn is_self_crossing(distance: Vec<i32>) -> bool {
        // start at origin
        // then go counterclockwise (up, left, down, right)
        // (x, y)
        let directions = vec![(0, 1), (-1, 0), (0, -1), (1, 0)];
        let mut vertical_lines = Vec::new();
        let mut horizontal_lines = Vec::new();
        let mut cur_point = (0, 0);
        let mut corners = BTreeSet::new();
        for (i, dist) in distance.into_iter().enumerate() {
            let dir = directions[i % directions.len()];
            let dx = dist as i64 * dir.0;
            let dy = dist as i64 * dir.1;
            let end = (cur_point.0 + dx, cur_point.1 + dy);
            let min_x = cur_point.0.min(end.0);
            let max_x = cur_point.0.max(end.0);
            let min_y = cur_point.1.min(end.1);
            let max_y = cur_point.1.max(end.1);
            if i % directions.len() == 0 || i % directions.len() == 2 {
                // vertical line (start_y, end_y, x)
                vertical_lines.push((min_y, max_y, min_x));
            } else {
                // horizontal line (start_x, end_x, y)
                horizontal_lines.push((min_x, max_x, min_y));
            }
            if cur_point != (0, 0) {
                corners.insert(cur_point);
            }
            cur_point = end;
        }
        vertical_lines.sort();
        horizontal_lines.sort();
        // check to see if any vertical lines intersect with each other
        let mut latest_vertical = BTreeMap::new();
        for &(start_y, end_y, x) in vertical_lines.iter() {
            let last_y = latest_vertical.entry(x).or_insert(i64::MIN);
            if *last_y > start_y {
                return true; // intersection
            } else {
                *last_y = end_y;
            }
        }
        // similar for horizontal lines, except we want to actually store what's where
        let mut latest_horizontal = BTreeMap::new();
        for &(start_x, end_x, y) in horizontal_lines.iter() {
            let lines: &mut Vec<(i64, i64)> = latest_horizontal.entry(y).or_insert(Vec::new());
            if let Some(line) = lines.last()
                && line.1 > start_x
            {
                return true; // intersection
            } else {
                lines.push((start_x, end_x));
            }
        }
        // none of the horizontal/vertical intersect
        // so now we line scan
        let mut to_pop = BinaryHeap::new();
        let mut active = BTreeSet::new();
        let mut v_idx = 0;
        for (y, lines) in latest_horizontal {
            // add anything that has become active
            while v_idx < vertical_lines.len() && vertical_lines[v_idx].0 <= y {
                active.insert(vertical_lines[v_idx].2); // this x value is occupied
                to_pop.push((Reverse(vertical_lines[v_idx].1), vertical_lines[v_idx].2)); // until we reach end_y
                v_idx += 1;
            }
            // pop anything that has become inactive
            while let Some(&(Reverse(end_y), x)) = to_pop.peek()
                && end_y < y
            {
                to_pop.pop();
                active.remove(&x);
            }
            // see if any of the lines intersect
            for (start_x, end_x) in lines {
                // retrieve at most 3 things for each range -> still just lgn time
                for x in active.range(start_x..=end_x) {
                    if !corners.contains(&(*x, y)) {
                        return true;
                    }
                }
            }
        }
        false
    }
}
