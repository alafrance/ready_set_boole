pub(crate) mod rpn {
    use crate::utils::char_to_bool::rpn::char_to_bool;
    use crate::utils::TreeNode;
    use crate::utils::rpn_op::rpn::{is_operator, list_operand_bool};

    pub fn eval_formula(formula: &str) -> Result<bool, String> {
        let tree = TreeNode::new_formula(formula, list_operand_bool())?;
        calculate_with_tree(tree)
    }

    fn calculate_with_tree(tree_node: TreeNode) -> Result<bool, String> {
        let operator = tree_node.value;

        if operator == '!' {
            let child = tree_node.left.unwrap();
            let child = match is_operator(child.value) {
                true => calculate_with_tree(*child),
                false => char_to_bool(child.value)
            }.unwrap();
            return Ok(!child);
        }
        let left_node = tree_node.left.unwrap();
        let right_node = tree_node.right.unwrap();

        let left_node = match is_operator(left_node.value) {
            true => calculate_with_tree(*left_node),
            false => char_to_bool(left_node.value)
        }.unwrap();

        let right_node = match is_operator(right_node.value) {
            true => calculate_with_tree(*right_node),
            false => char_to_bool(right_node.value),
        }.unwrap();

        match operator {
            '&' => Ok(left_node & right_node),
            '|' => Ok(left_node | right_node),
            '^' => Ok(left_node ^ right_node),
            '>' => Ok(!left_node | right_node),
            '=' => Ok(left_node == right_node),
            _ => Err("Error operator".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ex03_eval_formula::rpn::eval_formula;

    #[test]
    fn test_simple_formula() {
        assert_eq!(eval_formula("10&"), Ok(false));
        assert_eq!(eval_formula("11>"), Ok(true));
        assert_eq!(eval_formula("11^"), Ok(false));
        assert_eq!(eval_formula("10^"), Ok(true));
        assert_eq!(eval_formula("10>"), Ok(false));
        assert_eq!(eval_formula("11="), Ok(true));
        assert_eq!(eval_formula("10="), Ok(false));
    }

    #[test]
    fn test_complexe_formula() {
        assert_eq!(eval_formula("1011||="), Ok(true));
        assert_eq!(eval_formula("10|1&"), Ok(true));
        assert_eq!(eval_formula("101|&"), Ok(true));
        assert_eq!(eval_formula("10|0&"), Ok(false));
        assert_eq!(eval_formula("00|1&"), Ok(false));
        assert_eq!(eval_formula("10|1&"), Ok(true));
    }

    #[test]
    fn test_negation_formula() {
        assert_eq!(eval_formula("1!"), Ok(false));
        assert_eq!(eval_formula("0!"), Ok(true));
        assert_eq!(eval_formula("0!!!!!!"), Ok(false));
        assert_eq!(eval_formula("1!!!!!!"), Ok(true));

        assert_eq!(eval_formula("1!1&"), Ok(false));
    }
}