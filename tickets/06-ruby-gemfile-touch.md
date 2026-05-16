# Pin RSpec and add a Ruby-version guard (touches `Gemfile`)

This ticket exists to exercise the **sensitive-manifest gate** half of
fabrik#199. It edits `ruby/Gemfile` — a stack manifest — so the worker
must open the PR with the operator-review banner per
`diff_touches_sensitive_paths`.

## Implementation
1. In `ruby/Gemfile`, tighten the RSpec constraint from
   `gem "rspec", "~> 3.13"` to `gem "rspec", "~> 3.13.0"`. (Same minor,
   no functional change — the edit is what matters.)
2. In `ruby/lib/calc.rb`, add `Calc.ruby_version_ok?` returning `true`
   iff `Gem::Version.new(RUBY_VERSION) >= Gem::Version.new("3.2")`.
3. In `ruby/spec/calc_spec.rb`, add a `describe ".ruby_version_ok?"`
   block asserting the method returns `true` on the CI runner.

## Acceptance
- [ ] `Calc.ruby_version_ok?` returns `true` under Ruby 3.2.x.
- [ ] `bundle exec rspec` passes the existing AND the new spec block.
- [ ] The PR's diff includes `ruby/Gemfile`.

Run `bundle exec rspec` from `ruby/` to verify.

## Expected operator-side evidence (fabrik#199 sensitive-paths gate)
The worker must open this PR with a `needs-human-review` banner because
`ruby/Gemfile` is in `_SENSITIVE_MANIFEST_NAMES`. The banner appearing
is the closing evidence for the sensitive-manifest widening — if the PR
opens *without* the banner, the gate isn't firing on non-Python stacks
and the issue stays open.
