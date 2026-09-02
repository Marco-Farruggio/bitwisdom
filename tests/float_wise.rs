use bitwisdom::traits::FloatWise;

#[test]
fn f32_constants() {
    assert_eq!(f32::N_BITS, 32);
    assert_eq!(f32::EXPONENT_BITS, 8);
    assert_eq!(f32::MANTISSA_BITS, 23);
}

#[test]
fn f64_constants() {
    assert_eq!(f64::N_BITS, 64);
    assert_eq!(f64::EXPONENT_BITS, 11);
    assert_eq!(f64::MANTISSA_BITS, 52);
}