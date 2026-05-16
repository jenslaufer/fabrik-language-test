# Add `TripleValue` to the .NET stack

## Implementation
1. In `dotnet/src/Calc/Calculator.cs`, add
   `public static long TripleValue(long n) => n * 3;`.
2. In `dotnet/tests/Calc.Tests/CalculatorTests.cs`, add an xUnit
   `[Theory]` `TripleValue_TriplesItsInput` with `InlineData(0L, 0L)`,
   `InlineData(4L, 12L)`, `InlineData(-3L, -9L)`.

## Acceptance
- [ ] `Calculator.TripleValue(0) == 0`, `TripleValue(4) == 12`, `TripleValue(-3) == -9`.
- [ ] Both `DoubleValue_DoublesItsInput` and `TripleValue_TriplesItsInput`
      pass under `dotnet test`.

Run `dotnet test` from `dotnet/`.
