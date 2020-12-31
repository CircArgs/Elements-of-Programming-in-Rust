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
/// a naive solution using strings
struct StringSolution{}
impl StringSolution{
    fn new()->Self{
        StringSolution{}
    }
}

macro_rules! string_parity {
    ($t: ty) => {
        /// implementation of Parity for String solution
        /// iterate through the string and count the number of 1s and then send back whether the count is odd or even
        impl Parity<$t> for StringSolution {
            fn parity(&mut self, x: $t) -> u8 {
                let s = format!("{:b}", x);
                s.chars().map(|c| if c=='1' {1} else {0}).sum::<u8>()%2
            }
        }
    };
}

string_parity!(usize);

/// recursively compute the parity of right shifted subwords
/// **NO** dynamic prog lookup in hashmap
struct Solution1 {}
impl Solution1 {
    fn new() -> Self {
        Solution1 {}
    }
}

macro_rules! parity1 {
    ($t: ty) => {
        /// implementation of Parity for solution 1
        /// recursively pull the lowest bit off and xor it with the parity of the remainder of the word
        impl Parity<$t> for Solution1 {
            fn parity(&mut self, x: $t) -> u8 {
                if x == 0 {
                    //base case parity of 0 is 0
                    0
                } else {
                    //get lowest bit's parity and `XOR` it with rest
                    ((x & 1) as u8) ^ self.parity(x >> 1)
                }
            }
        }
    };
}

parity1!(usize);



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

macro_rules! parity2 {
    ($t: ty) => {
        /// implementation of Parity for solution 2
        /// ex. fold 32 bits into two 16 bit sequences and xor them, then repeat to 8, 4, 2, 1, done
        /// 0 = parity of 11010100 = parity of 1101 XOR 0100 = 1001 = parity of 10 XOR 01 = 11 = 1 XOR 1 = 0
        impl Parity<$t> for Solution2<$t> {
            fn parity(&mut self, x: $t) -> u8 {
                let mut ret = x ^ (x >> (self.n_bits / 2));
                let mut i = self.n_bits / 4;
                while i >= 1 {
                    ret ^= ret >> i;
                    i /= 2;
                }
                (ret & 1) as u8
            }
        }
    };
}

parity2!(usize);



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

macro_rules! parity3 {
    ($t: ty) => {
        /// implementation of Parity for solution 3
        /// same as solution 2 but starts attempting to lookup results once the size of the folding reaches the size of type K
        impl<K> Parity<$t> for Solution3<K, $t> {
            fn parity(&mut self, x: $t) -> u8 {
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
    };
}

parity3!(usize);



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pairty_sols() {
        let mut string_sol = StringSolution::new();
        let mut sol1 = Solution1::new();
        let mut sol2: Solution2<usize> = Solution2::new();
        let mut sol3: Solution3<u16, usize> = Solution3::new();
        let mut sols: Vec<&mut dyn Parity<usize>> = vec![&mut string_sol];//, &mut sol1, &mut sol2, &mut sol3];
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
