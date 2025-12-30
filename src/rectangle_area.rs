use std::collections::BTreeSet;

// segtree, where each leaf node represents
// an interval [a, b)
#[derive(Clone, Debug)]
struct Info {
    from: i32,
    to: i32,
    covered_amount: i32,
    covered_times: usize,
}
struct Segtree {
    levels: Vec<Vec<Info>>,
}
impl Segtree {
    pub fn new(rectangles: &Vec<Vec<i32>>) -> Self {
        let mut x_points = BTreeSet::new();
        for rectangle in rectangles.iter() {
            x_points.insert(rectangle[0]);
            x_points.insert(rectangle[2]);
        }

        // make first level
        let x_vec: Vec<_> = x_points.into_iter().collect();
        let mut first_level = Vec::new();
        for i in 1..x_vec.len() {
            first_level.push(Info {
                from: x_vec[i - 1],
                to: x_vec[i],
                covered_amount: 0,
                covered_times: 0,
            });
        }

        // make the rest of the levels
        let mut levels = Vec::new();
        levels.push(first_level);
        let mut back = levels.last().unwrap();
        while back.len() > 1 {
            // push next level
            let mut next_level = Vec::new();
            for i in (0..back.len()).step_by(2) {
                if i + 1 < back.len() {
                    // combine
                    next_level.push(Info {
                        from: back[i].from,
                        to: back[i + 1].to,
                        covered_amount: 0,
                        covered_times: 0,
                    });
                } else {
                    next_level.push(back[i].clone());
                }
            }
            levels.push(next_level);
            back = levels.last().unwrap();
        }
        Segtree { levels }
    }

    pub fn enable(&mut self, from: i32, to: i32) {
        self.helper(from, to, 1, self.levels.len() - 1, 0);
    }
    pub fn disable(&mut self, from: i32, to: i32) {
        self.helper(from, to, -1, self.levels.len() - 1, 0);
    }
    fn helper(&mut self, from: i32, to: i32, increment: i32, level: usize, level_idx: usize) {
        let info = &self.levels[level][level_idx];
        if from <= info.from && info.to <= to {
            // this is the first thing totally in the interval
            self.levels[level][level_idx].covered_times =
                (info.covered_times as i32 + increment) as usize;
        } else if !(from >= info.to || to <= info.from) {
            // some overlap
            // so update left and right, and then postorder update this
            self.helper(from, to, increment, level - 1, level_idx * 2);
            if level_idx * 2 + 1 < self.levels[level - 1].len() {
                self.helper(from, to, increment, level - 1, level_idx * 2 + 1);
            }
        }
        self.update(level, level_idx);
    }
    fn update(&mut self, level: usize, level_idx: usize) {
        let info = &self.levels[level][level_idx];
        let val = if info.covered_times > 0 {
            info.to - info.from
        } else {
            if level == 0 {
                0
            } else {
                // ask children
                self.levels[level - 1][level_idx * 2].covered_amount
                    + if level_idx * 2 + 1 < self.levels[level - 1].len() {
                        self.levels[level - 1][level_idx * 2 + 1].covered_amount
                    } else {
                        0
                    }
            }
        };
        self.levels[level][level_idx].covered_amount = val;
    }
    pub fn query(&self) -> i32 {
        self.levels[self.levels.len() - 1][0].covered_amount
    }
}

pub struct Solution;
impl Solution {
    pub fn rectangle_area(rectangles: Vec<Vec<i32>>) -> i32 {
        // rectangles[i] = [xbl, ybl, xur, yur]
        let mut start_ends: Vec<_> = (0..rectangles.len())
            .flat_map(|i| vec![(i, true), (i, false)])
            .collect();
        start_ends.sort_by_key(|&(i, is_start)| rectangles[i][if is_start { 1 } else { 3 }]);
        let mut root = Segtree::new(&rectangles);
        let mut total = 0;
        let mut prev_y = rectangles.iter().map(|r| r[1]).min().unwrap();
        const C: i64 = 1_000_000_007;
        for (rec_idx, is_start) in start_ends.into_iter() {
            let cur_y = rectangles[rec_idx][if is_start { 1 } else { 3 }];
            let dy = cur_y - prev_y;
            total += (root.query() as i64 * dy as i64) % C;
            total %= C;

            if is_start {
                root.enable(rectangles[rec_idx][0], rectangles[rec_idx][2]);
            } else {
                root.disable(rectangles[rec_idx][0], rectangles[rec_idx][2]);
            }
            prev_y = cur_y;
        }
        total as i32
    }
}
