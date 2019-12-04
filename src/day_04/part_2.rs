use rayon::prelude::*;
use std::collections::BTreeMap;

use crate::day_04::part_1::{
    password_has_right_length,
    password_is_monotonic,
};

pub fn count_passwords() -> usize {
    let mut split = super::INPUT.split('-');
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
    let has_no_larger_group = password_has_no_larger_group(&chars);
    let is_monotonic = password_is_monotonic(&chars);

    let larger_group_ok = if has_double {
        has_double
    } else {
        has_no_larger_group
    };

    [has_right_length, has_double, larger_group_ok, is_monotonic]
        .iter()
        .all(|r| *r)
}

fn password_has_double(chars: &[char]) -> bool {
    let mut count = BTreeMap::default();
    for c in chars {
        *count.entry(c).or_insert(0) += 1;
    }

    count.iter().filter(|(_, count)| **count == 2).count() >= 1
}

fn password_has_no_larger_group(chars: &[char]) -> bool {
    let mut count = BTreeMap::default();
    for c in chars {
        *count.entry(c).or_insert(0) += 1;
    }

    count
        .iter()
        .max_by(|(_, x), (_, y)| x.cmp(y))
        .map(|(_, count)| *count <= 2)
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
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

    #[test]
    fn password_has_no_larger_group_valid() {
        assert!(super::password_has_no_larger_group(&[
            '1', '1', '2', '2', '3', '3'
        ]));
    }

    #[test]
    fn password_has_no_larger_group_invalid() {
        assert!(!super::password_has_no_larger_group(&[
            '1', '2', '3', '4', '4', '4'
        ]));
    }
}
