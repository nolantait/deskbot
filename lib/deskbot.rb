# frozen_string_literal: true

require_relative "deskbot/version"
require_relative "deskbot/deskbot"

module Deskbot
  class Error < StandardError; end
  # Your code goes here...

  def self.type_string(text, flags: [], wpm: 60.0, noise: 0.0)
    _type_string(text, flags.map(&:to_s), wpm, noise)
  end
end
