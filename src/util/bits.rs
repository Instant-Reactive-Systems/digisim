pub use super::bitext::BitExt;
use BitsEnum::*;

#[derive(Debug, Clone, PartialEq, Eq)]
enum BitsEnum {
    Optimized(u64),
    Dynamic(Vec<bool>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bits {
    inner: BitsEnum,
    size: usize,
}

impl Bits {
    pub fn new(n: usize) -> Self {
        let inner = if n < 64 {
            Optimized(0)
        } else {
            let mut bits = Vec::new();
            bits.resize(n, false);
            Dynamic(bits)
        };

        Self {
            inner,
            size: n,
        }
    }

    pub fn set_bit(&mut self, n: usize, value: bool) {
        debug_assert!(n < self.size, "Cannot access out-of-bounds bit.");

        match &mut self.inner {
            Optimized(bits) => bits.set_bit_to(n, value),
            Dynamic(bits) => bits[n] = value,
        }
    }

    pub fn get_bit(&self, n: usize) -> bool {
        debug_assert!(n < self.size, "Cannot access out-of-bounds bit.");

        match &self.inner {
            Optimized(bits) => bits.check_bit(n),
            Dynamic(bits) => bits[n],
        }
    }

    pub fn to_number(&self) -> u64 {
        match &self.inner {
            Optimized(bits) => *bits & self.size as u64,
            Dynamic(bits) => bits.iter().rev().enumerate().map(|(i, &x)| i as u64 * x as u64).sum(),
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn to_vec(&self) -> Vec<bool> {
        match &self.inner {
            Optimized(bits) => {
                let mut res = Vec::with_capacity(self.size);
                for i in 0..self.size {
                    res.push(bits.check_bit(i));
                }
                res
            },
            Dynamic(bits) => bits.clone(),
        }
    }

    pub fn clear(&mut self) {
        match &mut self.inner {
            Optimized(bits) => bits.clear_all(),
            Dynamic(bits) => bits.iter_mut().for_each(|bit| *bit = false),
        }
    }
}
