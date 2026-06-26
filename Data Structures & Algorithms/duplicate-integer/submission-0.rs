impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut previously_seen = Vec::new();

        for number in nums.iter() {
            if previously_seen.contains(number) { return true;}
            previously_seen.push(*number)
        }
        return false;
    }
}
