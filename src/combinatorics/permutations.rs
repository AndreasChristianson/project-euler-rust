pub fn all_permutations<T>(input: Vec<T>) -> Vec<Vec<T>> {
    return vec![];
}

#[cfg(test)]
mod all_permutations_tests {
    use super::all_permutations;
    #[test]
    fn empty_list() {
        assert_eq!(all_permutations::<i32>(vec![]), Vec::<Vec<i32>>::new());
    }
    #[test]
    fn one_element() {
        assert_eq!(all_permutations(vec![1]), vec![vec![1]]);
    }
    #[test]
    fn two_elements() {
        assert_eq!(all_permutations(vec![1, 2]), vec![vec![1, 2], vec![2, 1]]);
    }

    #[test]
    fn three_elements_lexicographic() {
        assert_eq!(
            all_permutations(vec![1, 2, 3]),
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ]
        );
    }
}

pub fn all_permutations_with_missing<T>(input: Vec<T>) -> Vec<Vec<T>> {
    return vec![];
}

#[cfg(test)]
mod all_permutations_with_missing_tests {
    use super::all_permutations_with_missing;
    #[test]
    fn empty_list() {
        assert_eq!(all_permutations_with_missing::<i32>(vec![]), Vec::<Vec<i32>>::new());
    }
    #[test]
    fn one_element() {
        assert_eq!(
            all_permutations_with_missing(vec![1]),
            vec![vec![], vec![1]]
        );
    }
    #[test]
    fn two_elements() {
        assert_eq!(
            all_permutations_with_missing(vec![1, 2]),
            vec![vec![], vec![1], vec![2], vec![1, 2], vec![2, 1]]
        );
    }

    #[test]
    fn three_elements_lexicographic() {
        assert_eq!(
            all_permutations_with_missing(vec![1, 2, 3]),
            vec![
                vec![],
                vec![1],
                vec![2],
                vec![3],
                vec![1, 2],
                vec![1, 3],
                vec![2, 1],
                vec![2, 3],
                vec![3, 1],
                vec![3, 2],
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ]
        );
    }
}

pub fn unordered_permutations_with_missing<T>(input: Vec<T>) -> Vec<Vec<T>> {
  return vec![];
}

#[cfg(test)]
mod unordered_permutations_with_missing_tests {
    use super::unordered_permutations_with_missing;
    #[test]
    fn empty_list() {
        assert_eq!(unordered_permutations_with_missing::<i32>(vec![]), Vec::<Vec<i32>>::new());
    }
    #[test]
    fn one_element() {
        assert_eq!(
          unordered_permutations_with_missing(vec![1]),
            vec![vec![], vec![1]]
        );
    }
    #[test]
    fn two_elements() {
        assert_eq!(
          unordered_permutations_with_missing(vec![1, 2]),
            vec![vec![], vec![1], vec![2], vec![1, 2]]
        );
    }

    #[test]
    fn three_elements_lexicographic() {
        assert_eq!(
          unordered_permutations_with_missing(vec![1, 2, 3]),
            vec![
                vec![],
                vec![1],
                vec![2],
                vec![3],
                vec![1, 2],
                vec![1, 3],
                vec![2, 3],
                vec![1, 2, 3],
            ]
        );
    }
}