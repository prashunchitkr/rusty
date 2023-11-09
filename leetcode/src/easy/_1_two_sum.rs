use std::collections::HashMap;

struct Solution;

impl Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            if map.contains_key(&complement) {
                return vec![map[&complement], i as i32];
            }
            map.insert(num, i as i32);
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;

        assert_eq!(Solution::two_sum(nums, target), vec![0, 1]);
    }
}
