extern crate project_euler_rust;
use project_euler_rust::text::number_as_text::number_to_spoken_text;
/*

<p>If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.</p>
<p>If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many letters would be used? </p>
<br /><p class="note"><b>NOTE:</b> Do not count spaces or hyphens. For example, 342 (three hundred and forty-two) contains 23 letters and 115 (one hundred and fifteen) contains 20 letters. The use of "and" when writing out numbers is in compliance with British usage.</p>

*/

fn count_chars(string: String) -> usize {
    let sanitized: Vec<char> = string.chars().filter(|&c| !r#" -"#.contains(c)).collect();
    return sanitized.len();
}

#[test]
fn solution() {
  let mut numbers = String::new();
  for n in 1..=1000 {
    numbers.push_str(&number_to_spoken_text(n));
  }
  assert_eq!(count_chars(numbers), 21124);
}

#[test]
fn given1() {
  let mut numbers = String::new();
  for n in 1..=5 {
    numbers.push_str(&number_to_spoken_text(n));
  }
  assert_eq!(count_chars(numbers), 19);

}

#[test]
fn given2() {
    let string = number_to_spoken_text(342);
    assert_eq!(count_chars(string), 23);
}

#[test]
fn given3() {
    let string = number_to_spoken_text(115);
    assert_eq!(count_chars(string), 20);
}
