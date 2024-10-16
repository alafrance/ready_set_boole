mod set {

    pub fn map(x: u16, y: u16) -> f64 {
        let mut result: u32 = 0;
        let x = x as u32;
        let y = y as u32;
        for i in 0..16 {
            result |= ((x >> i) & 1) << (2 * i);
            result |= ((y >> i) & 1) << (2 * i + 1);
        }

        // Le résultat est un entier sans signe, qu'on convertit en flottant
        result as f64
    }

}

#[cfg(test)]
mod tests {
    use crate::ex10_map::set::map;

    #[test]
    fn test() {
        assert_eq!(map(0, 0), 0.0);
        assert_eq!(map(1, 0), 1.0);
        assert_eq!(map(0, 1), 2.0);
        assert_eq!(map(1, 1), 3.0);
        assert_eq!(map(2, 0), 4.0);
        assert_eq!(map(0, 2), 8.0);
        assert_eq!(map(2, 2), 12.0);
        assert_eq!(map(u16::MAX, u16::MAX), u32::MAX as f64);
        assert_eq!(map(32767, 32767), 1073741823.0);
        assert_eq!(map(54321, 12345), 1527779203.0);
    }
}