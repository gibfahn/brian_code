fn main() {
    println!("Hi");
}

pub fn is_prime(maybe_prime: u64) -> bool {
    if maybe_prime == 1 {
        return false;
    }
    for x in 2..maybe_prime {
        let remainder = maybe_prime % x;
        if remainder == 0 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::is_prime;

    const EXPECTED_PRIMES: [u64; 25] = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97,
    ];

    #[test]
    fn test_is_prime() {
        let mut actual_primes = Vec::new();
        for n in 1..=100 {
            if is_prime(n) {
                actual_primes.push(n);
            }
        }
        assert_eq!(EXPECTED_PRIMES.as_ref(), actual_primes);
    }
}
