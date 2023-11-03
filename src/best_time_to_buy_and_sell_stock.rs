// You are given an array prices where prices[i] is the price of a given stock on the ith day.
// You want to maximize your profit by choosing a single day to buy one stock and choosing a
// different day in the future to sell that stock.Return the maximum profit you can achieve from
// this transaction. If you cannot achieve any profit, return 0.
pub fn max_profit(prices: Vec<i32>) -> i32 {
    prices
        .iter()
        .fold((0, i32::MAX), |(mut max_profit, mut buy), sell| {
            buy = i32::min(*sell, buy);
            max_profit = i32::max(max_profit, sell - buy);
            (max_profit, buy)
        })
        .0
}

// ALGORITHM:
// sell = ith item
// buy = min(price, cost)
// max_profit = max(max_profit, sell - buy)
// i++

/*
Example algorithm run-through:
 S
 B
[2, 3, 1, 0]
C = P
max_profit = P-C = 0

    S
 B
[2, 3, 1, 0]
C = C
max_profit = P-C = 1

       S
       B
[2, 3, 1, 0]
C = P
max_profit = max_profit = 1

          S
          B
[2, 3, 1, 0]
C = P
max_profit = max_profit = 1
return 1


*/
