use rayon::prelude::*;
use std::collections::BTreeMap;

pub fn count_passwords() -> usize {
    _count_passwords(super::INPUT)
}

fn _count_passwords(input: &str) -> usize {
    let mut split = input.split('-');
    let first: usize = split.next().unwrap().parse().unwrap();
    let second: usize = split.next().unwrap().parse().unwrap();

    (first..=second)
        .into_par_iter()
        .map(|password| password.to_string())
        .filter(|password| is_password(password))
        .count()
}

pub fn is_password(password: &str) -> bool {
    let chars: Vec<_> = password.chars().collect();

    let has_right_length = password_has_right_length(&chars);
    let has_double = password_has_double(&chars);
    let is_monotonic = password_is_monotonic(&chars);

    [has_right_length, has_double, is_monotonic]
        .iter()
        .all(|r| *r)
}

pub fn password_has_right_length(chars: &[char]) -> bool {
    chars.len() == 6
}

pub fn password_has_double(chars: &[char]) -> bool {
    let mut count = BTreeMap::default();
    for c in chars {
        *count.entry(c).or_insert(0) += 1;
    }

    count
        .iter()
        .max_by(|(_, x), (_, y)| x.cmp(y))
        .map(|(_, count)| *count >= 2)
        .unwrap_or_default()
}

pub fn password_is_monotonic(chars: &[char]) -> bool {
    chars
        .iter()
        .filter_map(|c| c.to_digit(10))
        .fold((true, 0), |acc, x| (acc.0 && (acc.1 <= x), x))
        .0
}

#[cfg(test)]
mod tests {
    extern crate test;

    use test::Bencher;

    #[test]
    fn is_password_valid1() {
        assert!(super::is_password("111111"));
    }

    #[test]
    fn is_password_valid2() {
        assert!(super::is_password("112345"));
    }

    #[test]
    fn is_password_valid3() {
        assert!(super::is_password("888889"));
    }

    #[test]
    fn is_password_invalid_wrong_length() {
        assert!(!super::is_password("1"));
    }

    #[test]
    fn is_password_invalid_not_monotonic() {
        assert!(!super::is_password("223450"));
    }

    #[test]
    fn is_password_invalid_no_double() {
        assert!(!super::is_password("123789"));
    }

    #[bench]
    fn bench_count_passwords_single(b: &mut Bencher) {
        let input = test::black_box("0-1");

        b.iter(|| super::_count_passwords(input))
    }

    #[bench]
    fn bench_count_passwords_lots(b: &mut Bencher) {
        let input = test::black_box("0-10000");

        b.iter(|| super::_count_passwords(input))
    }
}