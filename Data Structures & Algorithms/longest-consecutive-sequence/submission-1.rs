impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set : HashSet<_> = nums.into_iter().collect();
        let mut longest = 0;

        for num in &set {
            if !set.contains(&(num-1)) {
                let mut len = 1; 
                while set.contains(&(num+len)) {
                    len += 1;
                }
                longest = longest.max(len);
            }
        }
        longest
    }
}
