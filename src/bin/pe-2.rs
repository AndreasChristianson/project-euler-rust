fn main() {
    println!("pe #2");
    let mut sum_of_evens: i64 = 0;
    let mut sum: i64 = 0;
    let mut last: i64 = 0;
    let mut current: i64 = 1;

    while current < 4_000_000 {
        sum_of_evens += if current % 2 == 0 { current } else { 0 };
        sum += current;
        let next = last + current;
        last = current;
        current = next;
        println!("current: {0:>10}", current);
    }

    println!();
    println!("sum of evens: {0:>10}", sum_of_evens);
    println!("sum:          {0:>10}", sum);
}
