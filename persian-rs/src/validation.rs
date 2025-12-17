//!This module provade two function for validating Iranian nasional id([crate::validation::is_valid_national_id])
//! and card number([crate::validation::is_valid_card_number])

///Checks if entered Iranian nasional id is valid
///
/// Returns `false` if the enterd string is not 10 digit long without spaces or chars
///
pub fn is_valid_national_id(code: &str) -> bool {
    if code.len() != 10 {
        return false;
    }

    let digits: Vec<u32> = match code.chars().map(|c| c.to_digit(10)).collect() {
        Some(v) => v,
        None => return false,
    };

    let control_digit = digits[9];
    let sum: u32 = digits
        .iter()
        .take(9)
        .enumerate()
        .map(|(index, &digit)| digit * (10 - index as u32))
        .sum();
    let remainder = sum % 11;

    if remainder < 2 {
        remainder == control_digit
    } else {
        (11 - remainder) == control_digit
    }
}
///Checks if the inputed Iranian card number is valid using Luhn algorithm
///
/// returns false if the entered string is not 8 to 19 digit long wihout spaces or chars
pub fn is_valid_card_number(number: &str) -> bool {
    if number.len() < 8 || number.len() > 19 || !number.chars().all(char::is_numeric) {
        return false;
    }

    let sum = number
        .chars()
        .rev()
        .filter_map(|c| c.to_digit(10))
        .enumerate()
        .map(|(i, digit)| {
            if i % 2 == 1 {
                let d = digit * 2;
                if d > 9 {
                    d - 9
                } else {
                    d
                }
            } else {
                digit
            }
        })
        .sum::<u32>();

    sum % 10 == 0
}
