RSpec.describe Deskbot::Color do
  describe ".from_hex" do
    it "parses a hex color" do
      result = described_class.from_hex("#ff00ff00")

      expect(result.red).to eq(255)
      expect(result.green).to eq(0)
      expect(result.blue).to eq(255)
      expect(result.alpha).to eq(0)
    end

    it "parses a hex color without alpha" do
      result = described_class.from_hex("#ff00ff")

      expect(result.red).to eq(255)
      expect(result.green).to eq(0)
      expect(result.blue).to eq(255)
      expect(result.alpha).to eq(255)
    end
  end
end
