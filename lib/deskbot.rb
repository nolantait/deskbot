# frozen_string_literal: true

require "dry/types"
require "dry/struct"

require_relative "deskbot/version"
require_relative "deskbot/deskbot"
require_relative "deskbot/types"
require_relative "deskbot/point"
require_relative "deskbot/color"
require_relative "deskbot/size"

module Deskbot
  class Error < StandardError; end

  module_function

  Flag = Types::Coercible::String.enum(
    "shift",
    "control",
    "alt",
    "meta",
    "help"
  )

  Button = Types::Coercible::String.enum(
    "left",
    "middle",
    "right"
  )

  Flags = Types::Array.of(Flag)

  Character = Types::Coercible::String.constrained(size: 1)

  ScrollDirection = Types::Coercible::String.enum(
    "up",
    "down"
  )

  def type(text, flags: [], wpm: 60.0, noise: 0.0)
    _type_string(
      Types::String[text],
      Flags[flags],
      Types::Float[wpm],
      Types::Float[noise]
    )
  end

  def toggle_key(key, down: true, flags: [], modifier_delay_ms: 0.0)
    _toggle_key(
      Character[key],
      Types::Bool[down],
      Flags[flags],
      Types::Float[modifier_delay_ms]
    )
  end

  def tap_key(key, flags: [], delay_ms: 0.0, modifier_delay_ms: 0.0)
    _tap_key(
      Character[key],
      Flags[flags],
      Types::Float[delay_ms],
      Types::Float[modifier_delay_ms]
    )
  end

  def mouse_location
    Point.new(_mouse_location)
  end

  def move_mouse(x, y) # rubocop:disable Naming/MethodParameterName
    _move_mouse(
      Types::Coercible::Float[x],
      Types::Coercible::Float[y]
    )
  end

  def smooth_move_mouse(x, y, duration: 1) # rubocop:disable Naming/MethodParameterName
    _smooth_move_mouse(
      Types::Coercible::Float[x],
      Types::Coercible::Float[y],
      Types::Coercible::Float[duration]
    )
  end

  def toggle_mouse(button = "left", down: true)
    _toggle_mouse(
      Button[button],
      Types::Bool[down]
    )
  end

  def click(button = "left", delay_ms: nil)
    _click(
      Button[button],
      Types::Float.optional[delay_ms]
    )
  end

  def scroll(direction = "up", clicks: 1)
    _scroll(
      ScrollDirection[direction],
      Types::Integer[clicks]
    )
  end

  def get_color(x, y) # rubocop:disable Naming/MethodParameterName
    Color.new(
      _get_color(
        Types::Coercible::Float[x],
        Types::Coercible::Float[y]
      )
    )
  end

  def screen_size
    Size.new(_screen_size)
  end

  def screen_scale
    _screen_scale
  end

  def point_visible?(x, y) # rubocop:disable Naming/MethodParameterName
    _is_point_visible(
      Types::Coercible::Float[x],
      Types::Coercible::Float[y]
    )
  end

  def rect_visible?(x:, y:, width:, height:) # rubocop:disable Naming/MethodParameterName
    _is_rect_visible(
      Types::Coercible::Float[x],
      Types::Coercible::Float[y],
      Types::Coercible::Float[width],
      Types::Coercible::Float[height]
    )
  end

  def capture_screen
    _capture_screen
  end

  def capture_screen_portion(x:, y:, width:, height:) # rubocop:disable Naming/MethodParameterName
    _capture_screen_portion(
      Types::Coercible::Float[x],
      Types::Coercible::Float[y],
      Types::Coercible::Float[width],
      Types::Coercible::Float[height]
    )
  end
end
