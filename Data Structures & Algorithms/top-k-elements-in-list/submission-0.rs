impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for n in nums {
            *map.entry(n).or_insert(0) += 1;
        }
        let mut list: Vec<_> = map.into_iter().collect();
        list.sort_by_key(|&(_, v)| std::cmp::Reverse(v));
        list.into_iter().take(k as usize).map(|(n, _)| n).collect()
    }
}
