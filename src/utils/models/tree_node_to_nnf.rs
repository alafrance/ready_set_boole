use crate::utils::TreeNode;
impl TreeNode {
    pub fn to_nnf(&self) -> TreeNode {
        let mut nnf = self.clone();
        nnf.to_nnf_recursive();
        nnf
    }
}
