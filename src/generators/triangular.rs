extern crate num;

pub struct Triangular<T: num::PrimInt> {
    n: T,
    sum: T,
}

impl<T: num::PrimInt> Iterator for Triangular<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.n = self.n + T::one();
        self.sum = self.sum + self.n;

        return Some(self.sum);
    }
}

pub fn triangular<T: num::PrimInt>() -> Triangular<T> {
    Triangular {
        n: T::zero(),
        sum: T::zero(),
    }
}
pub fn triangular_at_n(n: u128) -> u128 {
    n * (n + 1) / (2)
}

#[cfg(test)]
mod triangular_tests {
    use super::*;
    #[test]
    fn initial() {
        let mut generator = triangular();

        assert_eq!(1, generator.next().unwrap());
        assert_eq!(3, generator.next().unwrap());
        assert_eq!(6, generator.next().unwrap());
        assert_eq!(10, generator.next().unwrap());
        assert_eq!(15, generator.next().unwrap());
        assert_eq!(21, generator.next().unwrap());
        assert_eq!(28, generator.next().unwrap());
        assert_eq!(36, generator.next().unwrap());
        assert_eq!(45, generator.next().unwrap());
    }

    #[test]
    fn large() {
        assert_eq!(1_631_432_881, triangular_at_n(57_121));
        assert_eq!(372_759_573_255_306, triangular_at_n(27_304_196));
        assert_eq!(5_000_050_000, triangular_at_n(100_000));
        assert_eq!(762_078_938_126_809_995, triangular_at_n(1_234_567_890));
    }
}
