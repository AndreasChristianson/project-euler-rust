extern crate project_euler_rust;
use project_euler_rust::generators::hailstone::hailstone;

/*
<p>The following iterative sequence is defined for the set of positive integers:</p>
<p class="margin_left"><var>n</var> → <var>n</var>/2 (<var>n</var> is even)<br /><var>n</var> → 3<var>n</var> + 1 (<var>n</var> is odd)</p>
<p>Using the rule above and starting with 13, we generate the following sequence:</p>
<div class="center">13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1</div>
<p>It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms. Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.</p>
<p>Which starting number, under one million, produces the longest chain?</p>
<p class="note"><b>NOTE:</b> Once the chain starts the terms are allowed to go above one million.</p>

*/

#[test]
fn solution() {
    let mut max_start = 1;
    let mut max_count = 0;
    for c in 1..1_000_000 {
        let count = hailstone(c).count();
        if count > max_count {
            max_count = count;
            max_start = c;
        }
    }
    assert_eq!(max_count, 524);
    assert_eq!(max_start, 837799);
}
