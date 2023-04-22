use color_eyre::{eyre::eyre, Result};

/// A bubble sort algorithm could be used to sort the list of numbers into ascending order.
/// Takes a list of numbers and sorts them in-place.
pub fn f(x: f64) -> Result<f64> {
    if x < 0.0 && x >= -1.0 {
        return Ok(-2.0 * x);
    } else if x >= 0.0 && x < 3.0 {
        return Ok(4.0 * x - (x * x));
    } else if x >= 3.0 && x <= 4.0 {
        return Ok(2.0 * x - 3.0);
    }
    return Err(eyre!("What the hell is this number"));
}

#[cfg(test)]
mod tests {
    use crate::further_maths::f;

    #[test]
    fn test_further_maths() {
        let output = f(0.0).unwrap();
        assert_eq!(0.0, output);
    }
}

/*
struct Brian {
    age: f64,
    number_of_legs: u8,
    name: String,
    gender: Gender,
}

impl Brian {
    fn new() -> Brian {
        return Brian {
            age: 14.0,
            number_of_legs: 2,
            name: "Briceps Fahnestock".to_string(),
            gender: Gender::Boy,
        };
    }
    fn say_hello(&self) {
        println!(
            "Hello {}, you are {} years old, you have gender {} and {} legs",
            self.name, self.age, self.gender, self.number_of_legs,
        );
    }
}

fn do_stuff() {
    let brian = Brian::new();
    brian.say_hello();
    brian.gender.likes()
}

#[derive(Debug, PartialEq)]
enum Gender {
    Boy,
    Girl,
    Other,
}

impl Gender {
    fn likes(&self) {
        if self == Gender::Boy {
            println!("I like Physics");
        } else if self == Gender::Girl {
            println!("I like Tiktok");
        } else if self == Gender::Other {
            println!("???");
        }
    }
}
*/
