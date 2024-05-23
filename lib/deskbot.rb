# frozen_string_literal: true

require "dry/types"

require_relative "deskbot/version"
require_relative "deskbot/deskbot"
require_relative "deskbot/types"

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

  def type_string(text, flags: [], wpm: 60.0, noise: 0.0)
    _type_string(text, Flags[flags], wpm, noise)
  end

  def toggle(key, down: true, flags: [], modifier_delay_ms: 0.0)
    _toggle(Character[key], down, Flags[flags], modifier_delay_ms)
  end

  def tap(key, flags: [], delay_ms: 0.0, modifier_delay_ms: 0.0)
    _tap(Character[key], Flags[flags], delay_ms, modifier_delay_ms)
  end
end
