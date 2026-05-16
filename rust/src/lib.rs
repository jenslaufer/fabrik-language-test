// Fabrik language-test — Rust stack.
// Starter function: every Rust ticket adds a sibling here.

pub fn double_value(n: i64) -> i64 {
    n * 2
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
}
