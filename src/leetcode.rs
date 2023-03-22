pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut index = 0;
        for num in nums.iter() {
            let diff = target - num;
            if map.contains_key(&diff) {
                return vec![map.get(&diff).unwrap().clone(), index as i32];
            } else {
                map.insert(num.clone(), index as i32);
            }
            index += 1;
        }
        vec![-1, -1]
    }
}
