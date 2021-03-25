use std::cmp::min;

pub fn traverse_all_directions<T: Copy>(input: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut ret = vec![];
    ret.append(&mut traverse_ltr_ttb(&input));
    ret.append(&mut traverse_ttb_ltr(&input));
    ret.append(&mut traverse_diagonally_tl_br(&input));
    ret.append(&mut traverse_diagonally_tr_bl(&input));

    return ret;
}

fn traverse_ltr_ttb<T: Copy>(input: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    return input.clone();
}

#[cfg(test)]
mod traverse_ltr_ttb_tests {
    use super::traverse_ltr_ttb;
    #[test]
    fn simple() {
        let actual = traverse_ltr_ttb::<i32>(&vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8]]);
        assert_eq!(actual[0][0], 1);
        assert_eq!(actual[0][1], 2);
        assert_eq!(actual[0][2], 3);
        assert_eq!(actual[0][3], 4);
        assert_eq!(actual[1][0], 5);
        assert_eq!(actual[1][1], 6);
        assert_eq!(actual[1][2], 7);
        assert_eq!(actual[1][3], 8);
    }

    #[test]
    fn one_dimensional() {
        let actual = traverse_ltr_ttb::<i32>(&vec![vec![1, 2, 3]]);
        assert_eq!(actual[0][0], 1);
        assert_eq!(actual[0][1], 2);
        assert_eq!(actual[0][2], 3);
    }

    #[test]
    fn singular() {
        let actual = traverse_ltr_ttb::<i32>(&vec![vec![9]]);
        assert_eq!(actual[0][0], 9);
    }

    #[test]

    fn tower() {
        let actual = traverse_ltr_ttb::<i32>(&vec![vec![1], vec![2], vec![3], vec![4]]);
        assert_eq!(actual[0][0], 1);
        assert_eq!(actual[1][0], 2);
        assert_eq!(actual[2][0], 3);
        assert_eq!(actual[3][0], 4);
    }
}

fn traverse_ttb_ltr<T: Copy>(input: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let col_length = input[0].len();
    let row_length = input.len();
    let mut ret = vec![vec![]; col_length];

    for r in 0..row_length {
        for c in 0..col_length {
            ret[c].push(input[r][c])
        }
    }
    return ret;
}

#[cfg(test)]
mod traverse_ttb_ltr_tests {
    use super::traverse_ttb_ltr;
    #[test]
    fn simple() {
        let actual = traverse_ttb_ltr::<i32>(&vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8]]);
        assert_eq!(actual[0][0], 1);
        assert_eq!(actual[0][1], 5);
        assert_eq!(actual[1][0], 2);
        assert_eq!(actual[1][1], 6);
        assert_eq!(actual[2][0], 3);
        assert_eq!(actual[2][1], 7);
        assert_eq!(actual[3][0], 4);
        assert_eq!(actual[3][1], 8);
    }

    #[test]
    fn one_dimensional() {
        let actual = traverse_ttb_ltr::<i32>(&vec![vec![1, 2, 3]]);
        assert_eq!(actual[0][0], 1);
        assert_eq!(actual[1][0], 2);
        assert_eq!(actual[2][0], 3);
    }

    #[test]
    fn singular() {
        let actual = traverse_ttb_ltr::<i32>(&vec![vec![9]]);
        assert_eq!(actual[0][0], 9);
    }

    #[test]

    fn tower() {
        let actual = traverse_ttb_ltr::<i32>(&vec![vec![1], vec![2], vec![3], vec![4]]);
        assert_eq!(actual[0][0], 1);
        assert_eq!(actual[0][1], 2);
        assert_eq!(actual[0][2], 3);
        assert_eq!(actual[0][3], 4);
    }
}

fn traverse_diagonally_tl_br<T: Copy>(input: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let row_count = input.len();
    if row_count == 0 {
        return vec![];
    }
    let col_count = input[0].len();
    let mut ret = vec![vec![]; col_count + row_count - 1];
    let mut output_slot = 0;

    for r in (0..row_count).rev() {
        for i in 0..min(row_count - r, col_count) {
            ret[output_slot].push(input[r + i][i]);
        }
        output_slot += 1;
    }
    if col_count > 1 {
        for c in 1..col_count {
            for i in 0..min(col_count - c, row_count) {
                ret[output_slot].push(input[i][c + i]);
            }
            output_slot += 1;
        }
    }

    return ret;
}

