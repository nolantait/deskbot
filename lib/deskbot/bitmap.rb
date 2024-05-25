# frozen_string_literal: true

module Deskbot
  class Bitmap
    Rgba = Types::Array
      .of(Types::Integer.constrained(gteq: 0, lteq: 255))
      .constrained(size: 4)

    def bounds
      Area.new(_bounds)
    end

    def find(image_path, tolerance: nil)
      Point.new(
        _find(
          Types::String[image_path],
          Types::Float.optional[tolerance]
        )
      )
    end

    def find_color(color, tolerance: nil)
      Point.new(
        _find_color(
          Rgba[color],
          Types::Float.optional[tolerance]
        )
      )
    end

    def all(image_path, tolerance: nil)
      _all(
        Types::String[image_path],
        Types::Float.optional(tolerance)
      ).map { |point| Point.new(point) }
    end

    def count(image_path, tolerance: nil)
      _count(image_path, tolerance)
    end
  end
end
