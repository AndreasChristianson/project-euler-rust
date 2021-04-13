extern crate num;
use num::Integer;

pub struct Hailstone {
    curr: u128,
}

impl Iterator for Hailstone {
    type Item = u128;

    fn next(&mut self) -> Option<u128> {
        if self.curr == 1 {
            return None;
        }
        if self.curr.is_even() {
            self.curr = self.curr / 2;
            return Some(self.curr);
        }
        self.curr = self.curr * 3 + 1;
        return Some(self.curr);
    }
}

pub fn hailstone(initial: u128) -> Hailstone {
    Hailstone { curr: initial }
}

#[cfg(test)]
mod hailstone_tests {
    use super::*;
    #[test]
    fn first_couple_fib() {
        let mut generator = hailstone(1);

        assert!(generator.next().is_none());
    }

    #[test]
    fn number_of_max_hailstone_13() {
        let generator = hailstone(13);

        let max = generator.max().unwrap_or_default();

        assert_eq!(40, max)
    }

    #[test]
    fn number_of_max_hailstone_27() {
        let generator = hailstone(27);

        let max = generator.max().unwrap_or_default();

        assert_eq!(9232, max)
    }

    #[test]
    fn steps_for_93571393692802302() {
        let generator = hailstone(93571393692802302);

        let count = generator.count();

        assert_eq!(2091, count)
    }
}
