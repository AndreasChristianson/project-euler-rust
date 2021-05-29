extern crate project_euler_rust;
use project_euler_rust::combinatorics::count_permutations::count_ways_to_pick;
/*

<p>Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down, there are exactly 6 routes to the bottom right corner.</p>
<div class="center">
<img src="project/images/p015.png" class="dark_img" alt="" /></div>
<p>How many such routes are there through a 20×20 grid?</p>

*/

/*
  after calculating 0x0 = 1, 1x1 = 2, 2x2 = 6, 3x3 = 20, 4x4 = 70
  found https://oeis.org/A000984
  count_routes(n, n) = C(2n, n)
*/

fn count_routes(d: u128) -> u128 {
  return count_ways_to_pick(d, d * 2);
}

#[test]
fn solution() {
  let routes = count_routes(20);

  assert_eq!(routes, 137846528820);
}

#[test]
fn given() {
    let routes = count_routes(2);

    assert_eq!(routes, 6);
}

#[test]
fn extrapolate1x1() {
    let routes = count_routes(1);

    assert_eq!(routes, 2);
}

#[test]
fn extrapolate3x3() {
    let routes = count_routes(3);

    assert_eq!(routes, 6*2+2*4);
    assert_eq!(routes, 20);
}

#[test]
fn extrapolate4x4() {
    let routes = count_routes(4);

    assert_eq!(routes, 2*(6*2+2*4)+2*(5+4+6));
}