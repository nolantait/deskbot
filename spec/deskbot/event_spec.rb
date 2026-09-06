# frozen_string_literal: true

RSpec.describe Deskbot::Event do
  describe ".build" do
    it "builds a mouse_move event" do
      event = described_class.build(
        "type" => "mouse_move",
        "x" => 10,
        "y" => 20
      )

      expect(event.type).to eq(:mouse_move)
      expect(event.x).to eq(10)
      expect(event.y).to eq(20)
      expect(event.coords).to eq([10, 20])
      expect(event).to be_move
      expect(event).to be_mouse_event
      expect(event).not_to be_key_event
    end

    it "builds a key_down event with a symbol key" do
      event = described_class.build("type" => "key_down", "key" => "l_control")

      expect(event.type).to eq(:key_down)
      expect(event.key).to eq(:l_control)
      expect(event).to be_key_event
      expect(event.coords).to be_nil
    end

    it "builds a mouse_up event with a numeric button" do
      event = described_class.build("type" => "mouse_up", "button" => 1)

      expect(event.type).to eq(:mouse_up)
      expect(event.button).to eq(1)
    end
  end

  describe "#to_h" do
    it "exposes the attributes" do
      event = described_class.new(type: :mouse_move, x: 1, y: 2)

      expect(event.to_h).to eq(type: :mouse_move, key: nil, button: nil, x: 1, y: 2)
    end
  end

  it "rejects unknown event types" do
    expect { described_class.new(type: :hover) }
      .to raise_error(Dry::Types::ConstraintError)
  end
end
