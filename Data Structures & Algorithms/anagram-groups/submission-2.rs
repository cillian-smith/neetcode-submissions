use std::collections::HashMap;

impl Solution {
    pub fn sort_string(s: String) -> String {
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort_by(|a, b| a.cmp(b));
        chars.into_iter().collect()
    }

    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::with_capacity(strs.len());

        for s in strs {
            let sorted = Self::sort_string(s.clone());
            map.entry(sorted).or_default().push(s);
        }

        map.into_values().collect()
    }
}