fn main() {
    let s = String::from("Hello, world!");
    let len = count_str_len(&s);
    println!("The length of '{}' is {}.", s, len);
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("{}",slice == &[2, 3]);
}
fn count_str_len(s: &String) -> usize {
    s.len()
}
