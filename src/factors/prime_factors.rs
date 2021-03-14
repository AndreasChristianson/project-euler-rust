use super::super::generators::primes::*;

pub fn prime_factors<T: num::PrimInt>(target: T) -> Vec<T> {
    let mut ret: Vec<T> = vec![];
    let mut adjusted_target: T = target;
    let mut generator: PrimeSequence<T> = primes();

    loop {
        let prime = generator.next().unwrap();
        if prime * prime > adjusted_target {
            break;
        }
        if adjusted_target % prime == T::zero() {
            ret.push(prime);
            adjusted_target = adjusted_target / prime;
            generator = primes()
        }
    }
    ret.push(adjusted_target);

    return ret;
}

#[cfg(test)]
mod prime_factors_tests {
    use super::*;
    #[test]
    fn prime_factors_of_10() {
        assert_eq!(vec![2, 5], prime_factors(10));
    }
    #[test]
    fn prime_factors_of_1_000_000() {
        assert_eq!(
            vec![2, 2, 2, 2, 2, 2, 5, 5, 5, 5, 5, 5],
            prime_factors(1_000_000)
        );
    }
    #[test]
    fn prime_factors_of_171_158_644_061_440_576_710_668_800_200_900() {
        assert_eq!(
            vec![
                2, 2, 3, 3, 5, 5, 7, 7, 11, 11, 13, 13, 17, 17, 19, 19, 23, 23, 29, 29, 31, 31, 37,
                37, 41, 41, 43, 43
            ],
            prime_factors(171_158_644_061_440_576_710_668_800_200_900u128)
        );
    }
    #[test]
    fn prime_factors_of_1_234_567() {
        assert_eq!(vec![127, 9721], prime_factors(1_234_567));
    }
    #[test]
    fn prime_factors_of_103_604_144_578_327_252_671_759_911_115_679_211() {
        let product: u128 = 103_604_144_578_327_252_671_759_911_115_679_211;
        assert_eq!(
            vec![7727, 7741, 7753, 7757, 7759, 7789, 7793, 7817, 7823],
            prime_factors(product)
        );
    }
    #[test]
    fn prime_factors_of_primes() {
        assert_eq!(vec![7727], prime_factors(7727));
        assert_eq!(vec![7741], prime_factors(7741));
        assert_eq!(vec![7753], prime_factors(7753));
    }
}
