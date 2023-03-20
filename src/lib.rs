use std::cmp::{max, min};

/// Calculates the Jaro distance between lhs and rhs.
/// The returned score is normalized to be between 0.0 and 1.0.
/// 0 means completely same.
/// 1 means no similarity.
pub fn jaro_distance(lhs: &str, rhs: &str) -> f64 {
    if lhs.is_empty() || rhs.is_empty() {
        return 0.0;
    }

    let w1 = 1_f64 / 3_f64;
    let w2 = 1_f64 / 3_f64;
    let wt = 1_f64 / 3_f64;

    let limit = max(lhs.len(), rhs.len()) / 2 - 1;

    let lhs_matched = {
        let mut matched: Vec<char> = Vec::with_capacity(lhs.len());
        let mut rhs = rhs.to_string();
        for (i, s1) in lhs.chars().enumerate() {
            let left = i.saturating_sub(limit);
            let right = min(i + limit + 1, rhs.len());
            if rhs[left..right].contains(s1) {
                let index = rhs[left..right].find(s1).expect("index not found") + left;
                rhs.replace_range(index..index + 1, " ");
                matched.push(s1);
            }
        }
        matched
    };

    let rhs_matched = {
        let mut matched: Vec<char> = Vec::with_capacity(rhs.len());
        let mut lhs = lhs.to_string();
        for (i, s2) in rhs.chars().enumerate() {
            let left = i.saturating_sub(limit);
            let right = min(i + limit + 1, lhs.len());
            if lhs[left..right].contains(s2) {
                let index = lhs[left..right].find(s2).expect("index not found") + left;
                lhs.replace_range(index..index + 1, " ");
                matched.push(s2);
            }
        }
        matched
    };

    let transpositions = lhs_matched
        .iter()
        .zip(rhs_matched.iter())
        .filter(|(s1, s2)| s1 != s2)
        .count()
        / 2;
    let count = lhs_matched.len();

    if count == 0 {
        0.0
    } else {
        let count = count as f64;
        let transpositions = transpositions as f64;
        w1 * (count / lhs.len() as f64)
            + w2 * (count / rhs.len() as f64)
            + wt * ((count - transpositions) / count)
    }
}

/// The length of the prefix to be considered when calculating the Jaro-Winkler distance.
/// For english, 4 is recommended.
/// For japanese, 2 is recommended.
pub enum PrefixLength {
    One,
    Two,
    Three,
    Four,
}

impl PrefixLength {
    pub fn to_isize(&self) -> isize {
        match self {
            PrefixLength::One => 1,
            PrefixLength::Two => 2,
            PrefixLength::Three => 3,
            PrefixLength::Four => 4,
        }
    }
}

/// Calculates the Jaro-Winkler distance between two strings.
/// The returned score is normalized to be between 0.0 and 1.0.
/// 0 means completely same.
/// 1 means no similarity.
pub fn jaro_winkler_distance(lhs: &str, rhs: &str, prefix_length: &PrefixLength) -> f64 {
    let jaro_distance = jaro_distance(lhs, rhs);

    let mut prefix = 0;
    for (s1, s2) in lhs.chars().zip(rhs.chars()) {
        if s1 == s2 {
            prefix += 1;
            if prefix >= prefix_length.to_isize() {
                break;
            }
        } else {
            break;
        }
    }

    let l = prefix as f64;
    let p = 0.1;

    jaro_distance + (l * p * (1.0 - jaro_distance))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_jaro_distance(lhs: &str, rhs: &str, expected: f64) {
        let actual = jaro_distance(lhs, rhs);
        assert_eq!(expected, actual, "jaro distance between {lhs} and {rhs}");
    }

    fn test_jaro_winkler_distance(lhs: &str, rhs: &str, expected: f64) {
        let actual = jaro_winkler_distance(lhs, rhs, &PrefixLength::Four);
        assert_eq!(
            expected, actual,
            "jaro winkler distance between {lhs} and {rhs}"
        );
    }

    #[test]
    fn jaro_distance_diffirent_prefix() {
        test_jaro_distance("hello", "world", 0.4666666666666667);
        test_jaro_distance("CRATE", "TRACE", 0.7333333333333333);
        test_jaro_distance("hello world", "HeLLo W0rlD", 0.6363636363636362);
    }

    #[test]
    fn jaro_distance_same_prefix() {
        test_jaro_distance("martha", "marhta", 0.9444444444444444);
        test_jaro_distance("marhta", "martha", 0.9444444444444444);
        test_jaro_distance("martha", "marhat", 0.9444444444444444);
        test_jaro_distance("saturday", "sunday", 0.7527777777777778);
        test_jaro_distance("abcdefg", "abcdzxy", 0.7142857142857142);
    }

    #[test]
    fn jaro_distance_completely_same_or_diffirent() {
        test_jaro_distance("hello", "hello", 1.0);
        test_jaro_distance("", "helloworld", 0.0);
        test_jaro_distance("gamemaster", "", 0.0);
    }

    #[test]
    fn jaro_winkler_distance_different_prefix() {
        test_jaro_winkler_distance("hello", "world", 0.4666666666666667);
        test_jaro_winkler_distance("CRATE", "TRACE", 0.7333333333333333);
        test_jaro_winkler_distance("hello world", "HeLLo W0rlD", 0.6363636363636362);
    }

    #[test]
    fn jaro_winkler_distance_same_prefix() {
        test_jaro_winkler_distance("martha", "marhta", 0.9611111111111111);
        test_jaro_winkler_distance("marhta", "martha", 0.9611111111111111);
        test_jaro_winkler_distance("martha", "marhat", 0.9611111111111111);
        test_jaro_winkler_distance("saturday", "sunday", 0.7775);
        test_jaro_winkler_distance("abcdefg", "abcdzxy", 0.8285714285714285);
    }

    #[test]
    fn jaro_winkler_distance_completely_same_or_diffirent() {
        test_jaro_winkler_distance("hello", "hello", 1.0);
        test_jaro_winkler_distance("", "helloworld", 0.0);
        test_jaro_winkler_distance("gamemaster", "", 0.0);
    }

    #[test]
    fn jaro_winkler_distance_prefix_length_option() {
        let lhs = "abcdefghijk";
        let rhs = "abcdzxywvut";

        // PrefixLength::One
        let actual = jaro_winkler_distance(lhs, rhs, &PrefixLength::One);
        let expected = 0.6181818181818182;
        assert_eq!(
            expected, actual,
            "jaro winkler distance between {lhs} and {rhs}; prefix length: 1"
        );

        // PrefixLength::Two
        let actual = jaro_winkler_distance(lhs, rhs, &PrefixLength::Two);
        let expected = 0.6606060606060605;
        assert_eq!(
            expected, actual,
            "jaro winkler distance between {lhs} and {rhs}; prefix length: 2"
        );

        // PrefixLength::Three
        let actual = jaro_winkler_distance(lhs, rhs, &PrefixLength::Three);
        let expected = 0.703030303030303;
        assert_eq!(
            expected, actual,
            "jaro winkler distance between {lhs} and {rhs}; prefix length: 3"
        );

        // PrefixLength::Four
        let actual = jaro_winkler_distance(lhs, rhs, &PrefixLength::Four);
        let expected = 0.7454545454545454;
        assert_eq!(
            expected, actual,
            "jaro winkler distance between {lhs} and {rhs}; prefix length: 4"
        );
    }
}
