// clippy2.rs
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a hint.

/*
Code description:
    - The code is checking if the option is Some and if it is, it adds the value to res
    - The code is using the if let syntax
 */
fn main() {
    let mut res = 42;
    let option = Some(12);

    if let Some(x) = option {
        res += x;
    }
    println!("{}", res);
}
