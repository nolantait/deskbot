# frozen_string_literal: true

module Deskbot
  class Bitmap
    def bounds
      Area.new(_bounds)
    end

    def find(image_path, tolerance: nil)
      Point.new(_find(image_path, tolerance))
    end

    def find_color(color, tolerance: nil)
      Point.new(_find_color(color, tolerance))
    end

    def all(image_path, tolerance: nil)
      _all(image_path, tolerance).map { |point| Point.new(point) }
    end

    def count(image_path, tolerance: nil)
      _count(image_path, tolerance)
    end
  end
end
