# frozen_string_literal: true

module Deskbot
  class Screen
    # This is the API for any provider

    def initialize(provider)
      @provider = provider
    end

    def alert(message)
      @provider.alert(Types::String[message])
    end

    def type(text, flags: [], wpm: 60.0, noise: 0.0)
      @provider.type(
        Types::String[text],
        Types::Flags[flags],
        Types::Float[wpm],
        Types::Float[noise]
      )
    end

    def toggle_key(key, down: true, flags: [], modifier_delay_ms: 0.0)
      @provider.toggle_key(
        Types::Character[key],
        Types::Bool[down],
        Types::Flags[flags],
        Types::Float[modifier_delay_ms]
      )
    end

    def tap_key(key, flags: [], delay_ms: 0.0, modifier_delay_ms: 0.0)
      @provider.tap_key(
        Types::Character[key],
        Types::Flags[flags],
        Types::Float[delay_ms],
        Types::Float[modifier_delay_ms]
      )
    end

    def mouse_location
      Point.new(@provider.mouse_location)
    end

    def move_mouse(x, y) # rubocop:disable Naming/MethodParameterName
      @provider.move_mouse(
        Types::Coercible::Float[x],
        Types::Coercible::Float[y]
      )
    end

    def smooth_move_mouse(x, y, duration: 1) # rubocop:disable Naming/MethodParameterName
      @provider.smooth_move_mouse(
        Types::Coercible::Float[x],
        Types::Coercible::Float[y],
        Types::Coercible::Float[duration]
      )
    end

    def toggle_mouse(button = "left", down: true)
      @provider.toggle_mouse(
        Types::Button[button],
        Types::Bool[down]
      )
    end

    def click(button = "left", delay_ms: nil)
      @provider.click(
        Types::Button[button],
        Types::Float.optional[delay_ms]
      )
    end

    def scroll(direction = "up", clicks: 1)
      @provider.scroll(
        Types::ScrollDirection[direction],
        Types::Integer[clicks]
      )
    end

    def color_at(x, y) # rubocop:disable Naming/MethodParameterName
      red, green, blue, alpha = @provider.color_at(
        Types::Coercible::Float[x],
        Types::Coercible::Float[y]
      )

      Color.new(red:, green:, blue:, alpha:)
    end

    def screen_size
      Size.new(@provider.screen_size)
    end

    def screen_scale
      @provider.screen_scale
    end

    def point_visible?(x, y) # rubocop:disable Naming/MethodParameterName
      @provider.is_point_visible(
        Types::Coercible::Float[x],
        Types::Coercible::Float[y]
      )
    end

    def area_visible?(x:, y:, width:, height:) # rubocop:disable Naming/MethodParameterName
      @provider.is_area_visible(
        Types::Coercible::Float[x],
        Types::Coercible::Float[y],
        Types::Coercible::Float[width],
        Types::Coercible::Float[height]
      )
    end

    def capture_screen
      bitmap = @provider.capture_screen
      Deskbot::Bitmap.new(bitmap)
    end

    def capture_screen_area(x:, y:, width:, height:) # rubocop:disable Naming/MethodParameterName
      bitmap = @provider.capture_screen_area(
        Types::Coercible::Float[x],
        Types::Coercible::Float[y],
        Types::Coercible::Float[width],
        Types::Coercible::Float[height]
      )

      Deskbot::Bitmap.new(bitmap)
    end
  end
end
