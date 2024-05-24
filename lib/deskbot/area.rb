# frozen_string_literal: true

module Deskbot
  class Area < Dry::Struct
    attribute :x, Types::Float
    attribute :y, Types::Float
    attribute :width, Types::Float
    attribute :height, Types::Float

    transform_keys(&:to_sym)
  end
end
