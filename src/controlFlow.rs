/// Determine the length of the collatz sequence beginning at `n`.
fn collatz_length(mut n: i32) -> u32 {
    let mut len: u32 = 1;
    
    while n != 1 {
        if n % 2 == 0 {
            n = n / 2;
        } else {
            n = 3 * n + 1;
        }
        len += 1;
    }
    len
}

fn test_collatz_length() {
    assert_eq!(collatz_length(11), 15);
    println!("collatz_length tests pass");
}
  
fn main() {
    let n = 5;
    test_collatz_length();
    println!("The length of {n} is {}", collatz_length(n));
}