#[cfg(test)]
mod traverse_diagonally_tl_br_tests {
    use super::traverse_diagonally_tl_br;
    #[test]
    fn wide() {
        let actual = traverse_diagonally_tl_br::<i32>(&vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8]]);
        assert_eq!(actual[0][0], 5);
        assert_eq!(actual[1][0], 1);
        assert_eq!(actual[1][1], 6);
        assert_eq!(actual[2][0], 2);
        assert_eq!(actual[2][1], 7);
        assert_eq!(actual[3][0], 3);
        assert_eq!(actual[3][1], 8);
        assert_eq!(actual[4][0], 4);
    }

    #[test]
    fn tall() {
        let actual =
            traverse_diagonally_tl_br::<i32>(&vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]]);
        assert_eq!(actual[0][0], 7);
        assert_eq!(actual[1][0], 5);
        assert_eq!(actual[1][1], 8);
        assert_eq!(actual[2][0], 3);
        assert_eq!(actual[2][1], 6);
        assert_eq!(actual[3][0], 1);
        assert_eq!(actual[3][1], 4);
        assert_eq!(actual[4][0], 2);
    }

    #[test]
    fn one_dimensional() {
        let actual = traverse_diagonally_tl_br::<i32>(&vec![vec![1, 2, 3]]);
        assert_eq!(actual[0][0], 1);
        assert_eq!(actual[1][0], 2);
        assert_eq!(actual[2][0], 3);
    }

    #[test]
    fn singular() {
        let actual = traverse_diagonally_tl_br::<i32>(&vec![vec![9]]);
        assert_eq!(actual[0][0], 9);
    }

    #[test]

    fn tower() {
        let actual = traverse_diagonally_tl_br::<i32>(&vec![vec![1], vec![2], vec![3], vec![4]]);
        assert_eq!(actual[0][0], 4);
        assert_eq!(actual[1][0], 3);
        assert_eq!(actual[2][0], 2);
        assert_eq!(actual[3][0], 1);
    }
    #[test]
    fn empty() {
        let actual = traverse_diagonally_tl_br::<i32>(&vec![]);
        assert!(actual.is_empty());
    }
}

fn traverse_diagonally_tr_bl<T: Copy>(input: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let row_count = input.len();
    if row_count == 0 {
        return vec![];
    }
    let col_count = input[0].len();
    let mut ret = vec![vec![]; col_count + row_count - 1];
    let mut output_slot = 0;

    for c in 0..col_count {
        for i in 0..min(c + 1, row_count) {
            ret[output_slot].push(input[i][c - i]);
        }
        output_slot += 1;
    }
    if row_count > 1 {
        for r in 1..row_count {
            for i in 0..min(row_count - r, col_count) {
                ret[output_slot].push(input[r + i][col_count - i - 1]);
            }
            output_slot += 1;
        }
    }

    return ret;
}

#[cfg(test)]
mod traverse_diagonally_tr_bl_tests {
    use super::traverse_diagonally_tr_bl;
    #[test]
    fn wide() {
        let actual = traverse_diagonally_tr_bl::<i32>(&vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8]]);
        assert_eq!(actual[0][0], 1);
        assert_eq!(actual[1][0], 2);
        assert_eq!(actual[1][1], 5);
        assert_eq!(actual[2][0], 3);
        assert_eq!(actual[2][1], 6);
        assert_eq!(actual[3][0], 4);
        assert_eq!(actual[3][1], 7);
        assert_eq!(actual[4][0], 8);
    }
    #[test]
    fn empty() {
        let actual = traverse_diagonally_tr_bl::<i32>(&vec![]);
        assert!(actual.is_empty());
    }

    #[test]
    fn tall() {
        let actual =
            traverse_diagonally_tr_bl::<i32>(&vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]]);
        assert_eq!(actual[0][0], 1);
        assert_eq!(actual[1][0], 2);
        assert_eq!(actual[1][1], 3);
        assert_eq!(actual[2][0], 4);
        assert_eq!(actual[2][1], 5);
        assert_eq!(actual[3][0], 6);
        assert_eq!(actual[3][1], 7);
        assert_eq!(actual[4][0], 8);
    }
}
