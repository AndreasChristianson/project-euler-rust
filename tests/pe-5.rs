extern crate project_euler_rust;

use project_euler_rust::factors::prime_factors::prime_factors;
use std::collections::HashMap;
/*
2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.

What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
*/

fn smallest_divisible_by_range(range: std::ops::Range<u64>) -> u64 {
    let mut shared_prime_factors: HashMap<u64, u8> = HashMap::new();

    for n in range {
        let factors = prime_factors(n);
        let mut prime_map: HashMap<u64, u8> = HashMap::new();
        for prime in factors {
            prime_map
                .entry(prime)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        for (key, value) in prime_map {
            let current_value = shared_prime_factors.entry(key).or_insert(0);
            if *current_value < value {
                *current_value = value;
            }
        }
    }
    let product = shared_prime_factors
        .iter()
        .fold(1, |mut accumulator, (prime, count)| {
            for _ in 0..*count {
                accumulator *= prime;
            }

            return accumulator;
        });
    return product;
}

#[test]
fn pe_5_given() {
    assert_eq!(smallest_divisible_by_range(2..10), 2520);
}

#[test]
fn pe_5_solution() {
    assert_eq!(smallest_divisible_by_range(2..20), 232792560);
}
