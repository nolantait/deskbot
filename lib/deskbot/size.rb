module Deskbot
  class Size < Dry::Struct
    attribute :width, Types::Float
    attribute :height, Types::Float

    transform_keys(&:to_sym)
  end
end
