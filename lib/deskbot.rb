# frozen_string_literal: true

require "dry/types"
require "dry/struct"

require_relative "deskbot/version"
require_relative "deskbot/deskbot"
require_relative "deskbot/types"
require_relative "deskbot/point"

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

  def toggle_mouse(button, down: true)
    _toggle_mouse(
      Button[button],
      Types::Bool[down]
    )
  end

  def click_mouse(button, delay_ms: nil)
    _click_mouse(
      Button[button],
      Types::Float.optional[delay_ms]
    )
  end
end
