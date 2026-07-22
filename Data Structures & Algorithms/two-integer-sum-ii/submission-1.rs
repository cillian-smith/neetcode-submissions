impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut l, mut r) = (0, numbers.len()-1);

        while l < r {
            let sum = numbers[l] + numbers[r];
            match sum.cmp(&target) {
                std::cmp::Ordering::Equal => return vec![(l+1) as i32, (r+1) as i32],
                std::cmp::Ordering::Greater => r -=1,
                std::cmp::Ordering::Less => l += 1,
            }
        }
        vec![]
    }
}
