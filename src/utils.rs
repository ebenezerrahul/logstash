pub fn exponent(mut base: u64, mut exponent: u64) -> u64 {
    let mut result = 1;
    if exponent == 0 {
        return 1;
    }
    while exponent != 0 {
        if exponent & 1 != 0 {
            result *= base;
        }
        base *= base;

        exponent = exponent >> 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_expoent() {
        assert_eq!(32, exponent(2, 5));
        assert_eq!(81, exponent(3, 4));
    }
}
