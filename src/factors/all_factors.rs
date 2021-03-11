// slow. need to use prime factors and combinatorics

pub fn factors<T: num::PrimInt>(target: T) -> Vec<T> {
    let mut n = T::from(2).unwrap();
    let mut ret = vec![];
    while n * n < target {
        if target % n == T::zero() {
            ret.push(n);
            ret.push(target / n);
            n = T::from(2).unwrap();
        }
        n = n + T::one();
    }
    return ret;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn factors_of_10() {
        assert_eq!(vec![2, 5], factors(10));
    }
    // #[test]
    fn factors_of_463_743_423_371() {
        assert_eq!(
            vec![7727, 7741, 7753, 59814707, 59907431, 60015973],
            factors(463_743_423_371u128)
        );
    }
}
