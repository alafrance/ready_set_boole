
#[derive(Clone, Debug)]
pub struct TreeNode {
    pub value: char,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}