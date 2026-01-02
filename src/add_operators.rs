pub struct Solution;
#[derive(Clone, Copy, PartialEq)]
enum Operator {
    End,
    Add,
    Sub,
    Mul,
    Concat,
}
impl PartialOrd for Operator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(match (self, other) {
            (Operator::End, _) => std::cmp::Ordering::Less,
            (_, Operator::End) => std::cmp::Ordering::Greater,
            (Operator::Add, Operator::Add)
            | (Operator::Sub, Operator::Sub)
            | (Operator::Mul, Operator::Mul)
            | (Operator::Concat, Operator::Concat)
            | (Operator::Add, Operator::Sub)
            | (Operator::Sub, Operator::Add) => std::cmp::Ordering::Equal,
            (Operator::Add, _) | (Operator::Sub, _) | (Operator::Mul, Operator::Concat) => {
                std::cmp::Ordering::Less
            }
            _ => std::cmp::Ordering::Greater,
        });
    }
}
impl Operator {
    fn call(&self, lhs: i64, rhs: i64) -> Result<i64, &'static str> {
        match self {
            Operator::Add => Ok(lhs + rhs),
            Operator::Concat => {
                if lhs != 0 {
                    Ok(lhs * 10 + rhs)
                } else {
                    Err("leading zeros")
                }
            }
            Operator::Mul => Ok(lhs * rhs),
            Operator::Sub => Ok(lhs - rhs),
            Operator::End => Ok(lhs),
        }
    }
}
impl Solution {
    fn rec(
        nums: &[i64],
        idx: usize,
        result: &mut Vec<String>,
        cur_seq: &mut Vec<Operator>,
        target: i64,
    ) {
        if idx == nums.len() {
            // then check that it's valid
            let mut lhs_stack = Vec::new();
            let mut op_stack = Vec::new();
            let mut res_str = String::new();
            cur_seq.push(Operator::End); // op with lowest priority to get all other operations to happen
            // process all numbers/ops
            for nums_idx in 0..nums.len() {
                let trailing_op = cur_seq[nums_idx];
                // display stuff
                res_str.push_str(&nums[nums_idx].to_string());
                match trailing_op {
                    Operator::Add => res_str.push_str("+"),
                    Operator::Sub => res_str.push_str("-"),
                    Operator::Mul => res_str.push_str("*"),
                    Operator::Concat | Operator::End => res_str.push_str(""),
                }
                // calculation w/ order of operations
                lhs_stack.push(Ok(nums[nums_idx]));
                while lhs_stack.len() >= 2
                    && let Ok(lhs) = lhs_stack[lhs_stack.len() - 2]
                    && let Ok(rhs) = lhs_stack[lhs_stack.len() - 1]
                    && op_stack.len() >= 1
                    // while the previous operator is more important than this, evaluate the prev
                    && op_stack[op_stack.len() - 1] >= trailing_op
                {
                    let eval_op: Operator = op_stack.pop().unwrap();
                    lhs_stack.pop();
                    lhs_stack.pop();
                    lhs_stack.push(eval_op.call(lhs, rhs));
                }
                op_stack.push(trailing_op);
            }
            // after finished processing, there should be one good result left;
            // compare it to the value we're trying to create
            if lhs_stack.len() == 1
                && let Ok(val) = lhs_stack[0]
                && val == target
            {
                result.push(res_str); // matched
            }
            // remove the Operator::End that we pushed
            cur_seq.pop();
        } else {
            // choose one of the operators to put here
            for op in vec![
                Operator::Add,
                Operator::Sub,
                Operator::Mul,
                Operator::Concat,
            ] {
                cur_seq.push(op);
                Self::rec(nums, idx + 1, result, cur_seq, target);
                cur_seq.pop();
            }
        }
    }
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        let mut result = Vec::new();
        let mut cur = Vec::new();
        Self::rec(
            &num.as_bytes()
                .iter()
                .map(|&b| (b - b'0') as i64)
                .collect::<Vec<i64>>(),
            1,
            &mut result,
            &mut cur,
            target as i64,
        );
        result
    }
}
