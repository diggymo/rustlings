// primitive_types6.rs
// Use a tuple index to access the second element of `numbers`.
// You can put this right into the `println!` where the ??? is.
// Execute `rustlings hint primitive_types6` for hints!

fn main() {
    let numbers = (1, 2, 3);
    let second = numbers.1;
    println!("The second number is {}", second);
}
