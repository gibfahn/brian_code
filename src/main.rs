use color_eyre::{eyre::Context, Result};
use std::io::Write;

pub mod bubble_sort;
pub mod further_maths;
pub mod harshad_numbers;
pub mod most_frequent_digit;
pub mod prime;

fn main() {
    loop {
        if let Err(e) = prompt_and_run() {
            println!("  Error: {e}");
        }
        println!();
    }
}

fn prompt_and_run() -> Result<()> {
    let mut s = String::new();
    print!("Input: ");
    let _ = std::io::stdout().flush();
    std::io::stdin()
        .read_line(&mut s)
        .wrap_err("Did not enter a correct string")?;
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    println!("  User input: {s}");
    println!("{}", further_maths::f(s.parse()?)?);
    Ok(())
}
