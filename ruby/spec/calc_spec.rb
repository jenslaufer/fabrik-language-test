RSpec.describe Calc do
  describe ".double_value" do
    it "returns n * 2" do
      expect(Calc.double_value(0)).to eq(0)
      expect(Calc.double_value(3)).to eq(6)
      expect(Calc.double_value(-2)).to eq(-4)
    end
  end
end
