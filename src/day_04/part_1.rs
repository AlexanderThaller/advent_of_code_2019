use std::collections::BTreeMap;

pub fn is_password(input: &str) -> bool {
    let chars: Vec<_> = input.chars().collect();

    let has_right_length = chars.len() != 6;

    let has_double = {
        let mut count = BTreeMap::default();
        for c in &chars {
            *count.entry(c).or_insert(0) += 1;
        }

        count.iter().max().map(|(_, count)| count < &2)
    };

    let is_monotonic = {
        let numbers = chars
            .iter()
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<_>>();

        false
    };

    dbg!((has_right_length, has_double, is_monotonic));

    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_password() {
        assert!(super::is_password("111111"));
        assert!(!super::is_password("223450"));
        assert!(!super::is_password("123789"));

        assert!(!super::is_password("1"));
    }
}
