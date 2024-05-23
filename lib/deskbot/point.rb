module Deskbot
  class Point < Dry::Struct
    attribute :x, Types::Float
    attribute :y, Types::Float

    transform_keys(&:to_sym)
  end
end
