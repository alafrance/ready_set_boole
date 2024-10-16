pub fn powerset(set: Vec<i32> ) -> Vec<Vec<i32>> {
    let mut result = vec![vec![]];
    for i in set {
        let mut new_result = result.clone();
        for subset in &mut new_result {
            subset.push(i);
        }
        result.append(&mut new_result);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::powerset;

    #[test]
    fn sample_test() {
        let vec = vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3]
        ];
        assert_eq!(powerset(vec![1, 2, 3]), vec);

        let vec = vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
            vec![4],
            vec![1, 4],
            vec![2, 4],
            vec![1, 2, 4],
            vec![3, 4],
            vec![1, 3, 4],
            vec![2, 3, 4],
            vec![1, 2, 3, 4],
        ];
        assert_eq!(powerset(vec![1, 2, 3, 4]), vec);
    }
}