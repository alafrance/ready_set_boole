mod nnf {
    use crate::utils::rpn_op::rpn::list_operand_maj_letter;
    use crate::utils::TreeNode;

    pub fn negation_normal_form(formula: &str) -> Result<String, String> {
        let tree = TreeNode::new_formula(&formula, list_operand_maj_letter())?;
        let tree = tree.to_nnf();
        Ok(tree.to_rpn())
    }
}

#[cfg(test)]
mod tests {
    use crate::ex05_negation_normal_form::nnf::negation_normal_form;

    #[test]
    fn test_negation_normal_form_negation() {
        assert_eq!(negation_normal_form("A!!!!"), Ok("A".to_string()));
        assert_eq!(negation_normal_form("A!!!!!"), Ok("A!".to_string()));
        assert_eq!(negation_normal_form("A"), Ok("A".to_string()));
    }

    #[test]
    fn test_nothing_changed() {
        assert_eq!(negation_normal_form("AB&"), Ok("AB&".to_string()));
        assert_eq!(negation_normal_form("AB|"), Ok("AB|".to_string()));
        assert_eq!(negation_normal_form("AB|C&"), Ok("AB|C&".to_string()));
    }

    #[test]
    fn test_simple_morgan_laws() {
        assert_eq!(negation_normal_form("AB="), Ok("A!B|AB!|&".to_string())); // improve it
        assert_eq!(negation_normal_form("AB>"), Ok("A!B|".to_string()));
    }

    #[test]
    fn test_simple_morgan_laws_2() {
        assert_eq!(negation_normal_form("AB&!"), Ok("A!B!|".to_string()));
        assert_eq!(negation_normal_form("AB|!"), Ok("A!B!&".to_string()));
        assert_eq!(negation_normal_form("AB&!C|"), Ok("A!B!|C|".to_string()));
    }
}