//! A no-std, const-friendly, no-unsafe bit manipulation library, written in pure Rust
//! Implemented for all primitives

#![no_std]

use core::ops::Range;

mod consts;
mod floats;
mod float_wise;

pub use consts::{bit, set_bit};
pub use float_wise::FloatWise;

pub trait Bitwise {
    fn bit(&self, index: usize) -> bool;
    fn set_bit(&mut self, index: usize);
    fn clear_bit(&mut self, index: usize);

    fn any_bits(&self, range: Range<usize>) -> bool;
    fn all_bits(&self, range: Range<usize>) -> bool;
    fn set_bits(&mut self, range: Range<usize>);
    fn clear_bits(&mut self, range: Range<usize>);
}

macro_rules! impl_bitwise {
    ($ty:ty, $name:ident, $range_name:ident) => {
        #[inline]
        const fn $name(index: usize) -> $ty {
            1 << index
        }

        /// Returns a mask with bits set in the specified range
        /// 
        /// A diagram of how this works:
        /// 11111111 >> 2 = 00111111
        /// 11111111 << 3 = 11111000
        ///                    &
        ///               = 00111000
        #[inline]
        const fn $range_name(range: Range<usize>) -> $ty {
            (!0 as $ty >> (<$ty>::BITS as usize - range.end))
                &
            (!0 as $ty << range.start)
        }

        impl Bitwise for $ty {
            #[inline]
            fn bit(&self, index: usize) -> bool {
                *self & $name(index) != 0
            }

            #[inline]
            fn set_bit(&mut self, index: usize) {
                *self |= $name(index);
            }

            #[inline]
            fn clear_bit(&mut self, index: usize) {
                *self &= !$name(index);
            }

            #[inline]
            fn any_bits(&self, range: Range<usize>) -> bool {
                *self & $range_name(range) != 0
            }

            #[inline]
            fn all_bits(&self, range: Range<usize>) -> bool {
                let mask = $range_name(range);
                *self & mask == mask
            }

            #[inline]
            fn set_bits(&mut self, range: Range<usize>) {
                *self |= $range_name(range);
            }

            #[inline]
            fn clear_bits(&mut self, range: Range<usize>) {
                *self &= !$range_name(range);
            }
        }
    };
}

// Rust-analyzer reports a false positive error here, E0109, frustrating
impl_bitwise!(u8,    u8_bit_mask,    u8_range_mask);
impl_bitwise!(u16,   u16_bit_mask,   u16_range_mask);
impl_bitwise!(u32,   u32_bit_mask,   u32_range_mask);
impl_bitwise!(u64,   u64_bit_mask,   u64_range_mask);
impl_bitwise!(u128,  u128_bit_mask,  u128_range_mask);
impl_bitwise!(usize, usize_bit_mask, usize_range_mask);

impl_bitwise!(i8,    i8_bit_mask,    i8_range_mask);
impl_bitwise!(i16,   i16_bit_mask,   i16_range_mask);
impl_bitwise!(i32,   i32_bit_mask,   i32_range_mask);
impl_bitwise!(i64,   i64_bit_mask,   i64_range_mask);
impl_bitwise!(i128,  i128_bit_mask,  i128_range_mask);
impl_bitwise!(isize, isize_bit_mask, isize_range_mask);