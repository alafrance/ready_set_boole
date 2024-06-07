mod set {
    use crate::utils::rpn_op::rpn::{is_operator, list_operand_maj_letter};
    use crate::utils::TreeNode;

    pub fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Result<Vec<i32>, String> {
        let tree = TreeNode::new_formula(formula, list_operand_maj_letter())?.to_nnf();
        calculate_with_tree(tree, &sets)
    }

    fn calculate_with_tree(tree_node: TreeNode, sets: &Vec<Vec<i32>>) -> Result<Vec<i32>, String> {
        let operator = tree_node.value;

        let set1 =  match tree_node.left {
            Some(node) => Some(match is_operator(node.value) {
                true => calculate_with_tree(*node, sets),
                false => Ok({
                    let index = node.value as u32 - 'A' as u32;
                    let set = sets.get(index as usize).ok_or("Error index".to_string())?;
                    set.clone()
                }),
            }?),
            None => None,
        }.unwrap();

        let set2 = match tree_node.right {
            Some(node) => Some(match is_operator(node.value) {
                true => calculate_with_tree(*node, sets),
                false => Ok({
                    let index = node.value as u32 - 'A' as u32;
                    let set = sets.get(index as usize).ok_or("Error index".to_string())?;
                    set.clone()
                }),
            }?),
            None => None,
        };

        match operator {
            '&' => {
                // intersection
                let set2 = set2.unwrap();

                Ok(set1.iter().filter(|&x| set2.contains(x)).cloned().collect::<Vec<i32>>())
            },
            '|' => {
                // union
                let set2 = set2.unwrap();

                Ok(set1.iter().chain(set2.iter()).cloned().collect::<Vec<i32>>())
            },
            '^' => {
                // difference
                let set2 = set2.unwrap();

                Ok(set1.iter().filter(|&x| !set2.contains(x)).cloned().collect::<Vec<i32>>())
            },
            '!' => {
                // negation
                Ok(vec![])
            },
            _ => Err("Error operator".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ex09_eval_set::set::eval_set;

    #[test]
    fn test() {
        let sets = vec![
            vec![0, 1, 2],
            vec![0, 3, 4]
        ];
        println!("{:?}", eval_set("AB&", sets));

        let sets = vec![
            vec![0, 1, 2],
            vec![3, 4, 5]
        ];
        println!("{:?}", eval_set("AB|", sets));

        let sets = vec![
            vec![0, 1, 2],
        ];
        println!("{:?}", eval_set("A!", sets));

        let sets = vec![
            vec![0, 1, 2, 3, 4],
            vec![3, 4, 5]
        ];
        println!("{:?}", eval_set("AB^", sets));
    }
}