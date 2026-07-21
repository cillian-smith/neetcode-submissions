impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for val in nums.iter() {
            if let Some(v) = map.get(&val) { 
                return true;
            } else {
                map.insert(*val, 1);
            }
        }
        false
    }
}
