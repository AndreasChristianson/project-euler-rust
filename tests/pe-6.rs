/*
<p>The sum of the squares of the first ten natural numbers is,</p>
$$1^2 + 2^2 + ... + 10^2 = 385$$
<p>The square of the sum of the first ten natural numbers is,</p>
$$(1 + 2 + ... + 10)^2 = 55^2 = 3025$$
<p>Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is $3025 - 385 = 2640$.</p>
<p>Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.</p>
*/

fn sum_of_squares(range: std::ops::Range<u128>) -> u128 {
    let mut sum = 0;
    for n in range {
        sum += n * n;
    }
    return sum;
}
fn square_of_sums(range: std::ops::Range<u128>) -> u128 {
    let mut sum = 0;
    for n in range {
        sum += n;
    }
    return sum * sum;
}

#[test]
fn sum_of_squares_test() {
    assert_eq!(sum_of_squares(0..11), 385);
}

#[test]
fn square_of_sums_test() {
    assert_eq!(square_of_sums(0..11), 3025);
}

#[test]
fn given() {
    assert_eq!(square_of_sums(0..11) - sum_of_squares(0..11), 2640);
}

#[test]
fn solution() {
    assert_eq!(square_of_sums(1..101) - sum_of_squares(1..101), 25164150);
}
