pub(crate) mod rpn {
    pub fn char_to_bool(c: char) -> Result<bool, String> {
        match c {
            '0' => Ok(false),
            '1' => Ok(true),
            _ => Err(format!("Invalid boolean character: {}", c)),
        }
    }
}
