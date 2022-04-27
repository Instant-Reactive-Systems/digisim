

pub trait BitExt {
    fn check_bit(&self, n: usize) -> bool;
    fn set_bit_to(&mut self, n: usize, value: bool);
    fn set_bit(&mut self, n: usize);
    fn clear_bit(&mut self, n: usize);
}


macro_rules! bitext_impl_for {
    ($t: ty) => {
        impl BitExt for $t {
            fn check_bit(&self, n: usize) -> bool {
                *self & (1 << n) != 0
            }

            fn set_bit_to(&mut self, n: usize, value: bool) {
                const LUT: &[$t] = &[<$t>::MIN, <$t>::MAX];
                let value = LUT[value as usize];
                *self ^= (!value ^ *self) & (1 << n);
            }

            fn set_bit(&mut self, n: usize) {
                *self |= 1 << n;
            }

            fn clear_bit(&mut self, n: usize) {
                *self &= !(1 << n);
            }
        }
    };
}

bitext_impl_for!(u64);
bitext_impl_for!(u32);
bitext_impl_for!(u16);
bitext_impl_for!(u8);
