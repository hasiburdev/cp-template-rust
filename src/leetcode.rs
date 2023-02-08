pub struct Solution {}

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        for number in nums {
            if set.contains(&number) {
                return true;
            }
            set.insert(number);
        }
        false
    }
}
