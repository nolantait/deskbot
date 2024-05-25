# frozen_string_literal: true

module Deskbot
  class Color < Dry::Struct
    attribute :red, Types::Integer
    attribute :green, Types::Integer
    attribute :blue, Types::Integer
    attribute :alpha, Types::Integer

    transform_keys do |key|
      {
        "r" => :red,
        "g" => :green,
        "b" => :blue,
        "a" => :alpha
        # rubocop:enable Style/StringHashKeys
      }.fetch(key, key)
    end

    def self.from_hex(hex)
      red, green, blue, alpha = hex
        .match(/^#(..)(..)(..)(..)?$/)
        .captures
        .map do |hex_pair|
          hex_pair&.hex
        end

      new(
        red:,
        green:,
        blue:,
        alpha: alpha || 255
      )
    end
  end
end
