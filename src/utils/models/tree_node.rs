use std::fmt::Display;

#[derive(Clone)]
pub struct TreeNode<T: Display> {
    pub value: T,
    pub left: Option<Box<TreeNode<T>>>,
    pub right: Option<Box<TreeNode<T>>>,
}

impl<T: Display> TreeNode<T> {
    // Créer un nouveau nœud
    pub fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    // Ajouter un fils gauche à un nœud
    pub(crate) fn add_left(&mut self, value: TreeNode<T>) {
        self.left = Some(Box::new(value));
    }

    // Ajouter un fils droit à un nœud
    pub(crate) fn add_right(&mut self, value: TreeNode<T>) {
        self.right = Some(Box::new(value));
    }

    // Méthode pour afficher l'arbre
    pub fn print_tree(&self) {
        self.print("", true);
    }

    // Fonction d'aide récursive pour imprimer l'arbre avec des indentations
    fn print(&self, prefix: &str, is_tail: bool) {
        println!("{}{} {}", prefix, if is_tail { "└──" } else { "├──" }, self.value);
        let new_prefix = if is_tail { "    " } else { "|   " };

        let children: Vec<&Box<TreeNode<T>>> = self.left.iter().chain(self.right.iter()).collect();
        if !children.is_empty() {
            for child in children.iter().take(children.len() - 1) {
                child.print(&format!("{}{}", prefix, new_prefix), false);
            }
        }
        if let Some(last_child) = children.last() {
            last_child.print(&format!("{}{}", prefix, new_prefix), true);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::TreeNode;

    #[test]
    fn test_create_tree() {
        let mut root = TreeNode::new(1);
        root.add_left(TreeNode::new(2));
        root.add_right(TreeNode::new(3));

        let left = root.left.as_mut().unwrap();
        left.add_left(TreeNode::new(4));
        left.add_right(TreeNode::new(5));

        let right = root.right.as_mut().unwrap();

        right.add_left(TreeNode::new(6));
        right.add_right(TreeNode::new(7));

        assert_eq!(root.value, 1);
        assert_eq!(root.left.as_ref().unwrap().value, 2);
        assert_eq!(root.right.as_ref().unwrap().value, 3);
        assert_eq!(root.left.as_ref().unwrap().left.as_ref().unwrap().value, 4);
        assert_eq!(root.left.as_ref().unwrap().right.as_ref().unwrap().value, 5);
        assert_eq!(root.right.as_ref().unwrap().left.as_ref().unwrap().value, 6);
        assert_eq!(root.right.as_ref().unwrap().right.as_ref().unwrap().value, 7);

        root.print_tree()
    }
}