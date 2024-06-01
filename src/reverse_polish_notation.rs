mod rpn {
    use crate::utils::TreeNode;

    pub fn eval_formula(formula: &str) -> Result<bool, String> {
        let tree = to_tree(formula);
        match tree {
            Some(tree) => calculate_with_tree(tree),
            None => Err("Error formula".to_string())
        }
    }

    pub fn to_tree(formula: &str) -> Option<TreeNode<char>> {
        let mut stack = Vec::new();

        for c in formula.chars() {
            if is_operand(c) {
                stack.push(TreeNode::new(c));
            } else if is_operator(c) {
                let mut tree = TreeNode::new(c);
                let right = stack.pop();
                let left = stack.pop();
                tree.add_left(left.unwrap());
                tree.add_right(right.unwrap());
                stack.push(tree);
            } else {
                panic!("Error formula with this letter : {}", c)
            }
        };
        stack.pop()
    }

    fn calculate_with_tree(tree_node: TreeNode<char>) -> Result<bool, String> {
        let operator = tree_node.value;
        let left_node = tree_node.left.unwrap();
        let right_node = tree_node.right.unwrap();

        let left_node = match is_operator(left_node.value) {
            true => calculate_with_tree(*left_node),
            false => char_to_bool(left_node.value)
        }.unwrap();

        let right_node = match is_operator(right_node.value) {
            true => calculate_with_tree(*right_node),
            false => char_to_bool(right_node.value),
        }.unwrap();

        match operator {
            '!' => Ok(left_node != right_node),
            '&' => Ok(left_node & right_node),
            '|' => Ok(left_node | right_node),
            '^' => Ok(left_node ^ right_node),
            '>' => Ok(!left_node | right_node),
            '=' => Ok(left_node == right_node),
            _ => Err("Error operator".to_string())
        }
    }

    // exercice 2

    pub fn print_truth_table(formula: &str) {
        let variables = extract_variables(formula).unwrap();
        if variables.len() == 0 {
            println!("No variables in formula");
            return;
        }
        let truth_table = generate_truth_table(variables.len());

        print_variables(&variables);
        for row in truth_table {
            print!("|");
            for value in &row {
                print!(" {} |", if *value { "1" } else { "0" });
            }
            println!(" {} |", if eval_formula_with_values(formula, &variables, &row) { "1" } else { "0" });
        }
    }

    fn eval_formula_with_values(formula: &str, variables: &Vec<String>, row: &Vec<bool>) -> bool {
        let mut formula = formula.to_string();
        for i in 0..variables.len() {
            formula = formula.replace(&variables[i], if row[i] { "1" } else { "0" });
        }
        let new_formula = formula.replace(' ', "");
        eval_formula(&new_formula).unwrap()
    }

    fn print_variables(variables: &Vec<String>) {
        print!("|");
        for var in variables {
            print!(" {} |", var);
        }
        println!(" = |");

        for _ in variables {
            print!("|---");
        }
        println!("|---|");
    }

    fn extract_variables(expression: &str) -> Result<Vec<String>, String> {
        let mut variables = Vec::new();
        for c in expression.chars() {
            if c.is_alphabetic() {
                let c = c.to_string();
                if !variables.contains(&c) {
                    variables.push(c.clone());
                } else {
                    return Err(format!("Error in formula, variable {} is duplicated", c));
                }
            }
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


    // private functions
    fn is_operator(c: char) -> bool {
        c == '!' || c == '&' || c == '|' || c == '^' || c == '>' || c == '<' || c == '='
    }

    fn is_operand(c: char) -> bool {
        c == '0' || c == '1'
    }

    fn char_to_bool(c: char) -> Result<bool, String> {
        match c {
            '0' => Ok(false),
            '1' => Ok(true),
            _ => Err(format!("Invalid boolean character: {}", c)),
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::reverse_polish_notation::rpn::{eval_formula, print_truth_table, to_tree};

    #[test]
    fn simple_test_tree() {
        let tree = to_tree("10&");
        match tree {
            Some(tree) => {
                tree.print_tree();
            },
            None => {
                println!("Error in formula i think");
                assert!(false);
            }
        }
    }

    #[test]
    fn test_to_tree_with_depth() {
        let tree = to_tree("1011||=");

        match tree {
            Some(tree) => {
                tree.print_tree();
            },
            None => {
                assert!(false);
            }
        }
    }

    #[test]
    fn test_calculate() {
        assert_eq!(eval_formula("1011||="), Ok(true));
        assert_eq!(eval_formula("10&"), Ok(false));
        assert_eq!(eval_formula("10|1&"), Ok(true));
        assert_eq!(eval_formula("101|&"), Ok(true));
        assert_eq!(eval_formula("10|0&"), Ok(false));
        assert_eq!(eval_formula("00|1&"), Ok(false));
        assert_eq!(eval_formula("11>"), Ok(true));
        assert_eq!(eval_formula("11^"), Ok(false));
        assert_eq!(eval_formula("10^"), Ok(true));
        assert_eq!(eval_formula("10>"), Ok(false));
        assert_eq!(eval_formula("11="), Ok(true));
        assert_eq!(eval_formula("10="), Ok(false));
    }

    #[test]
    fn test_print_truth_table() {
        print_truth_table("AB&C|");
    }
}