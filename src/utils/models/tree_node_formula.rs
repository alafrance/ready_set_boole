use crate::utils::rpn_op::rpn::is_operator;
use crate::utils::TreeNode;

impl TreeNode {
    pub fn new_formula(formula: &str, list_operand: Vec<char>) -> Result<TreeNode, String> {
        TreeNode::formula_to_tree(formula, list_operand)
    }

    pub fn formula_to_tree(formula: &str, list_operand: Vec<char>) -> Result<TreeNode, String> {
        let mut stack = Vec::new();

        for c in formula.chars() {
            if list_operand.contains(&c) {
                stack.push(TreeNode::new(c));
            } else if is_operator(c) {
                if c == '!' {
                    let mut tree = TreeNode::new(c);
                    if let Some(operand) = stack.pop() {
                        tree.add_left(operand);
                    } else {
                        return Err(format!("Error: malformed formula (missing operand for operator '{}'", c))
                    }
                    stack.push(tree);
                } else {
                    let mut tree = TreeNode::new(c);
                    if let Some(right) = stack.pop() {
                        tree.add_right(right);
                    } else {
                        return Err(format!("Error: malformed formula (missing right operand for operator '{}'", c))
                    }
                    if let Some(left) = stack.pop() {
                        tree.add_left(left);
                    } else {
                        return Err(format!("Error: malformed formula (missing left operand for operator '{}'", c))
                    }
                    stack.push(tree);
                }

            } else {
                return Err(format!("Error: invalid character in formula : '{}'", c))
            }
        };

        if stack.len() != 1 {
            return Err("Error: malformed formula (too many nodes)".to_string())
        }
        if let Some(tree) = stack.pop() {
            Ok(tree)
        } else {
            return Err("Error: malformed formula (missing root node)".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::rpn_op::rpn::{list_operand_bool, list_operand_maj_letter, list_operand_min_letter};
    use crate::utils::TreeNode;

    #[test]
    fn simple_test_tree() {
        let tree = TreeNode::new_formula("10&", list_operand_bool());
        match tree {
            Ok(tree) => {
                tree.print_tree();
            },
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn test_to_tree_with_depth() {
        let tree = TreeNode::new_formula("1011||=", list_operand_bool());

        match tree {
            Ok(tree) => {
                tree.print_tree();
            },
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn test_with_major_letter() {
        let tree = TreeNode::new_formula("A!B&", list_operand_maj_letter());
        match tree {
            Ok(tree) => {
                tree.print_tree();
            },
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn test_with_minor_letter() {
        let tree = TreeNode::new_formula("a!b&", list_operand_min_letter());
        match tree {
            Ok(tree) => {
                tree.print_tree();
            },
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn test_error_formula() {
        let tree = TreeNode::new_formula("A!B&", list_operand_min_letter());
        match tree {
            Ok(_) => {
                assert!(false);
            },
            Err(_) => {
                assert!(true);
            }
        }
    }

    #[test]
    fn test_error_formula_2() {
        let tree = TreeNode::new_formula("A!B&", list_operand_bool());
        match tree {
            Ok(_) => {
                assert!(false);
            },
            Err(_) => {
                assert!(true);
            }
        }
    }

    #[test]
    fn test_display_morgan_laws() {
        let tree = TreeNode::new_formula("A!B|AB!|&", list_operand_maj_letter());
        tree.unwrap().print_tree();
    }

    #[test]
    fn test_negation() {
        let tree = TreeNode::new_formula("AB!", list_operand_maj_letter());
        assert!(tree.is_err());
    }
}