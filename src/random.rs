//! # algorithm: [`Mersenne Twister Algorithm`](https://en.wikipedia.org/wiki/Mersenne_Twister).

#[derive(Debug)]
struct MersenneTwister {
    register: Vec<usize>,
    state: usize,
}

const N: usize = 624;
const M: usize = 397;
const A: usize = 0x9908_b0df;
const B: usize = 0x9d2c_5680;
const C: usize = 0xefc6_0000;
const K_INIT_OPERAND: usize = 0x6c07_8965;
const K_MAX_BITS: usize = 0xffff_ffff;
const K_UPPER_BITS: usize = 0x8000_0000;
const K_LOWER_BITS: usize = 0x7fff_ffff;

impl MersenneTwister {
    fn new_with_seed(seed: usize) -> Self {
        let mut register = Vec::with_capacity(N);
        let state = 0;

        register.push(seed);
        for i in 1..N {
            let prev = register[i - 1];
            let temp = K_INIT_OPERAND * (prev ^ (prev >> 30)) + i;
            register.push(temp & K_MAX_BITS);
        }

        MersenneTwister { register, state }
    }

    #[inline]
    pub fn new(seed: usize) -> Self {
        Self::new_with_seed(seed)
    }

    fn twister(&mut self) {
        for i in 0..N {
            let y = (self.register[i] & K_UPPER_BITS) + (self.register[(i + 1) % N] & K_LOWER_BITS);
            self.register[i] = self.register[(i + M) % N] ^ (y >> 1);

            if y % 2 == 0 {
                self.register[i] ^= A;
            }
        }
    }

    fn temper(&mut self) -> usize {
        if self.state == 0 {
            self.twister();
        }

        let mut y = self.register[self.state];
        y = y ^ (y >> 1);
        y = y ^ (y << 7) & B;
        y = y ^ (y << 15) & C;
        y = y ^ (y >> 18);

        self.state = (self.state + 1) % N;

        y
    }

    #[inline]
    pub fn gen(&mut self) -> usize {
        self.temper()
    }

    #[inline]
    pub fn reload_register(&mut self, register: Vec<usize>) {
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
        for i in 0..total {
            let temp = mt.gen();
            set.insert(temp.clone());
        }
        assert_eq!(set.len(), total);
    }
}
