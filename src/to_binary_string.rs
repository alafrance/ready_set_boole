pub fn to_binary_string(num: i32) -> String {
    format!("{:b}", num)
}

#[cfg(test)]
mod tests {
    use crate::to_binary_string::to_binary_string;

    #[test]
    fn test_to_binary_string_positive_numbers() {
        assert_eq!(to_binary_string(1), "1");
        assert_eq!(to_binary_string(2), "10");
        assert_eq!(to_binary_string(3), "11");
        assert_eq!(to_binary_string(4), "100");
        assert_eq!(to_binary_string(5), "101");
        assert_eq!(to_binary_string(6), "110");
        assert_eq!(to_binary_string(7), "111");
        assert_eq!(to_binary_string(8), "1000");
        assert_eq!(to_binary_string(9), "1001");
        assert_eq!(to_binary_string(10), "1010");
    }
}