fn main() {
    println!("pe #1");
    let sum: i32 = (0..1000).filter(|n| n%3 == 0 || n%5 == 0).sum();
    println!("{}", sum);
}
