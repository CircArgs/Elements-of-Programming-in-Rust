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
    // a hashmap to lookup the characters in
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
    // track to see if we have seen a negative
    // if we have then it will need to be the last (rtl) element
    // and we will only be able to see one
    let mut neg_seen = false;
    // using iterators and chained methods to iterate over the string's characters in reverse, enumerate to get the index `i`
    // and for each we will transform the character to a number by lookup in our map and then multiply it by `10^i`

    let ret = s
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| {
            // we will only hit this if there was a negative in the string but not the leftmost character (last in rtl)
            if neg_seen {
                panic!("string should only contain a single '-' as the leftmost character")
            }
            10_i64.pow(i as u32)
                * if c == '-' {
                    neg_seen = true;
                    0
                } else {
                    // we will just let a panic occur if a character is not a number (not in our map)
                    *string_int_map.get(&c).unwrap()
                }
        })
        .sum();

    if neg_seen {
        return ret * -1;
    }
    ret
}

/// Integer to String
/// will use i64 for convenience
fn int_to_string(x: i64) -> String {
    // create a local copy of x to work on. will need to mutate for this method
    let mut x = x;
    // we will need to add a negative sign if x is negative
    let mut neg = false;
    if x < 0 {
        neg = true;
        // our loop below needs positive x
        x *= -1;
    }
    // we will store the digits as strings in this vec and collect them all reversed into a String to return
    let mut ret = vec![];
    // we will reduce x by a digit each time we add to our return vector
    while x > 0 {
        // we mod by 10 to get the rightmost digit
        ret.push((x % 10).to_string());
        //this reduces x by a digit (integer division)
        x /= 10;
    }
    //add negative sign accordingly
    if neg {
        ret.push("-".to_string());
    }
    // we ned to `rev` because we added the digits ltr but we want to read the number rtl
    ret.iter().rev().cloned().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_string_to_int() {
        assert_eq!(string_to_integer(&"456"), 456);
        assert_eq!(string_to_integer(&"-456"), -456);
    }
    #[test]
    #[should_panic(expected = "string should only contain a single '-' as the leftmost character")]
    fn test_string_to_int_panic_invalid_neg_within() {
        string_to_integer(&"45-6");
    }
    #[test]
    #[should_panic(expected = "string should only contain a single '-' as the leftmost character")]
    fn test_string_to_int_panic_invalid_negs() {
        string_to_integer(&"-45-6");
    }
    #[test]
    #[should_panic]
    fn test_string_to_int_panic_not_a_number() {
        string_to_integer(&"hello 456");
    }
    #[test]
    fn test_int_to_string() {
        assert_eq!(int_to_string(456), "456".to_string());
        assert_eq!(int_to_string(-456), "-456".to_string());
    }
}
