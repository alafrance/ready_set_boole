use crate::utils::TreeNode;

impl TreeNode {

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
}