use rayon::prelude::*;
use std::collections::BTreeMap;

pub fn count_passwords() -> usize {
    let mut split = super::INPUT.split('-');
    let from: usize = split.next().unwrap().parse().unwrap();
    let to: usize = split.next().unwrap().parse().unwrap();

    _count_passwords(from, to)
}

fn _count_passwords(from: usize, to: usize) -> usize {
    (from..=to)
        .into_par_iter()
        .filter(|password| is_password(*password))
        .count()
}

pub fn is_password(password: usize) -> bool {
    let digits = digits_reverse(password).collect::<Vec<_>>();

    if !password_has_right_length(&digits) {
        return false;
    }

    if !password_has_double(&digits) {
        return false;
    }

    if !password_is_monotonic(&digits) {
        return false;
    }

    true
}

pub fn password_has_right_length(digits: &[u8]) -> bool {
    digits.len() == 6
}

pub fn password_has_double(digits: &[u8]) -> bool {
    let mut count = BTreeMap::default();
    for d in digits {
        *count.entry(d).or_insert(0) += 1;
    }

    count
        .iter()
        .max_by(|(_, x), (_, y)| x.cmp(y))
        .map(|(_, count)| *count >= 2)
        .unwrap_or_default()
}

pub fn password_is_monotonic(digits: &[u8]) -> bool {
    digits
        .iter()
        .rev()
        .fold((true, 0), |acc, x| (acc.0 && (acc.1 <= *x), *x))
        .0
}

pub fn digits_reverse(mut password: usize) -> impl Iterator<Item = u8> {
    std::iter::from_fn(move || match password {
        0 => None,
        _ => {
            let digit = password % 10;
            password /= 10;
            Some(digit as _)
        }
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_password_valid1() {
        assert!(super::is_password(111111));
    }

    #[test]
    fn is_password_valid2() {
        assert!(super::is_password(112345));
    }

    #[test]
    fn is_password_valid3() {
        assert!(super::is_password(888889));
    }

    #[test]
    fn is_password_invalid_wrong_length() {
        assert!(!super::is_password(1));
    }

    #[test]
    fn is_password_invalid_not_monotonic() {
        assert!(!super::is_password(223450));
    }

    #[test]
    fn is_password_invalid_no_double() {
        assert!(!super::is_password(123789));
    }

    #[test]
    fn digits() {
        assert_eq!(
            super::digits_reverse(123456).collect::<Vec<_>>(),
            [6, 5, 4, 3, 2, 1]
        );
    }
}

#[cfg(test)]
mod benchs {
    extern crate test;

    use test::Bencher;

    #[bench]
    fn digits(b: &mut Bencher) {
        let input = test::black_box(12345);

        b.iter(|| super::digits_reverse(input))
    }

    #[bench]
    fn is_password(b: &mut Bencher) {
        let input = test::black_box(12345);

        b.iter(|| super::is_password(input))
    }

    #[bench]
    fn count_passwords_single(b: &mut Bencher) {
        let from = test::black_box(0);
        let to = test::black_box(1);

        b.iter(|| super::_count_passwords(from, to))
    }

    #[bench]
    fn count_passwords_lots(b: &mut Bencher) {
        let from = test::black_box(0);
        let to = test::black_box(10000);

        b.iter(|| super::_count_passwords(from, to))
    }

    #[bench]
    fn count_passwords_input(b: &mut Bencher) {
        let input = test::black_box(crate::day_04::INPUT);

        let mut split = input.split('-');
        let from: usize = split.next().unwrap().parse().unwrap();
        let to: usize = split.next().unwrap().parse().unwrap();

        b.iter(|| super::_count_passwords(from, to))
    }
}
