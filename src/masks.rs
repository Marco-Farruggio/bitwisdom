use core::ops::Range;

#[inline]
pub const fn bit_mask(index: usize) -> u32 {
    1 << index
}

#[inline]
pub const fn range_mask(range: Range<usize>) -> u32 {
    ((1u32 << (range.end - range.start)) - 1) << range.start
}