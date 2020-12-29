//! EoPI pg 24
//! The parity of a binary word is 1 if the number of 1s in the word is odd; otherwise, it is 0.
//! For example, the parity of 1011 is 1, and the parity of 10001000 is 0.
//! Parity checks are used to detect single bit errors in data storage and communication.
//! It is fairly straightforward to write code that computes the parity of a single 64-bit word.
//! How would you compute the parity of a very large number of 64-bit words?
//! Hint: Use a lookup table, but don't use 2^64 entries!

use std::collections::HashMap;
use std::marker::PhantomData;
use std::mem::size_of;

/// A helper function for getting the number of bits of a type `T`.
fn n_bits<T>() -> usize {
    8 * size_of::<T>()
}

/// Fundamental Trait we will be implementing in this problem.
trait Parity<T> {
    fn parity(&mut self, x: T) -> u8;
}

/// recursively compute the parity of right shifted subwords
/// **NO** dynamic prog lookup in hashmap
struct Solution1 {}
impl Solution1 {
    fn new() -> Self {
        Solution1 {}
    }
}

impl Parity<usize> for Solution1 {
    fn parity(&mut self, x: usize) -> u8 {
        if x == 0 {
            //base case parity of 0 is 0
            0
        } else {
            //get lowest bit's parity and `XOR` it with rest
            ((x & 1) as u8) ^ self.parity(x >> 1)
        }
    }
}

/// iteratively fold a word in half `XOR`'ing all the way
/// **NO** dynamic prog lookup in hashmap
struct Solution2<T> {
    n_bits: T,
}

impl<T: From<usize>> Solution2<T> {
    fn new() -> Self {
        Solution2 {
            n_bits: T::from(n_bits::<T>()),
        }
    }
}

impl Parity<usize> for Solution2<usize> {
    fn parity(&mut self, x: usize) -> u8 {
        let mut ret = x ^ (x >> (self.n_bits / 2));
        let mut i = self.n_bits / 4;
        while i >= 1 {
            ret ^= ret >> i;
            i /= 2;
        }
        (ret & 1) as u8
    }
}

/// iteratively fold a word in half `XOR`'ing all the way
/// **YES** dynamic prog lookup in hashmap
struct Solution3<K, T> {
    /// K determins the maximum size of the hashmap
    /// e.g. u16 will fit all 16 bit unsigned integers 2<sup>16</sup> hashes
    n_bits: T,
    map: HashMap<T, u8>,
    _phantom: PhantomData<K>,
    _helper: Solution2<T>,
}

impl<K, T> Solution3<K, T> {
    fn new() -> Self
    where
        T: From<usize>,
    {
        Solution3 {
            n_bits: T::from(n_bits::<T>()),
            map: HashMap::new(),
            _phantom: PhantomData,
            _helper: Solution2::new(),
        }
    }
}

impl<K> Parity<usize> for Solution3<K, usize> {
    fn parity(&mut self, x: usize) -> u8 {
        let k_size = n_bits::<K>();
        let mut ret = x ^ (x >> (self.n_bits / 2));
        let mut i = self.n_bits / 4;
        while i >= 1 {
            if i <= k_size {
                match self.map.get(&ret) {
                    Some(p) => {
                        return *p;
                    }
                    None => {
                        let parity = self._helper.parity(ret);
                        self.map.insert(ret, parity);
                        return parity;
                    }
                }
            }
        }
        (ret & 1) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sols() {
        let mut sol1 = Solution1::new();
        let mut sol2: Solution2<usize> = Solution2::new();
        let mut sol3: Solution3<u16, usize> = Solution3::new();
        let mut sols: Vec<&mut dyn Parity<usize>> = vec![&mut sol1, &mut sol2, &mut sol3];
        for sol in sols.iter_mut() {
            assert_eq!(sol.parity(usize::from_str_radix("01", 2).unwrap()), 1);
            assert_eq!(
                sol.parity(usize::from_str_radix("111000111000111000111000", 2).unwrap()),
                0
            );
            assert_eq!(sol.parity((2 as usize).pow(5) - 1), 1);
            assert_eq!(sol.parity((2 as usize).pow(7) - 1), 1);
            assert_eq!(sol.parity((2 as usize).pow(9) - 1), 1);
            assert_eq!(sol.parity((2 as usize).pow(32) - 1), 0);
            assert_eq!(sol.parity((2 as usize).pow(60) - 1), 0);
        }
    }
}
