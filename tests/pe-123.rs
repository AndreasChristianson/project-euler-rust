extern crate project_euler_rust;
use project_euler_rust::generators::primes::*;

/*

<p>Let <i>p</i><sub>n</sub> be the <i>n</i>th prime: 2, 3, 5, 7, 11, ..., and let <i>r</i> be the remainder when (<i>p</i><sub>n</sub>−1)<sup><i>n</i></sup> + (<i>p</i><sub>n</sub>+1)<sup><i>n</i></sup> is divided by <i>p</i><sub>n</sub><sup>2</sup>.</p>
<p>For example, when <i>n</i> = 3, <i>p</i><sub>3</sub> = 5, and 4<sup>3</sup> + 6<sup>3</sup> = 280 ≡ 5 mod 25.</p>
<p>The least value of <i>n</i> for which the remainder first exceeds 10<sup>9</sup> is 7037.</p>
<p>Find the least value of <i>n</i> for which the remainder first exceeds 10<sup>10</sup>.</p>

*/

/*
data:
index: 1, prime: 2, left: 1, right: 3, left_plus_right: 4, prime_squared: 4, remainder: 0
index: 2, prime: 3, left: 4, right: 16, left_plus_right: 20, prime_squared: 9, remainder: 2
index: 3, prime: 5, left: 64, right: 216, left_plus_right: 280, prime_squared: 25, remainder: 5
index: 4, prime: 7, left: 1296, right: 4096, left_plus_right: 5392, prime_squared: 49, remainder: 2
index: 5, prime: 11, left: 100000, right: 248832, left_plus_right: 348832, prime_squared: 121, remainder: 110
index: 6, prime: 13, left: 2985984, right: 7529536, left_plus_right: 10515520, prime_squared: 169, remainder: 2
index: 7, prime: 17, left: 268435456, right: 612220032, left_plus_right: 880655488, prime_squared: 289, remainder: 238
index: 8, prime: 19, left: 11019960576, right: 25600000000, left_plus_right: 36619960576, prime_squared: 361, remainder: 2
index: 9, prime: 23, left: 1207269217792, right: 2641807540224, left_plus_right: 3849076758016, prime_squared: 529, remainder: 414
index: 10, prime: 29, left: 296196766695424, right: 590490000000000, left_plus_right: 886686766695424, prime_squared: 841, remainder: 2
index: 11, prime: 31, left: 17714700000000000, right: 36028797018963968, left_plus_right: 53743497018963968, prime_squared: 961, remainder: 682
index: 12, prime: 37, left: 4738381338321616896, right: 9065737908494995456, left_plus_right: 13804119246816612352, prime_squared: 1369, remainder: 2
index: 13, prime: 41, left: 671088640000000000000, right: 1265437718438866624512, left_plus_right: 1936526358438866624512, prime_squared: 1681, remainder: 1066
index: 14, prime: 43, left: 53148384174432398229504, right: 101938319743841411792896, left_plus_right: 155086703918273810022400, prime_squared: 1849, remainder: 2
index: 15, prime: 47, left: 8737103395697172336050176, right: 16543163447903718821855232, left_plus_right: 25280266843600891157905408, prime_squared: 2209, remainder: 1410
index: 16, prime: 53, left: 2857942574656970690381479936, right: 5227573613485916806405226496, left_plus_right: 8085516188142887496786706432, prime_squared: 2809, remainder: 2
index: 17, prime: 59, left: 951208868148684143308060622848, right: 1692665944473600000000000000000, left_plus_right: 2643874812622284143308060622848, prime_squared: 3481, remainder: 2006
index: 18, prime: 61, left: 101559956668416000000000000000000, right: 183252712161029662582812243656704, left_plus_right: 284812668829445662582812243656704, prime_squared: 3721, remainder: 2
index: 19, prime: 67, left: 37267887454924297153448283417870336, right: 65715730729138450753894037357330432, left_plus_right: 102983618184062747907342320775200768, prime_squared: 4489, remainder: 2546
index: 20, prime: 71, left: 7979226629761200100000000000000000000, right: 14016833953562607293918185758734155776, left_plus_right: 21996060583323807393918185758734155776, prime_squared: 5041, remainder: 2


looks like when index is even the remainder is `2`
when the index is odd the remainder is `prime * index * 2`
🤷
*/
#[allow(dead_code)]
fn prime_remainder_greater_than(target: i128) -> usize {
    let generator: PrimeSequence<i128> = primes();

    for (raw_index, prime) in generator.enumerate() {
        let index = raw_index + 1;
        let left = (prime - 1i128).pow(index as u32);
        let right = (prime + 1i128).pow(index as u32);
        let left_plus_right = left + right;
        let prime_squared = prime.pow(2);
        let remainder = left_plus_right % prime_squared;
        println!("index: {}, prime: {}, left: {}, right: {}, left_plus_right: {}, prime_squared: {}, remainder: {}", index, prime, left, right, left_plus_right, prime_squared, remainder);
        if remainder > target {
            return index;
        }
    }
    panic!("found the end of primes");
}

fn fast_prime_remainder_greater_than(target: u128) -> usize {
    let generator: PrimeSequence<u128> = primes();

    for (raw_index, prime) in generator.enumerate() {
        let index = raw_index + 1;
        if index % 2 == 0 {
            continue;
        }
        let remainder = prime * index as u128 * 2;
        if remainder > target {
            return index;
        }
    }
    panic!("found the end of primes");
}

#[test]
fn solution() {
    assert_eq!(21035, fast_prime_remainder_greater_than(10_000_000_000));
}

#[test]
fn given() {
    assert_eq!(7037, fast_prime_remainder_greater_than(1_000_000_000));
}
