mod rpn {
    use crate::ex04_print_truth_table::rpn::{eval_formula_with_values, extract_variables, generate_truth_table};
    pub fn sat(formula: &str) -> Result<bool, String> {
        let variables = extract_variables(&formula)?;
        let truth_table = generate_truth_table(variables.len());
        for row in truth_table {
            let formula = eval_formula_with_values(formula, &variables, &row)?;
            if formula {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use crate::ex07_sat::rpn::sat;

    #[test]
    fn all_tests_subject() {
        println!("{:?}", sat("AB|"));
        println!("{:?}", sat("AB&"));
        println!("{:?}", sat("AA!&"));
        println!("{:?}", sat("AA^"));
        println!("{:?}", sat("AA|!"));
        println!("{:?}", sat("AB&AB!&|"));
        println!("{:?}", sat("AB|AB|!&"));
    }
}