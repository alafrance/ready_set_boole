use crate::ex00::adder::adder;

pub fn multiplier(a: u32, b: u32) -> u32 {
    let mut factor = a;
    let mut multiplier = b;
    let mut result = 0;

    while multiplier != 0 {
        if multiplier & 1 != 0 {
            result = adder(result, factor);
        }
        multiplier >>= 1;
        factor <<= 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::multiplier;

    #[test]
    fn test_multiplier_positive_numbers() {
        assert_eq!(multiplier(1, 1), 1);
        assert_eq!(multiplier(2, 3), 6);
        assert_eq!(multiplier(10, 20), 200);
    }

    #[test]
    fn test_multiplier_with_zero() {
        assert_eq!(multiplier(0, 0), 0);
        assert_eq!(multiplier(0, 5), 0);
        assert_eq!(multiplier(5, 0), 0);
    }

    #[test]
    fn test_overflow() {
        assert_eq!(multiplier(u32::MAX, 2), u32::MAX - 1);
    }
}