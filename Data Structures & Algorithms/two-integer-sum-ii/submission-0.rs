impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut l, mut r) = (0, numbers.len()-1);

        while l < r {
            if numbers[r] + numbers[l] == target {return vec![(l+1) as i32, (r+1) as i32];}
            if numbers[l] + numbers[r] > target {
                r -= 1;
            } else {
                l += 1;
            }
        }
        vec![]
    }
}
