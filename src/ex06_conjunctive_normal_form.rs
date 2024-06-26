mod rpn {
    use crate::utils::restructure_rpn::rpn::restructure_rpn;
    use crate::utils::rpn_op::rpn::list_operand_maj_letter;
    use crate::utils::TreeNode;

    pub fn conjunctive_normal_form(formula: &str) -> Result<String, String> {
        let tree = TreeNode::new_formula(&restructure_rpn(formula), list_operand_maj_letter())?;
        let tree = tree.to_cnf();
        Ok(tree.to_rpn())
    }
}

#[cfg(test)]
mod tests {
    use crate::ex06_conjunctive_normal_form::rpn::conjunctive_normal_form;

    #[test]
    fn test_sample() {
        println!("{:?}", conjunctive_normal_form("AB&!"));
        println!("{:?}", conjunctive_normal_form("AB|!"));
        println!("{:?}", conjunctive_normal_form("AB|C&"));
        println!("{:?}", conjunctive_normal_form("AB|C|D|"));
        println!("{:?}", conjunctive_normal_form("AB&C&D&"));
        println!("{:?}", conjunctive_normal_form("AB&!C!|"));
        println!("{:?}", conjunctive_normal_form("AB|!C!&"));
    }
}