# frozen_string_literal: true

require "dry/types"
require "dry/struct"

require_relative "deskbot/version"
require_relative "deskbot/deskbot"
require_relative "deskbot/types"
require_relative "deskbot/point"
require_relative "deskbot/color"
require_relative "deskbot/size"
require_relative "deskbot/area"
require_relative "deskbot/bitmap"

require "deskbot/providers/autopilot/bitmap"
require "deskbot/providers/autopilot"

require "deskbot/screen"

module Deskbot
  class Error < StandardError; end

  module_function

  def self.screen(provider: Providers::Autopilot)
    @screen ||= Screen.new(provider)
  end
end
