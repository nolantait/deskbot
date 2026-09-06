# frozen_string_literal: true

RSpec.describe Deskbot::Event do
  let(:recorded_at) { Time.at(1_700_000_000.5) }
  let(:base_payload) { { "recorded_at" => recorded_at.to_f } }

  describe ".build" do
    it "builds a mouse_move event" do
      event = described_class.build(base_payload.merge("type" => "mouse_move", "x" => 10, "y" => 20))

      expect(event).to be_a(Deskbot::Event::MouseMove)
      expect(event.x).to eq(10)
      expect(event.y).to eq(20)
      expect(event.coords).to eq([10, 20])
      expect(event.recorded_at).to be_a(Time)
      expect(event.recorded_at).to eq(recorded_at)
    end

    it "builds a key_down event with a symbol key" do
      event = described_class.build(base_payload.merge("type" => "key_down", "key" => "l_control"))

      expect(event).to be_a(Deskbot::Event::KeyDown)
      expect(event.key).to eq(:l_control)
      expect(event.recorded_at).to eq(recorded_at)
    end

    it "builds a key_up event" do
      event = described_class.build(base_payload.merge("type" => "key_up", "key" => "space"))

      expect(event).to be_a(Deskbot::Event::KeyUp)
      expect(event.key).to eq(:space)
      expect(event.recorded_at).to eq(recorded_at)
    end

    it "builds a mouse_down event with a numeric button" do
      event = described_class.build(base_payload.merge("type" => "mouse_down", "button" => 1))

      expect(event).to be_a(Deskbot::Event::MouseDown)
      expect(event.button).to eq(1)
      expect(event.recorded_at).to eq(recorded_at)
    end

    it "builds a mouse_up event with a numeric button" do
      event = described_class.build(base_payload.merge("type" => "mouse_up", "button" => 1))

      expect(event).to be_a(Deskbot::Event::MouseUp)
      expect(event.button).to eq(1)
      expect(event.recorded_at).to eq(recorded_at)
    end
  end

  it "builds every type as an Event" do
    %w[key_down key_up mouse_down mouse_up mouse_move].each do |type|
      payload = base_payload.merge("type" => type, "key" => "space", "button" => 1, "x" => 1, "y" => 2)
      expect(described_class.build(payload)).to be_a(described_class)
    end
  end

  it "rejects unknown event types" do
    expect { described_class.build("type" => "hover") }
      .to raise_error(ArgumentError, "unknown event type: \"hover\"")
  end
end
