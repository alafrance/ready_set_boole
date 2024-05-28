mod rpn {
    use crate::utils::TreeNode;

    // pub fn calculate(formula: &str) -> Result<u32, String> {
    //     if !eval_formula(formula) {
    //         Err("Error formula".to_string())
    //     }
    //     else {
    //         Ok(0)
    //     }
    // }

    // pub fn eval_formula(formula: &str) -> bool {
        // formula.chars().all(|c| is_operator(c) || is_operand(c))
        // panic!("Not implemented");
        // false
    // }

    pub fn to_tree(formula: &str) -> Option<TreeNode<char>> {
        let mut stack = Vec::new();

        for c in formula.chars() {
            if is_operand(c) {
                stack.push(TreeNode::new(c));
            } else if is_operator(c) {
                let mut tree = TreeNode::new(c);
                let right = stack.pop();
                let left = stack.pop();
                tree.add_left(left.unwrap().value);
                tree.add_right(right.unwrap().value);
                stack.push(tree);
            } else {
                panic!("Error formula with this letter : {}", c)
            }
        };
        stack.pop()
    }


    // private functions
    fn is_operator(c: char) -> bool {
        c == '!' || c == '&' || c == '|' || c == '^' || c == '>' || c == '<' || c == '='
    }

    fn is_operand(c: char) -> bool {
        c == '0' || c == '1'
    }
}


#[cfg(test)]
mod tests {
    use crate::reverse_polish_notation::rpn::{to_tree};

    // #[test]
    // fn test_eval_formula() {
    //
    //     // 42 test
    //     assert_eq!(eval_formula("10&"), false);
    //     assert_eq!(eval_formula("10|"), true);
    //     assert_eq!(eval_formula("11>"), true);
    //     assert_eq!(eval_formula("10="), false);
    //     assert_eq!(eval_formula("1011||="), false);
    //
    //     // my test
    //     assert_eq!(eval_formula("21&1"), false);
    //     assert_eq!(eval_formula("1+1+1"), false);
    // }

    #[test]
    fn simple_test_tree() {
        let tree = to_tree("10&");
        match tree {
            Some(tree) => {
                tree.print_tree();
            },
            None => {
                println!("Error in formula i think");
                assert!(false);
            }
        }
    }

    #[test]
    fn test_depth_2_tree() {
        let tree = to_tree("1011||=");
        match tree {
            Some(tree) => {
                tree.print_tree();
            },
            None => {
                assert!(false);
            }
        }
    }
}