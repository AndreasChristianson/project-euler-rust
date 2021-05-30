pub fn pick<T: Copy>(input: &[T], n: usize) -> Vec<Vec<T>> {
    let mut ret = vec![];
    let count = input.len();

    if n == 0 {
        return vec![];
    }
    if n > count {
        return vec![];
    }
    if n == count {
        return vec![input.to_vec()];
    }
    if n == 1 {
        for i in 0..count {
            let current_pick = input.get(i).unwrap();
            ret.push(vec![*current_pick]);
        }
    } else {
        for i in 0..count - 1 {
            let current_pick = input.get(i).unwrap();
            let remaining = &input[i + 1..count];
            let sub_picks = pick(remaining, n - 1);
            for mut combination in sub_picks {
                combination.insert(0, *current_pick);
                ret.push(combination);
            }
        }
    }
    return ret;
}

#[cfg(test)]
mod pick_tests {
    use super::pick;
    #[test]
    fn empty_list() {
        assert_eq!(pick::<i32>(&vec![], 0), Vec::<Vec<i32>>::new());
    }
    #[test]
    fn one_element() {
        assert_eq!(pick(&vec![1], 0), Vec::<Vec<i32>>::new());
        assert_eq!(pick(&vec![1], 1), vec![vec![1]]);
    }
    #[test]
    fn two_elements() {
        assert_eq!(pick(&vec![1, 2], 1), vec![vec![1], vec![2]]);
        assert_eq!(pick(&vec![1, 2], 2), vec![vec![1, 2]]);
    }

    #[test]
    fn five_elements() {
        assert_eq!(
            pick(&vec![1, 2, 3, 4, 5], 1),
            vec![vec![1], vec![2], vec![3], vec![4], vec![5]]
        );
        assert_eq!(
            pick(&vec![1, 2, 3, 4, 5], 2),
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
                vec![1, 5],
                vec![2, 3],
                vec![2, 4],
                vec![2, 5],
                vec![3, 4],
                vec![3, 5],
                vec![4, 5]
            ]
        );

        assert_eq!(
            pick(&vec![1, 2, 3, 4, 5], 3),
            vec![
                vec![1, 2, 3],
                vec![1, 2, 4],
                vec![1, 2, 5],
                vec![1, 3, 4],
                vec![1, 3, 5],
                vec![1, 4, 5],
                vec![2, 3, 4],
                vec![2, 3, 5],
                vec![2, 4, 5],
                vec![3, 4, 5]
            ]
        );
    }
}

pub fn permute<T: Copy>(input: Vec<T>) -> Vec<Vec<T>> {
    let mut ret = vec![];
    let count = input.len();
    for n in 0..count + 1 {
        let mut picks = pick(&input, n);
        // println!("picks:{:?}, n:{}, count:{}", picks, n, count);
        ret.append(&mut picks);
    }
    return ret;
}

#[cfg(test)]
mod permute_tests {
    use super::permute;
    #[test]
    fn empty_list() {
        assert_eq!(permute::<i32>(vec![]), Vec::<Vec<i32>>::new());
    }
    #[test]
    fn one_element() {
        assert_eq!(permute(vec![1]), vec![vec![1]]);
    }
    #[test]
    fn two_elements() {
        assert_eq!(permute(vec![1, 2]), vec![vec![1], vec![2], vec![1, 2]]);
    }

    #[test]
    fn three_elements_lexicographic() {
        assert_eq!(
            permute(vec![1, 2, 3]),
            vec![
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

    #[test]
    fn five_elements_lexicographic() {
        assert_eq!(
            permute(vec![1, 2, 3, 4, 5]),
            vec![
                vec![1],
                vec![2],
                vec![3],
                vec![4],
                vec![5],
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
                vec![1, 5],
                vec![2, 3],
                vec![2, 4],
                vec![2, 5],
                vec![3, 4],
                vec![3, 5],
                vec![4, 5],
                vec![1, 2, 3],
                vec![1, 2, 4],
                vec![1, 2, 5],
                vec![1, 3, 4],
                vec![1, 3, 5],
                vec![1, 4, 5],
                vec![2, 3, 4],
                vec![2, 3, 5],
                vec![2, 4, 5],
                vec![3, 4, 5],
                vec![1, 2, 3, 4],
                vec![1, 2, 3, 5],
                vec![1, 2, 4, 5],
                vec![1, 3, 4, 5],
                vec![2, 3, 4, 5],
                vec![1, 2, 3, 4, 5],
            ]
        );
    }
}
