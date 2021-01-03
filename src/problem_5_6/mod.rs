//! EoPI pg 46 Buy and sell a stock once
//! This problem is concerned with the problem of optimally buying and selling a stock once,
//! as described on Page 2. As an example, consider the following sequence of stock prices:
//! <31.0,31.5,275,295,260,270,290,230,255,250>. The maximum profit that can be made with one buy
//! and one sell is 30-buy at 260 and sell at 290. Note that 250 is not the lowest price, nor 290 the
//! highest price.
//! 46Write
//! program that takes an array denoting the daily stock price, and retums the maximum profit
//! that could be made by buying and then selling one share of that stock. There is no need to buy if
//! a
//! no profit is possible.
//! Hint:ldentifying the minimum and maximum
//! height. Focus on valid differences.

use num::Zero;
use std::ops::Sub;

/// the primary solution
/// takes a slice `&[T]` and returns a T
/// where references to T can be subtracted (`impl Sub for &T {type Output = T}`) and are orderable (`impl Ord for T`)
/// the solution iterates through the slice once and for each element iterates through the remainder of the slice
/// for each element of the inner loop it find the difference which is the amount gained from selling
/// if a better result is found than what is currently had then that becomes the new solution
fn buy_sell<T>(prices: &[T]) -> T
where
    for<'a> &'a T: Sub<Output = T>,
    T: Ord + Zero,
{
    // track the soltuion
    let mut best: Option<T> = None;
    // iterate through all prices
    for i in 0..prices.len() {
        // and for each price following it take the difference to see what the profit would be
        for j in (i + 1)..prices.len() {
            let temp: T = &prices[j] - &prices[i];
            match best {
                // if the there is already a solution
                Some(ref mut d) => {
                    // and the current one is better the current one `temp` is the new best profit
                    if temp > *d {
                        *d = temp;
                    } // else current best is still best
                }
                // anything positive is greater than nothing
                None => {
                    if temp > T::zero() {
                        best = Some(temp);
                    }
                }
            }
        }
    }
    best.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_buy_sell() {
        let prices = vec![
            (vec![310, 315, 275, 295, 260, 270, 290, 230, 255, 250], 30),
            (vec![8, 11, 6, 19, 25, 4, 35, 15], 31),
        ];
        for (ps, ss) in prices {
            assert_eq!(buy_sell(&ps), ss);
        }
    }
}
