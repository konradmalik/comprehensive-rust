pub fn luhn(cc_number: &str) -> bool {
    let mut digits = str_into_digits(cc_number);
    if digits.len() < 2 {
        return false;
    }

    for d in digits.iter_mut().rev().skip(1).step_by(2) {
        let mut doubled = (*d) * 2;
        let mut sum = 0;
        while doubled != 0 {
            sum += doubled % 10;
            doubled /= 10;
        }
        *d = sum;
    }

    let sum_of_all: u32 = digits.iter().sum();
    sum_of_all % 10 == 0
}

fn str_into_digits(s: &str) -> Vec<u32> {
    let mut digits = Vec::new();
    for c in s.chars() {
        match c.to_digit(10) {
            Some(d) => digits.push(d),
            None => (),
        }
    }
    digits
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

#[allow(dead_code)]
fn main() {}

