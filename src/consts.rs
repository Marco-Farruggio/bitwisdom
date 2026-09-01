use crate::u32_bit_mask;
// need to make a macro here to generate these for all the integer types, but for now just implement for u32
pub const fn bit(value: u32, index: usize) -> bool {
    value & u32_bit_mask(index) != 0
}

pub const fn set_bit(value: u32, index: usize) -> u32 {
    value | u32_bit_mask(index)
}