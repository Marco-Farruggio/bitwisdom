use crate::masks::bit_mask;

pub const fn bit(value: u32, index: usize) -> bool {
    value & bit_mask(index) != 0
}

pub const fn set_bit(value: u32, index: usize) -> u32 {
    value | bit_mask(index)
}