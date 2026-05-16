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
}
