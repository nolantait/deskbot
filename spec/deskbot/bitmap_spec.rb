# frozen_string_literal: true

RSpec.describe Deskbot::Bitmap do
  let(:bitmap) { described_class.new(provider) }

  describe "#save" do
    let(:provider) { double("provider", save: nil) }

    it "saves the bitmap" do
      expect(provider).to receive(:save).with("images/test.png")
      bitmap.save("images/test.png")
    end
  end

  describe "#bounds" do
    let(:provider) do
      double(
        "provider",
        bounds: {
          "x" => 1.0,
          "y" => 2.0,
          "width" => 1920.0,
          "height" => 1080.0
        }
      )
    end

    it "returns an area" do
      area = bitmap.bounds
      expect(area).to be_a(Deskbot::Area)
    end
  end

  describe "#color_at" do
    let(:provider) { double("provider", color_at: [255, 255, 255, 255]) }

    it "returns a color" do
      color = bitmap.color_at(1, 1)
      expect(color).to be_a(Deskbot::Color)
    end
  end

  describe "#eql?" do
    let(:provider) { double("provider", bitmap_eq: true) }

    it "returns a boolean" do
      result = bitmap.eql?("images/test.png")

      expect(result).to be true
    end
  end

  describe "#find" do
    context "when a point is found" do
      let(:provider) { double("provider", find: { "x" => 1.0, "y" => 2.0 }) }

      it "returns a point in a monad" do
        result = bitmap.find("images/test.png")
        expect(result).to be_success
        expect(result.success).to be_a(Deskbot::Point)
      end
    end

    context "when a point is not found" do
      let(:provider) { double("provider", find: nil) }

      it "returns a failure in a monad" do
        result = bitmap.find("images/test.png")
        expect(result).to be_failure
      end
    end
  end

  describe "#find_color" do
    context "when a point is found" do
      let(:provider) { double("provider", find_color: { "x" => 1.0, "y" => 2.0 }) }

      it "returns a point" do
        result = bitmap.find_color([255, 255, 255, 255])
        expect(result).to be_success
        expect(result.success).to be_a(Deskbot::Point)
      end
    end

    context "when a point is not found" do
      let(:provider) { double("provider", find_color: nil) }

      it "returns nil" do
        result = bitmap.find_color([255, 255, 255, 255])
        expect(result).to be_failure
      end
    end
  end

  describe "#all" do
    let(:provider) { double("provider", all: [{ "x" => 1.0, "y" => 2.0 }]) }

    it "returns a list of points" do
      points = bitmap.all("images/test.png")
      expect(points).to all(be_a(Deskbot::Point))
    end
  end

  describe "#all_color" do
    let(:provider) { double("provider", all_color: [{ "x" => 1.0, "y" => 2.0 }]) }

    it "returns a list of points" do
      points = bitmap.all_color([255, 255, 255, 255])
      expect(points).to all(be_a(Deskbot::Point))
    end
  end
end
