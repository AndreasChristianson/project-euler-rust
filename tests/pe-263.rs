extern crate project_euler_rust;
use project_euler_rust::generators::primes::*;
use project_euler_rust::predicates::practical::is_practical;

/*

<p>
Consider the number 6. The divisors of 6 are: 1,2,3 and 6.<br />
Every number from 1 up to and including 6 can be written as a sum of distinct divisors of 6:<br />
1=1, 2=2, 3=1+2, 4=1+3, 5=2+3, 6=6.<br />
A number <var>n</var> is called a practical number if every number from 1 up to and including <var>n</var> can be expressed as a sum of distinct divisors of <var>n</var>.
</p>
<p>
A pair of consecutive prime numbers with a difference of six is called a sexy pair (since "sex" is the Latin word for "six"). The first sexy pair is (23, 29).
</p>
<p>
We may occasionally find a triple-pair, which means three consecutive sexy prime pairs, such that the second member of each pair is the first member of the next pair.
</p>
<p>
We shall call a number <var>n</var> such that :
</p><ul><li>(<var>n</var>-9, <var>n</var>-3), (<var>n</var>-3,<var>n</var>+3), (<var>n</var>+3, <var>n</var>+9) form a triple-pair, and
</li><li>the numbers <var>n</var>-8, <var>n</var>-4, <var>n</var>, <var>n</var>+4 and <var>n</var>+8 are all practical,
</li></ul>
an engineers’ paradise.

<p>
Find the sum of the first four engineers’ paradises.
</p>

*/

pub struct Paradise {
    quad_sexy_primes_sequence: SexyPrimeQuadrupletsSequence<u128>,
}

pub fn paradise_generator() -> Paradise {
    Paradise {
        quad_sexy_primes_sequence: quadruplets_sexy_primes(),
    }
}
impl Iterator for Paradise {
    type Item = u128;

    fn next(&mut self) -> Option<u128> {
        loop {
            let current = self.quad_sexy_primes_sequence.next().unwrap();
            println!("quad={:?}", current);

            let potential = current.0 + 1;
            let positions = vec![
                potential,
                potential + 4,
                potential + 8,
                potential + 12,
                potential + 16,
            ];

            let all_practical = positions
                .iter()
                .all(|potential_practical| is_practical(*potential_practical));
            if all_practical {
                let paradise = current.1 + 3;
                println!("paradise={:?}", paradise);

                return Some(paradise);
            }
        }
    }
}

#[test]
#[ignore]
fn solution() {
    let first_four: u128 = paradise_generator().take(4).sum(); // fourth is at 1146521020

    assert_eq!(2039506520, first_four);
}
