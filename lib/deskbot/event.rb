# frozen_string_literal: true

module Deskbot
  # Base class for the distinct device events yielded by {Screen#listen}.
  #
  # Listening yields an instance of one of the concrete subclasses, each of
  # which exposes only the fields relevant to it:
  #
  # - {Event::KeyDown} / {Event::KeyUp} - expose `key`
  # - {Event::MouseDown} / {Event::MouseUp} - expose `button`
  # - {Event::MouseMove} - exposes `x`, `y` and `coords`
  #
  # Dispatch on the event's class rather than on a `type` value:
  #
  #   Deskbot.screen.listen do |event|
  #     case event
  #     when Deskbot::Event::KeyDown   then puts "pressed #{event.key}"
  #     when Deskbot::Event::KeyUp     then puts "released #{event.key}"
  #     when Deskbot::Event::MouseDown then puts "button down: #{event.button}"
  #     when Deskbot::Event::MouseUp   then puts "button up: #{event.button}"
  #     when Deskbot::Event::MouseMove then puts event.coords.inspect
  #     end
  #   end
  #
  # Base `Event` is abstract and is never instantiated directly; use
  # {Event.build} to turn a native payload into the matching subclass.
  class Event
    # The time the event was recorded. Every concrete event exposes it.
    attr_reader :recorded_at

    # Builds the concrete event described by the raw hash yielded by the native
    # listener.
    def self.build(payload)
      subclass = {
        "key_down" => Event::KeyDown,
        "key_up" => Event::KeyUp,
        "mouse_down" => Event::MouseDown,
        "mouse_up" => Event::MouseUp,
        "mouse_move" => Event::MouseMove
      }[payload["type"]]

      unless subclass
        raise ArgumentError,
          "unknown event type: #{payload["type"].inspect}"
      end

      subclass.build(payload)
    end

    # Builds the recorded-at `Time` for an event from its native payload.
    def self.recorded_at_from(payload)
      Time.at(payload["recorded_at"])
    end

    # A key was pressed down. Exposes the pressed {#key}.
    class KeyDown < Event
      attr_reader :key

      def initialize(key:, recorded_at:)
        @key = key.to_sym
        @recorded_at = recorded_at
      end

      def self.build(payload)
        new(key: payload["key"], recorded_at: recorded_at_from(payload))
      end
    end

    # A key was released. Exposes the released {#key}.
    class KeyUp < Event
      attr_reader :key

      def initialize(key:, recorded_at:)
        @key = key.to_sym
        @recorded_at = recorded_at
      end

      def self.build(payload)
        new(key: payload["key"], recorded_at: recorded_at_from(payload))
      end
    end

    # A mouse button was pressed down. Exposes the pressed {#button}.
    class MouseDown < Event
      attr_reader :button

      def initialize(button:, recorded_at:)
        @button = button
        @recorded_at = recorded_at
      end

      def self.build(payload)
        new(button: payload["button"], recorded_at: recorded_at_from(payload))
      end
    end

    # A mouse button was released. Exposes the released {#button}.
    class MouseUp < Event
      attr_reader :button

      def initialize(button:, recorded_at:)
        @button = button
        @recorded_at = recorded_at
      end

      def self.build(payload)
        new(button: payload["button"], recorded_at: recorded_at_from(payload))
      end
    end

    # The mouse moved to a new position. Exposes {#x}, {#y} and {#coords}.
    class MouseMove < Event
      attr_reader :x, :y

      def initialize(x:, y:, recorded_at:) # rubocop:disable Naming/MethodParameterName
        @x = x
        @y = y
        @recorded_at = recorded_at
      end

      def self.build(payload)
        new(
          x: payload["x"],
          y: payload["y"],
          recorded_at: recorded_at_from(payload)
        )
      end

      # Returns the mouse position as `[x, y]`.
      def coords
        [x, y]
      end
    end
  end
end
