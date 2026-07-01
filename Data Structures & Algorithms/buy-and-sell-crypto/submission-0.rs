impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut l: usize = 0;
        let mut r: usize = 1;
        let mut profit: i32 = 0;

        while r < prices.len() {
            if prices[l] < prices[r] {
                let curr_profit = prices[r] - prices[l];
                profit = std::cmp::max(curr_profit, profit)
            } else {
                l = r
            }
            r += 1
        }

        return profit;
    }
}
