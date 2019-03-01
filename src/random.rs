//! # algorithm: [`Mersenne Twister Algorithm`](https://en.wikipedia.org/wiki/Mersenne_Twister).

#[derive(Debug, Clone)]
struct MersenneTwister {
    register: Vec<usize>,
    state: usize,
}

impl MersenneTwister {
    const N: usize = 624;
    const M: usize = 397;
    const A: usize = 0x9908_b0df;
    const B: usize = 0x9d2c_5680;
    const C: usize = 0xefc6_0000;
    const K_INIT_OPERAND: usize = 0x6c07_8965;
    const K_MAX_BITS: usize = 0xffff_ffff;
    const K_UPPER_BITS: usize = 0x8000_0000;
    const K_LOWER_BITS: usize = 0x7fff_ffff;

    #[inline]
    pub fn new(seed: usize) -> Self {
        Self::new_with_seed(seed)
    }

    fn new_with_seed(seed: usize) -> Self {
        let mut register = Vec::with_capacity(Self::N);
        let state = 0;

        register.push(seed);
        for i in 1..Self::N {
            let prev = register[i - 1];
            let temp = Self::K_INIT_OPERAND * (prev ^ (prev >> 30)) + i;
            register.push(temp & Self::K_MAX_BITS);
        }

        MersenneTwister { register, state }
    }

    fn twister(&mut self) {
        for i in 0..Self::N {
            let y = (self.register[i] & Self::K_UPPER_BITS)
                + (self.register[(i + 1) % Self::N] & Self::K_LOWER_BITS);
            self.register[i] = self.register[(i + Self::M) % Self::N] ^ (y >> 1);

            if y % 2 == 0 {
                self.register[i] ^= Self::A;
            }
        }
    }

    fn temper(&mut self) -> usize {
        if self.state == 0 {
            self.twister();
        }

        let mut y = self.register[self.state];
        y = y ^ (y >> 1);
        y = y ^ (y << 7) & Self::B;
        y = y ^ (y << 15) & Self::C;
        y = y ^ (y >> 18);

        self.state = (self.state + 1) % Self::N;

        y
    }

    #[inline]
    pub fn gen(&mut self) -> usize {
        self.temper()
    }

    #[inline]
    #[allow(dead_code)]
    pub fn reload_register(&mut self, register: Vec<usize>) {
        // Need to explain this function signature here.
        // Need to explain parameter `register`.
        self.state = 0;
        self.register = register;
    }
}

#[cfg(test)]
mod tests {
    use super::MersenneTwister;
    use std::collections::HashSet;

    #[test]
    fn test_mersenne_twister_algorithm() {
        let mut mt = MersenneTwister::new(0);
        let mut set = HashSet::new();

        let total: usize = 100;
        for _ in 0..total {
            let temp = mt.gen();
            set.insert(temp.clone());
        }
        assert_eq!(set.len(), total);
    }
}

#[derive(Clone)]
pub struct Random {
    generator: MersenneTwister,
}

impl Random {
    pub fn new(seed: usize) -> Self {
        Random {
            generator: MersenneTwister::new(seed),
        }
    }

    /// returns a random integer between two values. [min, max)
    /// min: lower bound value.
    /// max: upper bound value.
    #[inline]
    pub fn random_int(&mut self, min: usize, max: usize) -> usize {
        (self.generator.gen() + min) % max
    }

    // returns a random digit/number.
    // between 0 and 9. [0, 9]
    #[inline]
    pub fn random_digit(&mut self) -> usize {
        self.random_int(0, 10)
    }

    #[inline]
    pub fn choice<T: Clone>(&mut self, elements: &[T]) -> T {
        elements[self.random_int(0, elements.len())].clone()
    }
}
