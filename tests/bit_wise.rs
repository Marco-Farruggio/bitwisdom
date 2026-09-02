use bitwisdom::traits::Bitwise;

#[test]
fn bit() {
    let value = 0b1010_0100u8;

    assert!(value.bit(2));
    assert!(value.bit(5));
    assert!(!value.bit(0));
    assert!(!value.bit(1));
}

#[test]
fn set_bit() {
    let mut value = 0u8;

    value.set_bit(3);

    assert_eq!(value, 0b0000_1000);
    assert!(value.bit(3));
}

#[test]
fn clear_bit() {
    let mut value = 0b0000_1000u8;

    value.clear_bit(3);

    assert_eq!(value, 0);
    assert!(!value.bit(3));
}

#[test]
fn any_bits() {
    let value = 0b0001_1000u8;

    assert!(value.any_bits(3..6));
    assert!(value.any_bits(0..8));
    assert!(!value.any_bits(0..3));
}

#[test]
fn all_bits() {
    let value = 0b0001_1000u8;

    assert!(value.all_bits(3..5));
    assert!(!value.all_bits(0..8));
    assert!(!value.all_bits(0..4));
}

#[test]
fn set_bits() {
    let mut value = 0u8;

    value.set_bits(3..6);

    assert_eq!(value, 0b0011_1000);
    assert!(value.all_bits(3..6));
}

#[test]
fn clear_bits() {
    let mut value = 0b1111_1111u8;

    value.clear_bits(3..6);

    assert_eq!(value, 0b1100_0111);
    assert!(!value.any_bits(3..6));
}

#[test]
fn different_integer_types() {
    let mut u16_value = 0u16;
    u16_value.set_bits(8..12);

    assert_eq!(u16_value, 0x0F00);

    let mut u64_value = 0u64;
    u64_value.set_bit(63);

    assert_eq!(u64_value, 1u64 << 63);
}