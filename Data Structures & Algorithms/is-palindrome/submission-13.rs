impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        // sanitize 
        let cleaned: String = s.chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();
        let k : usize = cleaned.len();

        for i in 0..k/2 {
            if cleaned.chars().nth(i) != cleaned.chars().nth(k-i-1) {return false}
        }
        return true;

    }
}
