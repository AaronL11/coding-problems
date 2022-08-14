use std::collections::HashMap;


impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut result = vec![];
        for (i, n) in nums.iter().enumerate() {
            if let Some(&j) = map.get(n) {
                result = vec![j, i as i32];
                break;
            } else {
                map.insert(target - n, i as i32);
            }
        }
        result
    }
}

