use crate::ex04::print_truth_table::{eval_formula_with_values, extract_variables, generate_truth_table};
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

#[cfg(test)]
mod tests {
    use super::sat;

    #[test]
    fn all_tests_subject() {
        assert_eq!(sat("AB|"), Ok(true));
        assert_eq!(sat("AB&"), Ok(true));
        assert_eq!(sat("AA!&"), Ok(false));
        assert_eq!(sat("AA^"), Ok(false));
        assert_eq!(sat("AA|!"), Ok(true));
        assert_eq!(sat("AB&AB!&|"), Ok(true));
        assert_eq!(sat("AB|AB|!&"), Ok(false));
    }
}