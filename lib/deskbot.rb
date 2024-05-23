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

  def toggle(key, down: true, flags: [], modifier_delay_ms: 0.0)
    _toggle(
      Character[key],
      Types::Bool[down],
      Flags[flags],
      Types::Float[modifier_delay_ms]
    )
  end

  def tap(key, flags: [], delay_ms: 0.0, modifier_delay_ms: 0.0)
    _tap(
      Character[key],
      Flags[flags],
      Types::Float[delay_ms],
      Types::Float[modifier_delay_ms]
    )
  end

  def mouse_location
    Point.new(_location)
  end

  def move_mouse(x, y) # rubocop:disable Naming/MethodParameterName
    _move_to(
      Types::Coercible::Float[x],
      Types::Coercible::Float[y]
    )
  end
end
