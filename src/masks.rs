use core::ops::Range;

macro_rules! const_masks_for {
    ($ty:ty, $name:ident, $range_name:ident) => {
        #[inline]
        pub const fn $name(index: usize) -> $ty {
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
        pub const fn $range_name(range: Range<usize>) -> $ty {
            (!0 as $ty >> (<$ty>::BITS as usize - range.end))
                &
            (!0 as $ty << range.start)
        }
    };
}

// Rust-analyzer reports a false positive error here, E0109, frustrating
const_masks_for!(u8, u8_bit_mask, u8_range_mask);
const_masks_for!(u16, u16_bit_mask, u16_range_mask);
const_masks_for!(u32, u32_bit_mask, u32_range_mask);
const_masks_for!(u64, u64_bit_mask, u64_range_mask);
const_masks_for!(u128, u128_bit_mask, u128_range_mask);
const_masks_for!(usize, usize_bit_mask, usize_range_mask);

const_masks_for!(i8, i8_bit_mask, i8_range_mask);
const_masks_for!(i16, i16_bit_mask, i16_range_mask);
const_masks_for!(i32, i32_bit_mask, i32_range_mask);
const_masks_for!(i64, i64_bit_mask, i64_range_mask);
const_masks_for!(i128, i128_bit_mask, i128_range_mask);
const_masks_for!(isize, isize_bit_mask, isize_range_mask);