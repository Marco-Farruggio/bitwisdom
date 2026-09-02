use core::ops::Range;
use crate::masks::*;

macro_rules! const_fns_for {
    ($ty:ty, $ty_bit_mask:ident, $ty_range_mask:ident, $bit:ident, $set_bit:ident, $all_bits:ident, $any_bits:ident) => {
        /// gets a bit at the specified index
        #[inline]
        pub const fn $bit(value: $ty, index: usize) -> bool {
            value & $ty_bit_mask(index) != 0
        }

        /// sets a bit at the specified index to 1
        #[inline]
        pub const fn $set_bit(value: $ty, index: usize) -> $ty {
            value | $ty_bit_mask(index)
        }

        /// return true if all bits in a range are set
        #[inline]
        pub const fn $all_bits(value: $ty, range: Range<usize>) -> bool {
            let mask = $ty_range_mask(range);
            value & mask == mask
        }

        /// return true if any bits in a range are set
        #[inline]
        pub const fn $any_bits(value: $ty, range: Range<usize>) -> bool {
            value & $ty_range_mask(range) != 0
        }
    };
}

const_fns_for!(u8, u8_bit_mask, u8_range_mask, u8_bit, u8_set_bit, u8_all_bits, u8_any_bits);
const_fns_for!(u16, u16_bit_mask, u16_range_mask, u16_bit, u16_set_bit, u16_all_bits, u16_any_bits);
const_fns_for!(u32, u32_bit_mask, u32_range_mask, u32_bit, u32_set_bit, u32_all_bits, u32_any_bits);
const_fns_for!(u64, u64_bit_mask, u64_range_mask, u64_bit, u64_set_bit, u64_all_bits, u64_any_bits);
const_fns_for!(u128, u128_bit_mask, u128_range_mask, u128_bit, u128_set_bit, u128_all_bits, u128_any_bits);
const_fns_for!(usize, usize_bit_mask, usize_range_mask, usize_bit, usize_set_bit, usize_all_bits, usize_any_bits);

const_fns_for!(i8, i8_bit_mask, i8_range_mask, i8_bit, i8_set_bit, i8_all_bits, i8_any_bits);
const_fns_for!(i16, i16_bit_mask, i16_range_mask, i16_bit, i16_set_bit, i16_all_bits, i16_any_bits);
const_fns_for!(i32, i32_bit_mask, i32_range_mask, i32_bit, i32_set_bit, i32_all_bits, i32_any_bits);
const_fns_for!(i64, i64_bit_mask, i64_range_mask, i64_bit, i64_set_bit, i64_all_bits, i64_any_bits);
const_fns_for!(i128, i128_bit_mask, i128_range_mask, i128_bit, i128_set_bit, i128_all_bits, i128_any_bits);
const_fns_for!(isize, isize_bit_mask, isize_range_mask, isize_bit, isize_set_bit, isize_all_bits, isize_any_bits);