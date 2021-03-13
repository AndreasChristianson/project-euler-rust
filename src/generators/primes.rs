extern crate num;

pub struct PrimeSequence<T: num::PrimInt> {
    primes: Vec<T>,
    n: T,           // last number we checked
    nearest_six: T, // closest value divisible by 6
}

impl<T: num::PrimInt> Iterator for PrimeSequence<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        fn inc<V: num::PrimInt>(seq: &mut PrimeSequence<V>) {
            if seq.n < seq.nearest_six {
                seq.n = seq.n + V::from(2).unwrap();
            } else {
                seq.nearest_six = seq.nearest_six + V::from(6).unwrap();
                seq.n = seq.nearest_six - V::one();
            }
        }
        fn is_prime<V: num::PrimInt>(primes: &Vec<V>, n: V) -> bool {
            for &prime in primes {
                if prime * prime > n {                      /* Should always be exit here on a prime */
                    return true                             /* Bertrand's postulate states that for every integer n there is a prime number between n and 2n, since all primes are at least 2, if we are dealing with a prime number the last prime squared will be greater than the next prime */
                }
                if n % prime == V::zero() {
                    return false
                }
            }
            true                                           // should be mathematically impossible to get here, but rust still wants something
        }

        let two = T::from(2).unwrap();
        if self.n < two {
            self.n = two;
            self.primes.push(two);
            return Some(two);
        }

        if self.n == two {
            let three = T::from(3).unwrap();
            self.n = three;
            self.primes.push(three);
            return Some(three);
        }

        loop {
            inc(self);
            if is_prime(&self.primes, self.n) {
                self.primes.push(self.n);
                return Some(self.n)
            }
        }
    }
}

pub fn primes<T: num::PrimInt>() -> PrimeSequence<T> {
    PrimeSequence {
        primes: vec![],
        n: T::zero(),
        nearest_six: T::zero()
    }
}

#[cfg(test)]
mod primes_tests {
    use super::*;
    #[test]
    fn sum_of_first_primes_less_than_100() {
        let generator: PrimeSequence<u32> = primes();
        let generator2: PrimeSequence<u32> = primes();
        let sum: u32 = generator2.take_while(|&n| n < 100).sum();

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
