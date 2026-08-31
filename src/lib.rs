//! A no-std, const-friendly, no-unsafe bit manipulation library, written in pure Rust
//! Implemented for all primitives

#![no_std]

use core::ops::Range;

mod consts;
mod masks;

pub use masks::{bit_mask, range_mask};
pub use consts::{bit, set_bit};

pub trait Bitwise {
    fn bit(&self, index: usize) -> bool;
    fn set_bit(&mut self, index: usize);
    fn clear_bit(&mut self, index: usize);

    fn any_bits(&self, range: Range<usize>) -> bool;
    fn all_bits(&self, range: Range<usize>) -> bool;
    fn set_bits(&mut self, range: Range<usize>);
    fn clear_bits(&mut self, range: Range<usize>);
}

impl Bitwise for u32 {
    #[inline]
    fn bit(&self, index: usize) -> bool {
        *self & bit_mask(index) != 0
    }

    #[inline]
    fn set_bit(&mut self, index: usize) {
        *self |= bit_mask(index);
    }

    #[inline]
    fn clear_bit(&mut self, index: usize) {
        *self &= !bit_mask(index);
    }

    #[inline]
    fn any_bits(&self, range: Range<usize>) -> bool {
        *self & range_mask(range) != 0
    }

    #[inline]
    fn all_bits(&self, range: Range<usize>) -> bool {
        let masks = range_mask(range);
        *self & masks == masks
    }

    #[inline]
    fn set_bits(&mut self, range: Range<usize>) {
        *self |= range_mask(range);
    }

    #[inline]
    fn clear_bits(&mut self, range: Range<usize>) {
        *self &= !range_mask(range);
    }
}