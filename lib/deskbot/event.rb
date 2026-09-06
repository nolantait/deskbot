# frozen_string_literal: true

module Deskbot
  # A single device event (key/mouse down, key/mouse up, or mouse move) yielded
  # by {Screen#listen}. Events are discriminated by their `type` and only the
  # fields relevant to that type are populated.
  #
  # @example
  #   Deskbot.screen.listen do |event|
  #     case event.type
  #     when :mouse_move then puts "moved to #{event.x}, #{event.y}"
  #     when :key_down   then puts "pressed #{event.key}"
  #     when :mouse_up   then puts "released #{event.button}"
  #     end
  #   end
  class Event
    EVENT_TYPES = %i[key_down key_up mouse_down mouse_up mouse_move].freeze

    attr_reader :type, :key, :button, :x, :y

    def initialize(type:, key: nil, button: nil, x: nil, y: nil) # rubocop:disable Naming/MethodParameterName
      @type = Types::EventType[type]
      @key = key&.to_sym
      @button = button
      @x = x
      @y = y
    end

    # Builds an event from the raw hash yielded by the native listener.
    def self.build(payload)
      new(
        type: payload["type"],
        key: payload["key"],
        button: payload["button"],
        x: payload["x"],
        y: payload["y"]
      )
    end

    def coords
      [x, y] if x && y
    end

    def key_event?
      %i[key_down key_up].include?(type)
    end

    def mouse_event?
      %i[mouse_down mouse_up mouse_move].include?(type)
    end

    def move?
      type == :mouse_move
    end

    def to_h
      { type:, key:, button:, x:, y: }
    end
  end
end
