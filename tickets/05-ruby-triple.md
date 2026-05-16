# Add `Calc.triple_value` to the Ruby stack

## Implementation
1. In `ruby/lib/calc.rb`, add `def triple_value(n); n * 3; end` to the
   `Calc` module.
2. In `ruby/spec/calc_spec.rb`, add a sibling `describe ".triple_value"`
   block asserting `Calc.triple_value(0) == 0`, `4 → 12`, and `-3 → -9`.

## Acceptance
- [ ] `Calc.triple_value(0) == 0`, `Calc.triple_value(4) == 12`,
      `Calc.triple_value(-3) == -9`.
- [ ] Both `.double_value` and `.triple_value` describe blocks pass
      under `bundle exec rspec`.

Run `bundle exec rspec` from `ruby/`.
