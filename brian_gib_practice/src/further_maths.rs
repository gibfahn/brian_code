use color_eyre::{eyre::eyre, Result};

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

        let output = f(-0.5).unwrap();
        assert_eq!(1.0, output);

        let output = f(3.0).unwrap();
        assert_eq!(3.0, output);
    }
}