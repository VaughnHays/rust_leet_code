// mod two_sum;
// mod product_of_array_except_self;
// mod longest_consecutive_sequence;
// mod valid_palindrome;
// mod two_sum_sorted_input;
// mod TODO_three_sum;
// mod valid_parentheses;
// mod best_time_to_buy_and_sell_stock;
mod min_stack;

fn main() {
    // let res = best_time_to_buy_and_sell_stock::max_profit(vec![7, 1, 5, 3, 6, 4]);
    // println!("{:?}", res)

    let mut m_stack = min_stack::MinStack::new();
    m_stack.push(-2);
    m_stack.push(0);
    m_stack.push(-3);
    let mut m = m_stack.get_min();
    println!["{}", m];
    m_stack.pop();
    m = m_stack.top();
    println!["{}", m];
    m = m_stack.get_min();
    println!["{}", m];
    // minStack.getMin(); // return -3
    // minStack.pop();
    // minStack.top();    // return 0
    // minStack.getMin(); // return -2
}
