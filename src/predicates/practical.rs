use super::super::factors::all_factors::factors;
use super::super::factors::prime_factors::prime_factors;
use std::collections::HashSet;
// https://en.wikipedia.org/wiki/Practical_number#Characterization_of_practical_numbers
// if for each prime factor p, the sum of the factors of the product (σ) of the prime factors less than p plus one, is smaller than p, then the number is practical

/* example A: 429606 is practical
   prime factors: 2^1, 3^2,  29^1,  823^1
   starting with the second prime factor, 3:
   3 <= σ(2) + 1
   σ(2) = (the sum of the factors of 2) = 2 + 1 = 3
   3 <= 3 + 1 ✅

   now the third prime factor, 29:
   29 <= σ(2 * 3^2) + 1
   σ(18) = (the sum of the factors of 18) = 1 + 2 + 3 + 6 + 9 + 18 = 39
   29 <= 39 + 1 ✅

   now the fourth prime factor, 823:
   823 <= σ(2 * 3^2 * 29) + 1
   σ(522) = (the sum of the factors of 522) = 1 + 2 + 3 + 6 + 9 + 18 + 29 + 58 + 87 + 174 + 261 + 522 = 1170
   823 <= 1170 + 1 ✅

   example B: 246 is not practical
   prime factors: 2, 3, 41
   starting with the second prime factor, 3:
   3 <= σ(2) + 1
   σ(2) = (the sum of the factors of 2) = 2 + 1 = 3
   3 <= 3 + 1 ✅

   now the third prime factor, 41:
   41 <= σ(2 * 3) + 1
   σ(6) = (the sum of the factors of 6) = 1 + 2 + 3 + 6 = 12
   41 <= 12 + 1 ❌

*/

pub fn is_practical(n: u128) -> bool {
    // println!("N={:?}", n);

    if n == 1 || n == 2 {
        return true;
    }

    if n % 4 != 0 && n % 6 != 0 {
        return false;
    }
    let prime_factors = prime_factors(n);

    let mut checked: HashSet<u128> = HashSet::new();

    for (index, prime_factor) in prime_factors.iter().enumerate() {
        if checked.contains(prime_factor) {
            continue;
        }
        // println!("{}: {:?}", index, prime_factor);
        let product: u128 = prime_factors[0..index].iter().product();
        // println!("product = {:?}", product);
        let factors_for_sigma = factors(product);
        // println!("factors_for_sigma = {:?}", factors_for_sigma);
        let sigma: u128 = factors_for_sigma.iter().sum();
        // println!("sigma = {:?}", sigma);
        if *prime_factor > sigma + 1 {
            return false;
        }
        checked.insert(*prime_factor);
    }

    return true;
}

#[cfg(test)]
mod is_practical_tests {
    use super::*;

    #[test]
    fn first_few() {
        for n in vec![
            1u128, 2, 4, 6, 8, 12, 16, 18, 20, 24, 28, 30, 32, 36, 40, 42, 48, 54, 56, 60, 64, 66,
            72, 78, 80, 84, 88, 90, 96, 100, 104, 108, 112, 120, 126, 128, 132, 140, 144, 150, 156,
            160, 162, 168, 176, 180, 192, 196, 198, 200, 204, 208, 210, 216, 220, 224, 228, 234,
            240, 252,
        ] {
            assert!(is_practical(n));
        }
    }

    #[test]
    fn not_practical() {
        for n in vec![7919u128, 27644437, 319993, 433494437, 6643838879] {
            assert!(!is_practical(n));
        }
    }

    #[test]
    fn perfect_numbers() {
        for n in vec![
            6u128,
            28,
            496,
            8128,
            33550336,
            8589869056,
            137438691328,
            // 2305843008139952128
        ] {
            assert!(is_practical(n));
        }
    }
}
