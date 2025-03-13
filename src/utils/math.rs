pub trait CommonNumExt {
    fn gcd(self, b: Self) -> Self;
}

macro_rules! impl_common_num_ext {
    ($($ix:tt = $ux:tt),*) => {
        $(
            impl CommonNumExt for $ux {
                fn gcd(self, mut b: Self) -> Self {
                    let mut a = self;
                    if a == 0 || b == 0 { return a | b; }
                    let shift = (a | b).trailing_zeros();
                    a >>= a.trailing_zeros();
                    b >>= b.trailing_zeros();
                    while a != b {
                        if a > b { a -= b; a >>= a.trailing_zeros(); }
                        else { b -= a; b >>= b.trailing_zeros(); }
                    }
                    a << shift
                }
            }

            impl CommonNumExt for $ix {
                fn gcd(self, b: Self) -> Self {
                    fn w_abs(x: $ix) -> $ux { (if x.is_negative() { x.wrapping_neg() } else { x }) as _ }
                    w_abs(self).gcd(w_abs(b)) as _
                }
            }
        )*
    }
}
impl_common_num_ext!(i8 = u8, i16 = u16, i32 = u32, i64 = u64, i128 = u128, isize = usize);