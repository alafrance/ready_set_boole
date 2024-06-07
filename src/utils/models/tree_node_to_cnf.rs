use crate::utils::rpn_op::rpn::is_operator;
use crate::utils::TreeNode;
impl TreeNode {
    pub fn to_cnf(&self) -> TreeNode {
        let mut cnf = self.clone();
        cnf.to_nnf_recursive();
        if !&cnf.is_cnf() {
            cnf.apply_distributivity_recursive();
        }
        cnf
    }

    pub fn is_cnf(&self) -> bool {
        match self.value {
            '&' => {
                if let Some(ref left) = self.left {
                    if !left.is_cnf() {
                        return false;
                    }
                }
                if let Some(ref right) = self.right {
                    if !right.is_cnf() {
                        return false;
                    }
                }
                true
            },
            '|' => {
                let is_literal_or_negation = |node: &TreeNode| -> bool {
                    match node.value {
                        '!' => {
                            if let Some(ref left) = node.left {
                                return !is_operator(left.value);
                            }
                            false
                        }
                        _ => !is_operator(node.value),
                    }
                };

                if let Some(ref left) = self.left {
                    if !is_literal_or_negation(left) {
                        return false;
                    }
                }
                if let Some(ref right) = self.right {
                    if !is_literal_or_negation(right) {
                        return false;
                    }
                }
                true
            },
            _ => true
        }
    }
    fn apply_distributivity_recursive(&mut self) {
        match self.value {
            '|' => {
                if let Some(ref mut left) = self.left {
                    if let Some(ref mut right) = self.right {
                        if left.value == '&' {
                            if let Some(ref mut a) = left.left {
                                if let Some(ref mut b) = left.right {
                                    let c = right.clone();
                                    let mut new_self = TreeNode::new('&');
                                    let mut new_left = TreeNode::new('|');
                                    let mut new_right = TreeNode::new('|');
                                    new_left.add_left(*a.clone());
                                    new_left.add_right(*b.clone());
                                    new_right.add_left(*a.clone());
                                    new_right.add_right(*c.clone());
                                    new_self.add_left(new_left);
                                    new_self.add_right(new_right);
                                    *self = new_self;
                                    self.apply_distributivity_recursive();
                                }
                            }
                        } else if right.value == '&' {
                            if let Some(ref mut b) = right.left {
                                if let Some(ref mut c ) = right.right {
                                    let a = left.clone();
                                    let mut new_self = TreeNode::new('&');
                                    let mut new_left = TreeNode::new('|');
                                    let mut new_right = TreeNode::new('|');
                                    new_left.add_left(*a.clone());
                                    new_left.add_right(*b.clone());
                                    new_right.add_left(*a.clone());
                                    new_right.add_right(*c.clone());
                                    new_self.add_left(new_left);
                                    new_self.add_right(new_right);
                                    *self = new_self;
                                    self.apply_distributivity_recursive();
                                }
                            }
                        }
                    }
                }
            }
            _ => {
                if let Some(ref mut left) = self.left {
                    left.apply_distributivity_recursive();
                }
                if let Some(ref mut right) = self.right {
                    right.apply_distributivity_recursive();
                }
            }
        }
    }
}
