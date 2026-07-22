impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut l = 0;
        let bytes = s.as_bytes();
        let mut set = HashSet::new();
        let mut max_count = 0;

        for r in 0..bytes.len() {
            while set.contains(&bytes[r]) {
                set.remove(&bytes[l]);
                l += 1;
            }
            set.insert(bytes[r]);
            max_count = max_count.max((r-l+1) as i32);
        }
        return max_count
    }
}
