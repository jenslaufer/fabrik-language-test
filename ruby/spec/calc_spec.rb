RSpec.describe Calc do
  describe ".double_value" do
    it "returns n * 2" do
      expect(Calc.double_value(0)).to eq(0)
      expect(Calc.double_value(3)).to eq(6)
      expect(Calc.double_value(-2)).to eq(-4)
    end
  end

  describe ".triple_value" do
    it "returns n * 3" do
      expect(Calc.triple_value(0)).to eq(0)
      expect(Calc.triple_value(4)).to eq(12)
      expect(Calc.triple_value(-3)).to eq(-9)
    end
  end

  describe ".ruby_version_ok?" do
    it "returns true on Ruby 3.2 or later" do
      expect(Calc.ruby_version_ok?).to be(true)
    end
  end
end
