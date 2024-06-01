mod nnf {
    // use crate::utils::utils_rpn::rpn::{list_operand_maj_letter, to_tree};

    pub fn negation_normal_form(formula: &str) -> Result<String, String> {
        // let tree = to_tree(&formula, list_operand_maj_letter())?;

        Ok("Not implemented yet".to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::ex05_negation_normal_form::nnf::negation_normal_form;

    #[test]
    fn test_negation_normal_form() {
        let formula = "1!!!!!!!";
        negation_normal_form(formula).expect("TODO: panic message");
        // assert_eq!(nnf, "Not implemented yet");
    }
}