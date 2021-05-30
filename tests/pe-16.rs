extern crate project_euler_rust;
use project_euler_rust::strings::string_to_integer_sequence::to_int_sequence;
use num::BigUint;
/*

<p>2<sup>15</sup> = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.</p>
<p>What is the sum of the digits of the number 2<sup>1000</sup>?</p>

*/

#[test]
fn solution() {
  let value = BigUint::new(vec!(2)).pow(1000);
  let as_sequence = to_int_sequence(value.to_string());
  let mut sum = 0;

  for digit in as_sequence {
      sum += digit;
  }

  assert_eq!(1366, sum);
}

#[test]
fn given() {
  let value = BigUint::new(vec!(2)).pow(15);
  let as_sequence = to_int_sequence(value.to_string());
  let mut sum = 0;

  for digit in as_sequence {
      sum += digit;
  }

  assert_eq!(26, sum);
}
