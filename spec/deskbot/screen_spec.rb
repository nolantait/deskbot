RSpec.describe Deskbot::Screen do
  let(:screen) { described_class.new(provider) }

  describe "#alert" do
    let(:provider) { double("provider", alert: nil) }

    it "calls alert on the provider" do
      expect(provider).to receive(:alert).with("hello")
      screen.alert("hello")
    end
  end

  describe "#type" do
    let(:provider) { double("provider", type: nil) }

    it "calls type on the provider" do
      expect(provider).to receive(:type).with("hello", [], 60.0, 0.0)
      screen.type("hello")
    end
  end

  describe "#toggle_key" do
    let(:provider) { double("provider", toggle_key: nil) }

    it "calls toggle_key on the provider" do
      expect(provider).to receive(:toggle_key).with("a", true, [], 0.0)
      screen.toggle_key("a")
    end
  end

  describe "#tap_key" do
    let(:provider) { double("provider", tap_key: nil) }

    it "calls tap_key on the provider" do
      expect(provider).to receive(:tap_key).with("a", [], 0.0, 0.0)
      screen.tap_key("a")
    end
  end

  describe "#mouse_location" do
    let(:provider) { double("provider", mouse_location: { "x" => 1.0, "y" => 1.0 }) }

    it "returns a point" do
      point = screen.mouse_location

      expect(point).to be_a(Deskbot::Point)
    end
  end

  describe "#move_mouse" do
    let(:provider) { double("provider", move_mouse: nil) }

    it "calls move_mouse on the provider" do
      expect(provider).to receive(:move_mouse).with(1.0, 1.0)
      screen.move_mouse(1, 1)
    end
  end

  describe "#smooth_move_mouse" do
    let(:provider) { double("provider", smooth_move_mouse: nil) }

    it "calls smooth_move_mouse on the provider" do
      expect(provider).to receive(:smooth_move_mouse).with(1.0, 1.0, 1.0)
      screen.smooth_move_mouse(1, 1)
    end
  end

  describe "#toggle_mouse" do
    let(:provider) { double("provider", toggle_mouse: nil) }

    it "calls toggle_mouse on the provider" do
      expect(provider).to receive(:toggle_mouse).with("left", true)
      screen.toggle_mouse
    end
  end

  describe "#click" do
    let(:provider) { double("provider", click: nil) }

    it "calls click on the provider" do
      expect(provider).to receive(:click).with("left", nil)
      screen.click
    end
  end

  describe "#scroll" do
    let(:provider) { double("provider", scroll: nil) }

    it "calls scroll on the provider" do
      expect(provider).to receive(:scroll).with("up", 1.0)
      screen.scroll
    end
  end

  describe "#color_at" do
    let(:provider) { double("provider", color_at: [255, 255, 255, 255]) }

    it "returns a color" do
      color = screen.color_at(1, 1)
      expect(color).to be_a(Deskbot::Color)
    end
  end

  describe "#screen_size" do
    let(:provider) { double("provider", screen_size: { "width" => 1920.0, "height" => 1080.0 }) }

    it "returns a size" do
      size = screen.screen_size
      expect(size).to be_a(Deskbot::Size)
    end
  end

  describe "#screen_scale" do
    let(:provider) { double("provider", screen_scale: 1.0) }

    it "returns a float" do
      expect(screen.screen_scale).to eq 1.0
    end
  end

  describe "#point_visible?" do
    let(:provider) { double("provider", is_point_visible: true) }

    it "returns a boolean" do
      expect(screen.point_visible?(1, 1)).to be true
    end
  end

  describe "#area_visible?" do
    let(:provider) { double("provider", is_area_visible: true) }

    it "returns a boolean" do
      expect(screen.area_visible?(x: 1, y: 1, width: 2, height: 2)).to be true
    end
  end

  describe "#capture_screen" do
    let(:provider) { double("provider", capture_screen: double("Bitmap")) }

    it "returns a bitmap" do
      expect(screen.capture_screen).to be_a(Deskbot::Bitmap)
    end
  end

  describe "#capture_screen_area" do
    let(:provider) { double("provider", capture_screen_area: double("Bitmap")) }

    it "returns a bitmap" do
      expect(screen.capture_screen_area(x: 1, y: 1, width: 2, height: 2)).to be_a(Deskbot::Bitmap)
    end
  end
end
