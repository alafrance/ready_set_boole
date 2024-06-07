pub(crate) mod rpn {
    use crate::utils::rpn_op::rpn::is_operator;

    pub fn restructure_rpn(formula: &str) -> String {
        let mut operands = Vec::new();
        let mut operators = Vec::new();

        for c in formula.chars() {
            if is_operator(c) {
                operators.push(c);
            } else {
                operands.push(c);
            }
        }

        operands.extend(operators);
        operands.into_iter().collect()
    }
}