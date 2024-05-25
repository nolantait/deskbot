# frozen_string_literal: true

module Deskbot
  class Bitmap
    Rgba = Types::Array
      .of(Types::Integer.constrained(gteq: 0, lteq: 255))
      .constrained(size: 4)

    def bounds
      Area.new(_bounds)
    end

    def color_at(x, y) # rubocop:disable Naming/MethodParameterName
      red, green, blue, alpha = _get_pixel(x, y)
      Color.new(red:, green:, blue:, alpha:)
    end

    def eql?(image_path, tolerance: nil)
      _bitmap_eq(
        Types::String[image_path],
        Types::Float.optional[tolerance]
      )
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
        Types::Float.optional[tolerance]
      )
        .map { |point| Point.new(point) }
    end

    def all_color(color, tolerance: nil)
      _all_color(
        Rgba[color],
        Types::Float.optional[tolerance]
      )
        .map { |point| Point.new(point) }
    end

    def count(image_path, tolerance: nil)
      _count(
        Types::String[image_path],
        Types::Float.optional[tolerance]
      )
    end
  end
end
