extern crate num;

pub struct PrimeSequence<T: num::PrimInt> {
    primes: Vec<T>,
    n: T,         // 6n±1
    bottom: bool, // true = 6n-1, false = 6n+1
}

impl<T: num::PrimInt> Iterator for PrimeSequence<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        fn inc<V: num::PrimInt>(seq: &mut PrimeSequence<V>) {
            if seq.bottom {
                seq.bottom = false;
            } else {
                seq.bottom = true;
                seq.n = seq.n + V::from(6).unwrap();
            }
        }

        if self.n == T::zero() && self.bottom {
            inc(self);

            return Some(T::from(2).unwrap());
        }
        if self.n == T::zero() && !self.bottom {
            inc(self);

            return Some(T::from(3).unwrap());
        }

        'next: loop {
            let potential;
            if self.bottom {
                potential = self.n - T::one();
            } else {
                potential = self.n + T::one();
            }
            inc(self);

            for &n in &self.primes {
                if n * n > potential {
                    self.primes.push(potential);

                    return Some(potential);
                }
                if potential % n == T::zero() {
                    continue 'next;
                }
            }
            self.primes.push(potential);

            return Some(potential);
        }
    }
}

pub fn primes<T: num::PrimInt>() -> PrimeSequence<T> {
    PrimeSequence {
        primes: vec![],
        n: T::zero(),
        bottom: true,
    }
}

#[cfg(test)]
mod primes_tests {
    use super::*;
    #[test]
    fn sum_of_first_primes_less_than_100() {
        let generator: PrimeSequence<u32> = primes();
        let sum: u32 = generator.take_while(|&n| n < 100).sum();

        assert_eq!(1060, sum)
    }

    #[test]
    fn sum_of_first_primes_less_than_10000() {
        let generator: PrimeSequence<u32> = primes();
        let sum: u32 = generator.take_while(|&n| n < 10000).sum();

        assert_eq!(5736396, sum)
    }

    #[test]
    fn sum_of_first_1000_primes() {
        let generator: PrimeSequence<u128> = primes();
        let sum: u128 = generator.take(1000).sum();

        assert_eq!(3682913, sum)
    }

    #[test]
    fn first_couple_primes() {
        let mut generator: PrimeSequence<u32> = primes();

        assert_eq!(2, generator.next().unwrap());
        assert_eq!(3, generator.next().unwrap());
        assert_eq!(5, generator.next().unwrap());
        assert_eq!(7, generator.next().unwrap());
        assert_eq!(11, generator.next().unwrap());
        assert_eq!(13, generator.next().unwrap());
        assert_eq!(17, generator.next().unwrap());
        assert_eq!(19, generator.next().unwrap());
        assert_eq!(23, generator.next().unwrap());
        assert_eq!(29, generator.next().unwrap());
        assert_eq!(31, generator.next().unwrap());
    }

    #[test]
    fn number_of_primes_less_than_1_000_000() {
        let generator: PrimeSequence<u32> = primes();
        let count = generator.take_while(|&n| n < 1_000_000).count();

        assert_eq!(78498, count)
    }
}
