# frozen_string_literal: true

require_relative "deskbot/version"
require_relative "deskbot/deskbot"

module Deskbot
  class Error < StandardError; end
  # Your code goes here...

  def self.type_string(text, flag: :none, wpm: 60.0, noise: 0.0)
    _type_string(text, flag, wpm, noise)
  end
end
