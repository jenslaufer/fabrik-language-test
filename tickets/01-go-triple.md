# Add `Triple` to the Go stack

## Implementation
1. In `go/calc.go`, add `Triple(n int) int` that returns `n * 3`.
2. In `go/calc_test.go`, add `TestTriple` with the same case-table shape
   as `TestDouble`, covering 0, 4, and -3.

## Acceptance
- [ ] `Triple(0) == 0`, `Triple(4) == 12`, `Triple(-3) == -9`.
- [ ] `TestTriple` passes alongside `TestDouble`.

Run `go test ./...` from `go/` to verify.
