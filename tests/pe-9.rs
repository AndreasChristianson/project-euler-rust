extern crate project_euler_rust;

/*
<p>A Pythagorean triplet is a set of three natural numbers, <var>a</var> &lt; <var>b</var> &lt; <var>c</var>, for which,</p>
<div class="center"> <var>a</var><sup>2</sup> + <var>b</var><sup>2</sup> = <var>c</var><sup>2</sup></div>
<p>For example, 3<sup>2</sup> + 4<sup>2</sup> = 9 + 16 = 25 = 5<sup>2</sup>.</p>
<p>There exists exactly one Pythagorean triplet for which <var>a</var> + <var>b</var> + <var>c</var> = 1000.<br />Find the product <var>abc</var>.</p>
*/
#[derive(Debug, PartialEq)]
struct Triplet<T> {
    a: T,
    b: T,
    c: T,
}

fn find_triplet_with_sum(sum: usize) -> Option<Triplet<usize>> {
    for a in 1..sum {
        for b in a..sum {
            let c = sum - a - b;
            let a2 = a * a;
            let b2 = b * b;
            let c2 = c * c;
            if a2 + b2 > c2 {
                break;
            }
            if a2 + b2 == c2 {
                return Some(Triplet { a, b, c });
            }
        }
    }
    return None;
}

#[test]
fn given() {
    assert_eq!(
        find_triplet_with_sum(3 + 4 + 5).unwrap(),
        Triplet { a: 3, b: 4, c: 5 }
    );
}

#[test]
fn solution() {
    assert_eq!(
        find_triplet_with_sum(1000).unwrap(),
        Triplet {
            a: 200,
            b: 375,
            c: 425
        }
    );

    assert_eq!(200 * 375 * 425, 31875000);
}
