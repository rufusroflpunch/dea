use rand::{thread_rng, Rng};

pub fn dea() -> String {
    let digits = (0..6u32)
        .map(|_| {
            let mut rng = thread_rng();
            rng.gen_range(0..10u32)
        })
        .collect::<Vec<_>>();

    let final_digit = check_digit(&digits);

    let mut dea_num = String::with_capacity(9);

    let mut rng = thread_rng();
    dea_num.push(rng.gen_range('A'..='Z'));
    dea_num.push(rng.gen_range('A'..='Z'));
    for digit in digits {
        let d = char::from_digit(digit, 10).unwrap();
        dea_num.push(d);
    }

    let final_digit = char::from_digit(final_digit, 10).unwrap();
    dea_num.push(final_digit);

    dea_num
}

pub fn check(dea: &str) -> bool {
    if dea.len() != 9 {
        return false;
    }

    let dea = dea.to_ascii_uppercase();

    let mut chars = dea.chars();

    if !valid_letter(chars.next().unwrap()) {
        return false;
    }

    let ln_digit = chars.next().unwrap();
    if !valid_letter(ln_digit) && ln_digit != '9' {
        return false;
    }

    for digit in chars {
        if !valid_digit(digit) {
            return false;
        }
    }

    let digits = dea[2..8]
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    let final_digit = check_digit(&digits);

    if char::from_digit(final_digit, 10).unwrap() != dea.chars().last().unwrap() {
        return false;
    }

    return true;
}

fn valid_letter(ch: char) -> bool {
    ch.is_ascii_alphabetic()
}

fn valid_digit(ch: char) -> bool {
    ch.is_ascii_digit()
}

fn check_digit(digits: &[u32]) -> u32 {
    let sum1 = digits[0] + digits[2] + digits[4];
    let sum2 = 2 * (digits[1] + digits[3] + digits[5]);
    (sum1 + sum2) % 10
}

#[cfg(test)]
mod tests {
    use crate::{check, dea};

    #[test]
    fn fake_dea() {
        let fdea = dea();
        assert!(check(&fdea));
    }

    #[test]
    fn check_dea() {
        assert!(check("TI6628462")); // Normal DEA
        assert!(check("T96628462")); // With 9 for second digit (means business)
        assert!(!check("T96628463")); // Check digit is wrong
        assert!(!check("196628462")); // First digit is not a letter
    }
}
