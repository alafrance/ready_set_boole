mod set {
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
}

#[cfg(test)]
mod tests {
    use crate::ex08_powerset::set::powerset;

    #[test]
    fn sample_test() {
        println!("{:?}", powerset(vec![1, 2, 3]));
        println!("{:?}", powerset(vec![1, 2, 3, 4]));
    }
}