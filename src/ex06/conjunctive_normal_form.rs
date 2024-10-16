use crate::utils::restructure_rpn::rpn::restructure_rpn;
use crate::utils::rpn_op::rpn::list_operand_maj_letter;
use crate::utils::TreeNode;

pub fn conjunctive_normal_form(formula: &str) -> Result<String, String> {
    let tree = TreeNode::new_formula(&restructure_rpn(formula), list_operand_maj_letter())?;
    let tree = tree.to_cnf();
    Ok(tree.to_rpn())
}

#[cfg(test)]
mod tests {
    use super::conjunctive_normal_form;

    #[test]
    fn test_sample() {
        assert_eq!(conjunctive_normal_form("AB&!"), Ok("A!B!|".to_string()));
        assert_eq!(conjunctive_normal_form("AB|!"), Ok("A!B!&".to_string()));
        assert_eq!(conjunctive_normal_form("AB|C&"), Ok("ABC|&".to_string()));
        assert_eq!(conjunctive_normal_form("AB|C|D|"), Ok("ABCD|||".to_string()));
        assert_eq!(conjunctive_normal_form("AB&C&D&"), Ok("ABCD&&&".to_string()));
        assert_eq!(conjunctive_normal_form("AB&!C!|"), Ok("AB|AC|&".to_string()));
        assert_eq!(conjunctive_normal_form("AB|!C!&"), Ok("ABC|&".to_string()));
    }
}