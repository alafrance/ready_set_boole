mod nnf {
    use crate::utils_rpn::rpn::{list_operand_maj_letter, to_tree};

    pub fn negation_normal_form(formula: &str) -> Result<String, String> {
        let tree = to_tree(&formula, list_operand_maj_letter())?;

    }
}

#[cfg(test)]
mod tests {
    use crate::negation_normal_form::nnf::negation_normal_form;

    #[test]
    fn test_negation_normal_form() {
        let formula = "1!!!!!!!";
        negation_normal_form(formula);
        // assert_eq!(nnf, "Not implemented yet");
    }
}