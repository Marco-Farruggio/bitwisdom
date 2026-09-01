use super::Bitwise;

/// Float-specific bitwise operations trait
/// 
/// Provides methods for working with the IEEE 754 representation of floats,
/// including sign, exponent, and mantissa manipulation.
pub trait FloatWise: Bitwise {
    const N_BITS: usize;
    const EXPONENT_BITS: usize;
    const MANTISSA_BITS: usize;

    /// Returns the number of bits in the exponent field
    fn exponent_bits() -> usize;
    
    /// Returns the number of bits in the mantissa (fraction) field
    fn mantissa_bits() -> usize;
    
    /// Returns the bit index of the sign bit
    fn sign_bit_index() -> usize;
    
    /// Returns the bit range of the exponent field
    fn exponent_range() -> core::ops::Range<usize>;
    
    /// Returns the bit range of the mantissa field
    fn mantissa_range() -> core::ops::Range<usize>;
    
    /// Gets the sign bit (0 for positive, 1 for negative)
    fn get_sign(&self) -> bool;
    
    /// Flips the sign bit
    fn flip_sign(&mut self);

    /// Sets the sign to positive
    fn set_positive(&mut self);

    /// Sets the sign to negative
    fn set_negative(&mut self);
    
    /// Extracts the exponent bits as an integer
    fn get_exponent(&self) -> u64;

    /// Sets the exponent bits from an integer
    fn set_exponent(&mut self, exponent: u64);

    /// Extracts the mantissa bits as an integer
    fn get_mantissa(&self) -> u64;

    /// Sets the mantissa bits from an integer
    fn set_mantissa(&mut self, mantissa: u64);
}

impl FloatWise for f32 {
    const N_BITS: usize = 32;
    const EXPONENT_BITS: usize = 8;
    const MANTISSA_BITS: usize = 23;

    fn exponent_bits() -> usize {
        Self::EXPONENT_BITS
    }

    fn mantissa_bits() -> usize {
        Self::MANTISSA_BITS
    }

    fn sign_bit_index() -> usize {
        Self::N_BITS - 1
    }

    fn exponent_range() -> core::ops::Range<usize> {
        Self::MANTISSA_BITS..Self::MANTISSA_BITS + Self::EXPONENT_BITS
    }

    fn mantissa_range() -> core::ops::Range<usize> {
        0..Self::MANTISSA_BITS
    }

    fn get_sign(&self) -> bool {
        self.bit(Self::sign_bit_index())
    }

    fn flip_sign(&mut self) {
        self.set_bit(Self::sign_bit_index());
    }

    fn set_positive(&mut self) {
        self.clear_bit(Self::sign_bit_index());
    }

    fn set_negative(&mut self) {
        self.set_bit(Self::sign_bit_index());
    }

    fn get_exponent(&self) -> u64 {
        let range = Self::exponent_range();
        ((self.to_bits() >> range.start) & ((1u32 << (range.end - range.start)) - 1)) as u64
    }

    fn set_exponent(&mut self, exponent: u64) {
        let range = Self::exponent_range();
        let mask = ((1u32 << (range.end - range.start)) - 1) << range.start;
        let bits = self.to_bits();
        *self = f32::from_bits((bits & !mask) | (((exponent as u32) << range.start) & mask));
    }

    fn get_mantissa(&self) -> u64 {
        let range = Self::mantissa_range();
        ((self.to_bits() >> range.start) & ((1u32 << (range.end - range.start)) - 1)) as u64
    }

    fn set_mantissa(&mut self, mantissa: u64) {
        let range = Self::mantissa_range();
        let mask = ((1u32 << (range.end - range.start)) - 1) << range.start;
        let bits = self.to_bits();
        *self = f32::from_bits((bits & !mask) | (((mantissa as u32) << range.start) & mask));
    }
}

impl FloatWise for f64 {
    const N_BITS: usize = 64;
    const EXPONENT_BITS: usize = 11;
    const MANTISSA_BITS: usize = 52;

    fn exponent_bits() -> usize {
        Self::EXPONENT_BITS
    }

    fn mantissa_bits() -> usize {
        Self::MANTISSA_BITS
    }

    fn sign_bit_index() -> usize {
        Self::N_BITS - 1
    }

    fn exponent_range() -> core::ops::Range<usize> {
        Self::MANTISSA_BITS..Self::MANTISSA_BITS + Self::EXPONENT_BITS
    }

    fn mantissa_range() -> core::ops::Range<usize> {
        0..Self::MANTISSA_BITS
    }

    fn get_sign(&self) -> bool {
        self.bit(Self::sign_bit_index())
    }

    fn flip_sign(&mut self) {
        self.set_bit(Self::sign_bit_index());
    }

    fn set_positive(&mut self) {
        self.clear_bit(Self::sign_bit_index());
    }

    fn set_negative(&mut self) {
        self.set_bit(Self::sign_bit_index());
    }

    fn get_exponent(&self) -> u64 {
        let range = Self::exponent_range();
        (self.to_bits() >> range.start) & ((1u64 << (range.end - range.start)) - 1)
    }

    fn set_exponent(&mut self, exponent: u64) {
        let range = Self::exponent_range();
        let mask = ((1u64 << (range.end - range.start)) - 1) << range.start;
        let bits = self.to_bits();
        *self = f64::from_bits((bits & !mask) | ((exponent << range.start) & mask));
    }

    fn get_mantissa(&self) -> u64 {
        let range = Self::mantissa_range();
        (self.to_bits() >> range.start) & ((1u64 << (range.end - range.start)) - 1)
    }

    fn set_mantissa(&mut self, mantissa: u64) {
        let range = Self::mantissa_range();
        let mask = ((1u64 << (range.end - range.start)) - 1) << range.start;
        let bits = self.to_bits();
        *self = f64::from_bits((bits & !mask) | ((mantissa << range.start) & mask));
    }
}