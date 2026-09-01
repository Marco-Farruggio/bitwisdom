use core::ops::Range;
use crate::Bitwise;

macro_rules! impl_bitwise_float {
    ($ty:ty, $int_ty:ty, $name:ident, $range_name:ident) => {
        #[inline]
        const fn $name(index: usize) -> $int_ty {
            1 << index
        }

        #[inline]
        const fn $range_name(range: Range<usize>) -> $int_ty {
            ((1 << (range.end - range.start)) - 1) << range.start
        }

        impl Bitwise for $ty {
            #[inline]
            fn bit(&self, index: usize) -> bool {
                self.to_bits() & $name(index) != 0
            }

            #[inline]
            fn set_bit(&mut self, index: usize) {
                *self = <$ty>::from_bits(self.to_bits() | $name(index));
            }

            #[inline]
            fn clear_bit(&mut self, index: usize) {
                *self = <$ty>::from_bits(self.to_bits() & !$name(index));
            }

            #[inline]
            fn any_bits(&self, range: Range<usize>) -> bool {
                self.to_bits() & $range_name(range) != 0
            }

            #[inline]
            fn all_bits(&self, range: Range<usize>) -> bool {
                let mask = $range_name(range);
                self.to_bits() & mask == mask
            }

            #[inline]
            fn set_bits(&mut self, range: Range<usize>) {
                *self = <$ty>::from_bits(self.to_bits() | $range_name(range));
            }

            #[inline]
            fn clear_bits(&mut self, range: Range<usize>) {
                *self = <$ty>::from_bits(self.to_bits() & !$range_name(range));
            }
        }
    };
}

impl_bitwise_float!(f32, u32, f32_bit_mask, f32_range_mask);
impl_bitwise_float!(f64, u64, f64_bit_mask, f64_range_mask);
// future nightly-flag for f128