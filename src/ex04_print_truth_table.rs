mod rpn {
    use crate::ex03_eval_formula::rpn::eval_formula;

    pub fn print_truth_table(formula: &str) -> Result<(), String> {
        let variables = extract_variables(formula)?;
        let truth_table = generate_truth_table(variables.len());

        if formula.contains('1') || formula.contains('0') {
            return Err("Error in formula, formula contains 0 or 1".to_string());
        }

        let mut stdout = String::new();
        stdout += &*print_variables(&variables);
        for row in truth_table {

            stdout += "|";
            for value in &row {
                stdout += &format!(" {} |", if *value { "1" } else { "0" });
            }
            let formula = eval_formula_with_values(formula, &variables, &row);
            match formula {
                Ok(formula) => {
                    stdout += &format!(" {} |\n", if formula { "1" } else { "0" });
                },
                Err(e) => {
                    return Err(e);
                }
            }
        }
        println!("{}", stdout);
        Ok(())
    }

    fn eval_formula_with_values(formula: &str, variables: &Vec<String>, row: &Vec<bool>) -> Result<bool, String> {
        let mut formula = formula.to_string();
        for i in 0..variables.len() {
            formula = formula.replace(&variables[i], if row[i] { "1" } else { "0" });
        }
        eval_formula(&formula)
    }

    fn print_variables(variables: &Vec<String>) -> String {
        let header = variables.iter()
            .map(|var| format!(" {} |", var))
            .collect::<String>();
        let separator = variables.iter()
            .map(|_| "---|")
            .collect::<String>();

        format!("|{} = |\n|{}---|\n", header, separator)
    }

    fn extract_variables(expression: &str) -> Result<Vec<String>, String> {
        let mut variables = Vec::new();
        for c in expression.chars() {
            if c.is_ascii_uppercase() {
                let c = c.to_string();
                if !variables.contains(&c) {
                    variables.push(c.clone());
                } else {
                    return Err(format!("Error in formula, variable {} is duplicated", c));
                }
            }
        }
        if variables.is_empty() {
            return Err("Error in formula, no variables found".to_string());
        }
        Ok(variables)
    }

    fn generate_truth_table(n: usize) -> Vec<Vec<bool>> {
        let num_rows = 1 << n; // 2^n
        let mut table = Vec::with_capacity(num_rows);

        for i in 0..num_rows {
            let mut row = Vec::with_capacity(n);
            for j in 0..n {
                row.push((i & (1 << (n - j - 1))) != 0);
            }
            table.push(row);
        }

        table
    }
}

#[cfg(test)]
mod tests {
    use crate::ex04_print_truth_table::rpn::print_truth_table;

    #[test]
    fn test_print_truth_table() {
        assert!(print_truth_table("AB&C|").is_ok());
        assert!(print_truth_table("AB&C|D|E|").is_ok());
        assert!(print_truth_table("A!!").is_ok());
    }

    #[test]
    fn test_print_truth_table_no_variables() {
        assert!(print_truth_table("&|").is_err())
    }

    #[test]
    fn test_error_multiple_variables() {
        assert!(print_truth_table("AB&AC|").is_err());
    }

    #[test]
    fn test_error_formula() {
        assert!(print_truth_table("AB&1|").is_err());
        assert!(print_truth_table("").is_err());
        assert!(print_truth_table("ab&c|").is_err());
        assert!(print_truth_table("AB&|").is_err());
    }

}