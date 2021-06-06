use super::super::generators::primes::*;

pub fn is_prime<T: num::PrimInt>(number: T) -> bool {
    let zero = T::from(0).unwrap();
    let two = T::from(2).unwrap();
    if number < two {
        return false;
    }
    for p in primes() {
        if p * p > number {
            return true;
        }
        if number % p == zero {
            return false;
        }
    }

    panic!();
}

#[cfg(test)]
mod is_prime_tests {
    use super::is_prime;
    #[test]
    fn small_primes() {
        for n in vec![
            5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
            97,
        ] {
            assert!(is_prime(n));
        }
    }

    #[test]
    fn large_primes() {
        for n in vec![
            7919u128,
            27644437,     //bell
            319993,       //circular
            433494437,    //fib
            6643838879,   //lucas
            200560490131, //primordial
            32212254719,  //woodall
            65537,        //fermat
        ] {
            assert!(is_prime(n));
        }
    }

    #[test]
    #[ignore]
    fn large_slow_primes() {
        for n in vec![
            2305843009213693951u128, //mersenne
            304250263527209,         //primordial
            909090909090909091,      //unique
            1111111111111111111,     //unique
        ] {
            assert!(is_prime(n));
        }
    }

    #[test]
    fn small_composite() {
        for n in vec![5 * 7, 5 * 7 + 11, 23 * 29, 6, 29 * 31, 41 * 43 * 47] {
            assert_eq!(is_prime(n), false);
        }
    }

    #[test]
    fn large_composite() {
        for n in vec![
            10_000_000u128,
            7841 * 7853 * 7867 * 7873 * 7877,
            7879 * 7883 * 7901 * 7907 * 7919,
            8191 * 131071,
            55987 * 19531,
        ] {
            assert_eq!(is_prime(n), false);
        }
    }
    #[test]
    fn negative() {
        for n in vec![-1, -11, -111, -1234, -1001] {
            assert_eq!(is_prime(n), false);
        }
    }
    #[test]
    fn zero() {
        assert_eq!(is_prime(0), false);
    }
    #[test]
    fn one() {
        assert_eq!(is_prime(1), false);
    }
    #[test]
    fn two() {
        assert!(is_prime(2));
    }
    #[test]
    fn thee() {
        assert!(is_prime(3));
    }
}
