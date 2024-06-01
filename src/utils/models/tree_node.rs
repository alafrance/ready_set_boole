use crate::utils::rpn_op::rpn::is_operator;

#[derive(Clone)]
pub struct TreeNode {
    pub value: char,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    // Créer un nouveau nœud
    pub fn new(value: char) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
    pub fn new_formula(formula: &str, list_operand: Vec<char>) -> Result<TreeNode, String> {
        TreeNode::formula_to_tree(formula, list_operand)
    }


    // Ajouter un fils gauche à un nœud
    pub(crate) fn add_left(&mut self, new_value: TreeNode) {
        self.left = Some(Box::new(new_value));
    }

    // Ajouter un fils droit à un nœud
    pub(crate) fn add_right(&mut self, new_value: TreeNode) {
        self.right = Some(Box::new(new_value));
    }

    fn formula_to_tree(formula: &str, list_operand: Vec<char>) -> Result<TreeNode, String> {
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

    // Méthode pour afficher l'arbre
    pub fn print_tree(&self) {
        self.print("", true);
    }

    // Fonction d'aide récursive pour imprimer l'arbre avec des indentations
    fn print(&self, prefix: &str, is_tail: bool) {
        println!("{}{} {}", prefix, if is_tail { "└──" } else { "├──" }, self.value);
        let new_prefix = if is_tail { "    " } else { "|   " };

        let children: Vec<&Box<TreeNode>> = self.left.iter().chain(self.right.iter()).collect();
        if !children.is_empty() {
            for child in children.iter().take(children.len() - 1) {
                child.print(&format!("{}{}", prefix, new_prefix), false);
            }
        }
        if let Some(last_child) = children.last() {
            last_child.print(&format!("{}{}", prefix, new_prefix), true);
        }
    }

    pub fn to_rpn(&self) -> String {
        let mut rpn = String::new();
        self.to_rpn_recursive(&mut rpn);
        rpn
    }
    fn to_rpn_recursive(&self, rpn: &mut String) {
        if let Some(left) = &self.left {
            left.to_rpn_recursive(rpn);
        }
        if let Some(right) = &self.right {
            right.to_rpn_recursive(rpn);
        }
        rpn.push_str(&self.value.to_string());
    }

    pub fn to_nnf(&self) -> TreeNode {
        let mut nnf = self.clone();
        nnf.to_nnf_recursive();
        nnf
    }

    fn to_nnf_recursive(&mut self) {
        match self.value {
            '>' => {
                let mut new_self = TreeNode::new('|');
                new_self.add_left(TreeNode::new('!'));
                new_self.left.as_mut().unwrap().add_left(*self.left.take().unwrap());
                new_self.add_right(*self.right.take().unwrap());
                *self = new_self;
                self.to_nnf_recursive();
            },
            '=' => {
                let mut new_self = TreeNode::new('&');
                let mut left = TreeNode::new('|');
                left.add_left(TreeNode::new('!'));
                left.left.as_mut().unwrap().add_left(*self.left.take().unwrap());
                left.add_right(*self.right.take().unwrap());
                let mut right = TreeNode::new('|');
                right.add_left(TreeNode::new('!'));
                right.left.as_mut().unwrap().add_left(*self.right.take().unwrap());
                right.add_right(*self.left.take().unwrap());
                new_self.add_left(left);
                new_self.add_right(right);
                *self = new_self;
                self.to_nnf_recursive();
            },
            '!' => {
                if let Some(ref mut left) = self.left {
                    match left.value {
                        '!' => {
                            *self = *left.left.take().unwrap();
                            self.to_nnf_recursive();
                        },
                        // '&' => {
                        //     let mut new_self = TreeNode::new('|');
                        //     new_self.add_left(TreeNode::new('!'));
                        //     new_self.add_right(*left.left.take().unwrap());
                        //     *self = new_self;
                        //     self.to_nnf_recursive();
                        // },
                        // '|' => {
                        //     let mut new_self = TreeNode::new('&');
                        //     new_self.add_left(TreeNode::new('!'));
                        //     new_self.add_right(*left.left.take().unwrap());
                        //     *self = new_self;
                        //     self.to_nnf_recursive();
                        //
                        // },
                        _ => left.to_nnf_recursive()
                    }
                }
            },
            _ => {
                if let Some(ref mut left) = self.left {
                    left.to_nnf_recursive();
                }
                if let Some(ref mut right) = self.right {
                    right.to_nnf_recursive();
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::TreeNode;
    use crate::utils::rpn_op::rpn::{list_operand_bool, list_operand_maj_letter, list_operand_min_letter};

    #[test]
    fn test_create_tree() {
        let mut root = TreeNode::new('1');
        root.add_left(TreeNode::new('2'));
        root.add_right(TreeNode::new('3'));

        let left = root.left.as_mut().unwrap();
        left.add_left(TreeNode::new('4'));
        left.add_right(TreeNode::new('5'));

        let right = root.right.as_mut().unwrap();

        right.add_left(TreeNode::new('6'));
        right.add_right(TreeNode::new('7'));

        assert_eq!(root.value, '1');
        assert_eq!(root.left.as_ref().unwrap().value, '2');
        assert_eq!(root.right.as_ref().unwrap().value, '3');
        assert_eq!(root.left.as_ref().unwrap().left.as_ref().unwrap().value, '4');
        assert_eq!(root.left.as_ref().unwrap().right.as_ref().unwrap().value, '5');
        assert_eq!(root.right.as_ref().unwrap().left.as_ref().unwrap().value, '6');
        assert_eq!(root.right.as_ref().unwrap().right.as_ref().unwrap().value, '7');

        root.print_tree();
    }


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
}