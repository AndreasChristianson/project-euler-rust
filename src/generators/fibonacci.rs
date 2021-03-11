extern crate num;

pub struct Fibonacci<T: num::PrimInt> {
    curr: T,
    next: T,
    n: u8,
}

impl<T: num::PrimInt> Iterator for Fibonacci<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.n == 0 {
            self.n += 1;
            return Some(T::zero());
        }
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;

        return Some(self.curr);
    }
}

pub fn fibonacci<T: num::PrimInt>() -> Fibonacci<T> {
    Fibonacci {
        curr: T::zero(),
        next: T::one(),
        n: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn first_couple_fib() {
        let mut generator = fibonacci();

        assert_eq!(0, generator.next().unwrap());
        assert_eq!(1, generator.next().unwrap());
        assert_eq!(1, generator.next().unwrap());
        assert_eq!(2, generator.next().unwrap());
        assert_eq!(3, generator.next().unwrap());
        assert_eq!(5, generator.next().unwrap());
        assert_eq!(8, generator.next().unwrap());
        assert_eq!(13, generator.next().unwrap());
        assert_eq!(21, generator.next().unwrap());
        assert_eq!(34, generator.next().unwrap());
        assert_eq!(55, generator.next().unwrap());
    }

    #[test]
    fn number_of_fibonacci_less_than_1_000_000() {
        let generator: Fibonacci<u64> = fibonacci();

        let count = generator.take_while(|&n| n < 1_000_000).count();

        assert_eq!(31, count)
    }

    #[test]
    fn sum_of_first_1_000_fib() {
        // F(1)+...+F(n)=F(n+2)−1
        let generator: Fibonacci<u128> = fibonacci();

        let count: u128 = generator.take(100).sum();

        assert_eq!(573_147_844_013_817_084_101u128 - 1, count)
    }
}
