use rayon::prelude::*;
use std::collections::BTreeMap;

use crate::day_04::part_1::{
    digits_reverse,
    password_has_right_length,
    password_is_monotonic,
};

pub fn count_passwords() -> usize {
    _count_passwords(super::INPUT)
}

pub fn _count_passwords(input: &str) -> usize {
    let mut split = input.split('-');
    let first: usize = split.next().unwrap().parse().unwrap();
    let second: usize = split.next().unwrap().parse().unwrap();

    (first..=second)
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

fn password_has_double(digits: &[u8]) -> bool {
    let mut count = BTreeMap::default();
    for d in digits {
        *count.entry(d).or_insert(0) += 1;
    }

    count.iter().filter(|(_, count)| **count == 2).count() >= 1
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_password_valid1() {
        assert!(super::is_password(112_233));
    }

    #[test]
    fn is_password_valid2() {
        assert!(super::is_password(111_122));
    }

    #[test]
    fn is_password_invalid1() {
        assert!(!super::is_password(123_444));
    }
}

#[cfg(test)]
mod benchs {
    extern crate test;

    use test::Bencher;

    #[bench]
    fn is_password(b: &mut Bencher) {
        let input = test::black_box(12345);

        b.iter(|| super::is_password(input))
    }

    #[bench]
    fn count_passwords_single(b: &mut Bencher) {
        let input = test::black_box("0-1");

        b.iter(|| super::_count_passwords(input))
    }

    #[bench]
    fn count_passwords_lots(b: &mut Bencher) {
        let input = test::black_box("0-10000");

        b.iter(|| super::_count_passwords(input))
    }

    #[bench]
    fn count_passwords_input(b: &mut Bencher) {
        let input = test::black_box(crate::day_04::INPUT);

        b.iter(|| super::_count_passwords(input))
    }
}
