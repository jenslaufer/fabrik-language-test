# Add `triple_value` to the R stack

## Implementation
1. In `r/R/calc.R`, add `triple_value(n)` that returns `n * 3`. Mark it
   `@export` and add it to `r/NAMESPACE`.
2. In `r/tests/testthat/test-calc.R`, add a `test_that` block that
   checks `triple_value(0)`, `triple_value(4)`, and `triple_value(-3)`.

## Acceptance
- [ ] `triple_value(0) == 0`, `triple_value(4) == 12`, `triple_value(-3) == -9`.
- [ ] The new `test_that` block passes alongside the existing one.

Run `Rscript -e 'testthat::test_dir("tests/testthat")'` from `r/`.
