use num_bigint::{BigInt, BigUint};
use num_traits::One;
use std::io::{self, Write};

fn format_number(n: &impl ToString) -> String {
    let num_str = n.to_string();
    let mut result = String::new();
    let mut count = 0;

    for c in num_str.chars().rev() {
        if count == 3 {
            result.push(',');
            count = 0;
        }
        result.push(c);
        count += 1;
    }

    result.chars().rev().collect()
}

fn main() {
    let mut input = String::new();
    print!("Enter the number of bits: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();
    let bits: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter an integer.");
            return;
        }
    };

    if bits <= 0 {
        println!("Invalid bit size. Must be greater than 0.");
        return;
    }

    let max_value = BigUint::one() << bits;
    let max_value = max_value - BigUint::one();

    let signed_max = BigUint::one() << (bits - 1);
    let signed_max = signed_max - BigUint::one();

    let signed_min = BigInt::one() << (bits - 1);
    let signed_min = -signed_min;

    println!(
        "Max value for a {}-bit unsigned integer: {} ({})",
        bits,
        max_value,
        format_number(&max_value)
    );
    println!(
        "Signed range for a {}-bit integer: {} to {} ({} to {})",
        bits,
        signed_min,
        signed_max,
        format_number(&signed_min),
        format_number(&signed_max)
    );
}
