impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut seen = HashSet::with_capacity(nums.len());
        !nums.iter().all(|n| seen.insert(*n))
    }
}
