use crate::utils::TreeNode;
impl TreeNode {
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
                let self_left = self.left.take().unwrap();
                let self_right = self.right.take().unwrap();
                let mut new_self = TreeNode::new('&');
                new_self.add_left(TreeNode::new('|'));
                new_self.left.as_mut().unwrap().add_left(TreeNode::new('!'));
                new_self.left.as_mut().unwrap().left.as_mut().unwrap().add_left(*self_left.clone());
                new_self.left.as_mut().unwrap().add_right(*self_right.clone());
                new_self.add_right(TreeNode::new('|'));
                new_self.right.as_mut().unwrap().add_left(*self_left.clone());
                new_self.right.as_mut().unwrap().add_right(TreeNode::new('!'));
                new_self.right.as_mut().unwrap().right.as_mut().unwrap().add_left(*self_right.clone());
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
                        '|' => {
                            let mut new_self = TreeNode::new('&');
                            new_self.add_left(TreeNode::new('!'));
                            new_self.left.as_mut().unwrap().add_left(*left.left.take().unwrap());
                            new_self.add_right(TreeNode::new('!'));
                            new_self.right.as_mut().unwrap().add_left(*left.right.take().unwrap());
                            *self = new_self;
                            self.to_nnf_recursive();
                        },
                        '&' => {
                            let mut new_self = TreeNode::new('|');
                            new_self.add_left(TreeNode::new('!'));
                            new_self.left.as_mut().unwrap().add_left(*left.left.take().unwrap());
                            new_self.add_right(TreeNode::new('!'));
                            new_self.right.as_mut().unwrap().add_left(*left.right.take().unwrap());
                            *self = new_self;
                            self.to_nnf_recursive();
                        }
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
