use crate::utils::TreeNode;

impl TreeNode {

    // Créer un nouveau nœud
    pub fn new(value: char) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    // Ajouter un fils gauche à un nœud
    pub(crate) fn add_left(&mut self, new_value: TreeNode) {
        self.left = Some(Box::new(new_value));
    }

    // Ajouter un fils droit à un nœud
    pub(crate) fn add_right(&mut self, new_value: TreeNode) {
        self.right = Some(Box::new(new_value));
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

    pub fn to_nnf_recursive(&mut self) {
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
                        _ => {
                            left.to_nnf_recursive();
                        }
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
}
