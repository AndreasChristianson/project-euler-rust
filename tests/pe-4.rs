extern crate num;
extern crate project_euler_rust;
use project_euler_rust::strings::palindromes::is_palindrome;

/*
A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.

Find the largest palindrome made from the product of two 3-digit numbers.
*/

fn largest_product_palindrome(max: i32) -> i32 {
    let mut largest = 0;
    for i in (0..max + 1).rev() {
        if i * i < largest {
            break;
        }
        for j in (0..i + 1).rev() {
            let product = i * j;
            if product < largest {
                break;
            }
            if is_palindrome(&product.to_string()) {
                largest = product;
            }
        }
    }
    return largest;
}

#[test]
fn pe_4_given() {
    assert_eq!(largest_product_palindrome(99), 9009);
}

#[test]
fn pe_4_solution() {
    assert_eq!(largest_product_palindrome(999), 906609);
}
