//! EoPI pg 68 Interconvert Strings and Integers
//! In this problem, you are to irnplement methods that take a string representing an integer and retum
//! the corresponding integer, and vice versa. Your code should handle negative integers. You cannot
//! use library functions like int in Python.
//! Implement an integer to string conversion function, and a string to integer conversison function,
//! For example, if the input to the first function is the integer 314,it should retum the string "31.4" and
//! if the input to the second function is the string "314" it should return the integer 314.
//! Hint: Build the result one digit at a time.

use std::collections::HashMap;

/// String to Integer
/// Will only go to i64 for convenience here
fn string_to_integer(s: &str) -> i64 {
    let string_int_map: HashMap<char, i64> = [
        ('0', 0),
        ('1', 1),
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
    ]
    .iter()
    .cloned()
    .collect();

    s.chars()
        .rev()
        .enumerate()
        .map(|(i, c)| 10_i64.pow(i as u32) * string_int_map.get(&c).unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_string_to_int() {
        let x = 5;
        assert_eq!(string_to_integer(&"456"), 456);
    }
}
