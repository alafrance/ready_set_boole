pub(crate) mod rpn {

    pub fn is_operator(c: char) -> bool {
        c == '!' || c == '&' || c == '|' || c == '^' || c == '>' || c == '<' || c == '='
    }

    pub fn list_operand_bool() -> Vec<char> {
        vec!['0', '1']
    }

    pub fn list_operand_maj_letter() -> Vec<char> {
        ('A'..='Z').collect()
    }

    pub fn list_operand_min_letter() -> Vec<char> {
        ('a'..='z').collect()
    }
}
