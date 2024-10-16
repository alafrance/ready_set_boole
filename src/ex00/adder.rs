pub fn adder(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        adder(a ^ b, (a & b) << 1)
    }
}

#[cfg(test)]
mod tests {
    use super::adder;

    #[test]
    fn test_adder_positive_numbers() {
        assert_eq!(adder(1, 1), 2);
        assert_eq!(adder(2, 3), 5);
        assert_eq!(adder(10, 20), 30);
    }

    #[test]
    fn test_adder_with_zero() {
        assert_eq!(adder(0, 0), 0);
        assert_eq!(adder(0, 5), 5);
        assert_eq!(adder(5, 0), 5);
    }

    #[test]
    fn test_overflow() {
        assert_eq!(adder(u32::MAX, 1), u32::MIN);
    }
}
