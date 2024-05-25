# frozen_string_literal: true

RSpec.describe Deskbot do
  it "has a version number" do
    expect(Deskbot::VERSION).not_to be_nil
  end

  describe ".screen" do
    it "returns a screen" do
      expect(described_class.screen).to be_a(Deskbot::Screen)
    end
  end
end
