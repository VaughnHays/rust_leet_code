// mod two_sum;
// mod product_of_array_except_self;
// mod longest_consecutive_sequence;
//mod valid_palindrome;
// mod two_sum_sorted_input;
// mod TODO_three_sum;
// mod valid_parentheses;
mod best_time_to_buy_and_sell_stock;

fn main() {
    let res = best_time_to_buy_and_sell_stock::max_profit(vec![7, 1, 5, 3, 6, 4]);
    println!("{:?}", res)
}
