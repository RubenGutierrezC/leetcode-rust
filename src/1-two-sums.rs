use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash: HashMap<i32, i32> = HashMap::new();

        for (index, num) in nums.iter().enumerate()  {
            if let Some(x) = hash.get(&num) {
                return vec![*x, index as i32]
            } else {
                hash.insert(target - *num, index as i32);
            }
        }

        vec![]
    }
}

