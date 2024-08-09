fn fib(n: u32) -> u32 {
    if n < 2 {
        // The base case.
        // todo!("Implement this");
        return n;
    } else {
        // The recursive case.
        // todo!("Implement this");
        return fib(n - 1) + n;
    }
}

fn main() {
    let n = 5;
    println!("fib({n}) = {}", fib(n));
}