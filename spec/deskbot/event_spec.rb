# frozen_string_literal: true

RSpec.describe Deskbot::Event do
  describe ".build" do
    it "builds a mouse_move event" do
      event = described_class.build("type" => "mouse_move", "x" => 10, "y" => 20)

      expect(event).to be_a(Deskbot::Event::MouseMove)
      expect(event.x).to eq(10)
      expect(event.y).to eq(20)
      expect(event.coords).to eq([10, 20])
    end

    it "builds a key_down event with a symbol key" do
      event = described_class.build("type" => "key_down", "key" => "l_control")

      expect(event).to be_a(Deskbot::Event::KeyDown)
      expect(event.key).to eq(:l_control)
    end

    it "builds a key_up event" do
      event = described_class.build("type" => "key_up", "key" => "space")

      expect(event).to be_a(Deskbot::Event::KeyUp)
      expect(event.key).to eq(:space)
    end

    it "builds a mouse_down event with a numeric button" do
      event = described_class.build("type" => "mouse_down", "button" => 1)

      expect(event).to be_a(Deskbot::Event::MouseDown)
      expect(event.button).to eq(1)
    end

    it "builds a mouse_up event with a numeric button" do
      event = described_class.build("type" => "mouse_up", "button" => 1)

      expect(event).to be_a(Deskbot::Event::MouseUp)
      expect(event.button).to eq(1)
    end
  end

  it "builds every type as an Event" do
    %w[key_down key_up mouse_down mouse_up mouse_move].each do |type|
      payload = { "type" => type, "key" => "space", "button" => 1, "x" => 1, "y" => 2 }
      expect(described_class.build(payload)).to be_a(described_class)
    end
  end

  it "rejects unknown event types" do
    expect { described_class.build("type" => "hover") }
      .to raise_error(ArgumentError, "unknown event type: \"hover\"")
  end
end
