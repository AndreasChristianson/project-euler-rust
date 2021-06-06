extern crate num;

pub struct PrimeSequence<T: num::PrimInt> {
    primes: Vec<T>,
    n: T,           // last number we checked
    nearest_six: T, // closest value divisible by 6
    six: T,
    two: T,
    three: T,
}

impl<T: num::PrimInt> Iterator for PrimeSequence<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        fn is_prime<V: num::PrimInt>(primes: &Vec<V>, n: V) -> bool {
            for &prime in primes {
                if prime * prime > n {
                    // Should always be exit here on a prime
                    // Bertrand's postulate states that for every integer n there is a prime number between n and 2n
                    return true;
                }
                if n % prime == V::zero() {
                    return false;
                }
            }
            return true;
        }

        if self.n < self.two {
            self.n = self.two;
            return Some(self.two);
        }

        if self.n == self.two {
            self.n = self.three;
            return Some(self.n);
        }

        loop {
            if self.n < self.nearest_six {
                self.n = self.n + self.two;
            } else {
                self.nearest_six = self.nearest_six + self.six;
                self.n = self.nearest_six - T::one();
            }
            if is_prime(&self.primes, self.n) {
                self.primes.push(self.n);
                return Some(self.n);
            }
        }
    }
}

pub fn primes<T: num::PrimInt>() -> PrimeSequence<T> {
    PrimeSequence {
        primes: vec![],
        n: T::zero(),
        nearest_six: T::zero(),
        six: T::from(6).unwrap(),
        two: T::from(2).unwrap(),
        three: T::from(3).unwrap(),
    }
}

pub struct SexyPrimeSequence<T: num::PrimInt> {
    primes_sequence: PrimeSequence<T>,
    last: T,
}

pub fn sexy_primes<T: num::PrimInt>() -> SexyPrimeSequence<T> {
    SexyPrimeSequence {
        primes_sequence: primes(),
        last: T::zero(),
    }
}

impl<T: num::PrimInt> Iterator for SexyPrimeSequence<T> {
    type Item = (T, T);

    fn next(&mut self) -> Option<(T, T)> {
        loop {
            let current = self.primes_sequence.next().unwrap();
            let potential = (self.last, current);
            self.last = current;
            if potential.0 + self.primes_sequence.six == potential.1 {
                return Some(potential);
            }
        }
    }
}

pub struct SexyPrimeQuadrupletsSequence<T: num::PrimInt> {
    sexy_primes_sequence: SexyPrimeSequence<T>,
    first: (T, T),
    second: (T, T),
}

pub fn quadruplets_sexy_primes<T: num::PrimInt>() -> SexyPrimeQuadrupletsSequence<T> {
    SexyPrimeQuadrupletsSequence {
        sexy_primes_sequence: sexy_primes(),
        first: (T::zero(), T::zero()),
        second: (T::zero(), T::zero()),
    }
}

impl<T: num::PrimInt> Iterator for SexyPrimeQuadrupletsSequence<T> {
    type Item = (T, T, T, T);

    fn next(&mut self) -> Option<(T, T, T, T)> {
        loop {
            let current = self.sexy_primes_sequence.next().unwrap();
            let potential = (self.first, self.second, current);
            self.first = self.second;
            self.second = current;
            if potential.0 .1 == potential.1 .0 && potential.1 .1 == potential.2 .0 {
                return Some((
                    potential.0 .0,
                    potential.1 .0,
                    potential.1 .1,
                    potential.2 .1,
                ));
            }
        }
    }
}

#[cfg(test)]
mod sexy_prime_quadruplets_tests {
    use super::*;

    #[test]
    fn first_few() {
        let mut generator: SexyPrimeQuadrupletsSequence<u64> = quadruplets_sexy_primes();

        assert_eq!((251, 257, 263, 269), generator.next().unwrap());
        assert_eq!((1741, 1747, 1753, 1759), generator.next().unwrap());
        assert_eq!((3301, 3307, 3313, 3319), generator.next().unwrap());
        assert_eq!((5101, 5107, 5113, 5119), generator.next().unwrap());
        assert_eq!((5381, 5387, 5393, 5399), generator.next().unwrap());
    }
}

#[cfg(test)]
mod sexy_primes_tests {
    use super::*;

    #[test]
    fn first_few() {
        let mut generator: SexyPrimeSequence<u64> = sexy_primes();

        assert_eq!((23, 29), generator.next().unwrap());
        assert_eq!((31, 37), generator.next().unwrap());
        assert_eq!((47, 53), generator.next().unwrap());
        assert_eq!((53, 59), generator.next().unwrap());
    }

    #[test]
    fn greater_than_7900() {
        let generator: SexyPrimeSequence<u64> = sexy_primes();

        let next = generator.skip_while(|x| x.0 < 7900).next().unwrap();
        assert_eq!((7901, 7907), next);
    }

    #[test]
    fn count_less_than_100_000() {
        let generator: SexyPrimeSequence<u64> = sexy_primes();

        let count = generator.take_while(|x| x.0 < 100_000).count();
        assert_eq!(1940, count);
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
