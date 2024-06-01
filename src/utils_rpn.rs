pub(crate) mod rpn {
    use crate::utils::TreeNode;

    pub fn to_tree(formula: &str, list_operand: Vec<char>) -> Result<TreeNode<char>, String> {
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
        if let Some(tree) = stack.pop() {
            Ok(tree)
        } else {
            return Err("Error: malformed formula (missing root node)".to_string())
        }
    }

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

    pub fn char_to_bool(c: char) -> Result<bool, String> {
        match c {
            '0' => Ok(false),
            '1' => Ok(true),
            _ => Err(format!("Invalid boolean character: {}", c)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils_rpn::rpn::{list_operand_bool, list_operand_maj_letter, list_operand_min_letter, to_tree};

    #[test]
    fn simple_test_tree() {
        let tree = to_tree("10&", list_operand_bool());
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
        let tree = to_tree("1011||=", list_operand_bool());

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
        let tree = to_tree("A!B&", list_operand_maj_letter());
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
        let tree = to_tree("a!b&", list_operand_min_letter());
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
        let tree = to_tree("A!B&", list_operand_min_letter());
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
        let tree = to_tree("A!B&", list_operand_bool());
        match tree {
            Ok(_) => {
                assert!(false);
            },
            Err(_) => {
                assert!(true);
            }
        }
    }
}




