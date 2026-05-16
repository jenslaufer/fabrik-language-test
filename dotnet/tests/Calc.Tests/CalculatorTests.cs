using Xunit;

namespace Calc.Tests;

public class CalculatorTests
{
    [Theory]
    [InlineData(0L, 0L)]
    [InlineData(3L, 6L)]
    [InlineData(-2L, -4L)]
    public void DoubleValue_DoublesItsInput(long input, long expected)
    {
        Assert.Equal(expected, Calculator.DoubleValue(input));
    }

    [Theory]
    [InlineData(0L, 0L)]
    [InlineData(4L, 12L)]
    [InlineData(-3L, -9L)]
    public void TripleValue_TriplesItsInput(long input, long expected)
    {
        Assert.Equal(expected, Calculator.TripleValue(input));
    }
}
