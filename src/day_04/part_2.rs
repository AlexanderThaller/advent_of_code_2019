use rayon::prelude::*;
use std::collections::BTreeMap;

use crate::day_04::part_1::{
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
        .map(|password| password.to_string())
        .filter(|password| is_password(password))
        .count()
}

pub fn is_password(password: &str) -> bool {
    let chars: Vec<_> = password.chars().collect();

    if !password_has_right_length(&chars) {
        return false;
    }

    if !password_has_double(&chars) {
        return false;
    }

    if !password_is_monotonic(&chars) {
        return false;
    }

    return true;
}

fn password_has_double(chars: &[char]) -> bool {
    let mut count = BTreeMap::default();
    for c in chars {
        *count.entry(c).or_insert(0) += 1;
    }

    count.iter().filter(|(_, count)| **count == 2).count() >= 1
}

#[cfg(test)]
mod tests {
    extern crate test;

    use test::Bencher;

    #[test]
    fn is_password_valid1() {
        assert!(super::is_password("112233"));
    }

    #[test]
    fn is_password_valid2() {
        assert!(super::is_password("111122"));
    }

    #[test]
    fn is_password_invalid1() {
        assert!(!super::is_password("123444"));
    }

    #[bench]
    fn bench_is_password(b: &mut Bencher) {
        let input = test::black_box("12345");

        b.iter(|| super::is_password(input))
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

    #[bench]
    fn bench_count_passwords_input(b: &mut Bencher) {
        let input = test::black_box(crate::day_04::INPUT);

        b.iter(|| super::_count_passwords(input))
    }
}
