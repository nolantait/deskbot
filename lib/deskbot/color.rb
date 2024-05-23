module Deskbot
  class Color < Dry::Struct
    attribute :red, Types::Integer
    attribute :green, Types::Integer
    attribute :blue, Types::Integer
    attribute :alpha, Types::Integer

    transform_keys do |key|
      {
        # rubocop:disable Style/StringHashKeys
        "r" => :red,
        "g" => :green,
        "b" => :blue,
        "a" => :alpha
        # rubocop:enable Style/StringHashKeys
      }.fetch(key)
    end
  end
end
