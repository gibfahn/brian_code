use color_eyre::{eyre::Context, Result};
use std::io::Write;

pub mod bubble_sort;
pub mod further_maths;
pub mod harshad_numbers;
pub mod most_frequent_digit;
pub mod prime;

fn main() {
    let x = "Brian";
    let y = true;

    if !y {
        println!("Hi {x}");
    } else if false {
        println!("Bye {x}");
    } else {
        println!("Pie {x}");
    }

    let mut z = 0;
    loop {
        z += 1;
        print!("{z}Yo");
        if z == 100 {
            break;
        }
    }
    println!();

    let mut z = 0;
    while z <= 100 {
        print!("{z}Wo");
        z += 1;
    }
    println!();

    for z in 1..=100 {
        print!("{z}To");
    }
    println!();
}

// loop {
//     if let Err(e) = prompt_and_run() {
//         println!("  Error: {e}");
//     }
//     println!();
// }
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
