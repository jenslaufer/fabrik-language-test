// Fabrik language-test — Rust stack.
// Starter function: every Rust ticket adds a sibling here.

pub fn double_value(n: i64) -> Option<i64> {
    n.checked_mul(2)
}

pub fn triple_value(n: i64) -> i64 {
    n * 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn double_value_doubles_its_input() {
        assert_eq!(double_value(0), Some(0));
        assert_eq!(double_value(3), Some(6));
        assert_eq!(double_value(-2), Some(-4));
    }

    #[test]
    fn double_value_returns_none_on_overflow() {
        assert_eq!(double_value(i64::MAX), None);
    }

    #[test]
    fn triple_value_triples_its_input() {
        assert_eq!(triple_value(0), 0);
        assert_eq!(triple_value(4), 12);
        assert_eq!(triple_value(-3), -9);
    }
}
