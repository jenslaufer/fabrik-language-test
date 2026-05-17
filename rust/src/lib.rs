// Fabrik language-test — Rust stack.
// Starter function: every Rust ticket adds a sibling here.

pub fn double_value(n: i64) -> i64 {
    n * 2
}

pub fn triple_value(n: i64) -> i64 {
    n * 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn double_value_doubles_its_input() {
        assert_eq!(double_value(0), 0);
        assert_eq!(double_value(3), 6);
        assert_eq!(double_value(-2), -4);
    }

    #[test]
    fn triple_value_triples_its_input() {
        assert_eq!(triple_value(0), 0);
        assert_eq!(triple_value(4), 12);
        assert_eq!(triple_value(-3), -9);
    }
}
