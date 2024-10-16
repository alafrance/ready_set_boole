mod set {

    pub fn reverse_map(n: f64) -> Result<(u16, u16), &'static str>  {
        if n > u32::MAX as f64 {
            return Err("The number is too big");
        }
        let mut x: u32 = 0;
        let mut y: u32 = 0;
        let num: u32 = n as u32;
        for i in 0..16 {
            x |= ((num >> (2 * i)) & 1) << i;
            y |= ((num >> (2 * i + 1)) & 1) << i;
        }

        Ok((x as u16, y as u16))
    }

}

#[cfg(test)]
mod tests {
    use crate::ex11_reverse_map::set::reverse_map;

    #[test]
    fn test() {
        assert_eq!(reverse_map(0.0), Ok((0,0 )));
        assert_eq!(reverse_map(1.0), Ok((1,0 )));
        assert_eq!(reverse_map(2.0), Ok((0, 1)));
        assert_eq!(reverse_map(3.0), Ok((1, 1)));
        assert_eq!(reverse_map(4.0), Ok((2, 0)));
        assert_eq!(reverse_map(5.0), Ok((3, 0)));
        assert_eq!(reverse_map(6.0), Ok((2, 1)));
        assert_eq!(reverse_map(4294967295.0), Ok((u16::MAX, u16::MAX)));
        assert_eq!(reverse_map(f64::MAX), Err("The number is too big"));
        assert_eq!(reverse_map(u32::MAX as f64), Ok((u16::MAX, u16::MAX)));
        assert_eq!(reverse_map(1073741823.0), Ok((32767, 32767)));
        assert_eq!(reverse_map(1527779203.0), Ok((54321, 12345 )));
    }
}