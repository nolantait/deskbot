# frozen_string_literal: true

require "dry/types"

require_relative "deskbot/version"
require_relative "deskbot/deskbot"
require_relative "deskbot/types"

module Deskbot
  class Error < StandardError; end
  # Your code goes here...

  Flag = Types::Coercible::String.enum(
    "shift",
    "control",
    "alt",
    "meta",
    "help"
  )

  Flags = Types::Array.of(Flag)

  def self.type_string(text, flags: [], wpm: 60.0, noise: 0.0)
    _type_string(text, Flags[flags], wpm, noise)
  end
end
