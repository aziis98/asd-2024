use std::{collections::VecDeque, fmt::Debug};

pub struct RollingHash<T: Into<u64> + Clone> {
    modulus: u64,
    alphabet_size: u64,

    offset: u64,
    current_word: VecDeque<T>,
    hash: u64,
}

#[derive(Debug)]
pub struct Hashed {
    hash: u64,
    offset: u64,
}

fn wrapping_pow_correct(a: u64, b: u64) -> u64 {
    // // println!("Wrapping pow: {}^{}", a, b);

    // let mut result = 1u64;
    // for _ in 0..b {
    //     result = result.wrapping_mul(a);
    // }

    // // println!("=> {}", result);

    // result

    a.wrapping_pow(b as u32)
}

impl<T> RollingHash<T>
where
    T: Into<u64> + Clone + Debug,
{
    pub fn new(modulus: u64, alphabet_size: u64) -> Self {
        RollingHash {
            modulus,
            alphabet_size,

            offset: 0,
            current_word: VecDeque::new(),
            hash: 0,
        }
    }

    // pub fn hash(&self) -> u64 {
    //     self.hash % self.modulus
    // }

    pub fn hash(&self) -> Hashed {
        Hashed {
            hash: self.hash % self.modulus,
            offset: self.offset,
        }
    }

    pub fn compare(&self, lhs: &Hashed, rhs: &Hashed) -> bool {
        // println!("Comparing: {:?} {:?}", lhs, rhs);

        let (lhs, rhs) = if lhs.offset < rhs.offset {
            (lhs, rhs)
        } else {
            (rhs, lhs)
        };

        // Shift lhs to the right by the difference in offsets
        let shifted_lhs = (lhs.hash
            * wrapping_pow_correct(self.alphabet_size, rhs.offset - lhs.offset))
            % self.modulus;

        shifted_lhs == rhs.hash
    }

    pub fn hash_pattern(&self, pattern: &[T]) -> Hashed {
        let mut hash = 0;

        for (i, value) in pattern.iter().enumerate() {
            let char_hash =
                value.clone().into() * wrapping_pow_correct(self.alphabet_size, i as u64);

            hash += char_hash;
        }

        Hashed { hash, offset: 0 }
    }

    pub fn add_last(&mut self, value: T) {
        self.current_word.push_back(value.clone());

        let i = self.offset + (self.current_word.len() as u64) - 1;

        // println!("Alphabet size: {}", self.alphabet_size);
        // println!("Index: {}", i);

        // println!(
        //     "Adding: {:?} * {} to {}",
        //     value,
        //     wrapping_pow_correct(self.alphabet_size, i),
        //     self.hash
        // );
        self.hash += value.into() * wrapping_pow_correct(self.alphabet_size, i);
    }

    pub fn remove_first(&mut self) {
        let value = self.current_word.pop_front().unwrap();

        let i = self.offset;

        self.hash -= value.into() * wrapping_pow_correct(self.alphabet_size, i);

        self.offset += 1;
    }

    pub fn advance(&mut self, value: T) {
        self.add_last(value);
        self.remove_first();
    }

    pub fn hash_value_at(&self, h: &Hashed, pos: u64) -> u64 {
        let offset = h.offset;
        let hash = h.hash;

        let diff = pos as i64 - offset as i64;

        if diff < 0 {
            panic!("Invalid position");
        }

        (hash * wrapping_pow_correct(self.alphabet_size, diff as u64)) % self.modulus
    }

    pub fn hash_value_at_caret(&self, h: &Hashed) -> u64 {
        self.hash_value_at(h, self.offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rolling_hash() {
        println!("Rolling hash test");

        let modulus = 42;
        let alphabet_size = 4;

        let mut rh = RollingHash::<u64>::new(modulus, alphabet_size);
        let initial_pattern_hash = rh.hash_pattern(&[1, 2, 3, 4, 5]);
        println!("Initial pattern hash: {:?}", initial_pattern_hash);

        rh.add_last(1);
        rh.add_last(2);
        rh.add_last(3);
        rh.add_last(4);
        rh.add_last(5);

        println!("Hash: {:?}", rh.hash());

        rh.advance(0);

        println!("Hash: {:?}", rh.hash());

        rh.advance(1);
        rh.advance(2);
        rh.advance(3);
        rh.advance(4);
        rh.advance(5);

        println!("Current word: {:?}", rh.current_word);
        println!("Hash: {:?}", rh.hash());
        println!(
            "Shifted pattern hash: {}",
            rh.hash_value_at_caret(&initial_pattern_hash)
        );

        let pattern = initial_pattern_hash;
        let curr_hash = rh.hash();

        println!("Pattern hash: {:?}", pattern);
        println!("Current hash: {:?}", curr_hash);

        println!(
            "Pattern hash at caret: {}",
            rh.hash_value_at_caret(&pattern)
        );

        println!("Compare: {:?}", rh.compare(&pattern, &curr_hash));
    }

    #[test]
    fn test_geometry_rolling_hash() {
        println!("Geometry rolling hash test");

        let modulus = 10_000_000;
        let alphabet_size = 2;

        let mut rh = RollingHash::<u64>::new(modulus, alphabet_size);

        let initial_pattern_hash = rh.hash_pattern(&[1, 1, 1, 1]);

        rh.add_last(1);
        rh.add_last(1);
        rh.add_last(1);
        rh.add_last(1);

        println!("Initial pattern hash: {:?}", initial_pattern_hash);
        println!("Hash: {:?}", rh.hash());

        rh.advance(1);

        println!("Hash: {:?}", rh.hash());
        println!(
            "Shifted pattern hash: {:?}",
            rh.hash_value_at(&initial_pattern_hash, rh.offset)
        );
    }

    #[test]
    fn test_wrappping_pow() {
        println!("Wrapping pow test");

        let a = 2;
        let b = 3;

        let result = wrapping_pow_correct(a, b);

        assert_eq!(result, 8);

        let a = 3;
        let b = 100;
        let result = wrapping_pow_correct(a, b);

        assert_ne!(result, 0);
    }
}
