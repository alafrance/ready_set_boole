use crate::utils::TreeNode;
impl TreeNode {
    pub fn to_cnf(&self) -> TreeNode {
        let mut cnf = self.clone();
        cnf.to_nnf_or_cnf(true);
        cnf
    }
}
