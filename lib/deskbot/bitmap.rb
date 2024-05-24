module Deskbot
  class Bitmap
    def bounds
      Area.new(_bounds)
    end

    def find(image_path)
      Point.new(_find(image_path))
    end

    def all(image_path)
      _all(image_path).map { |point| Point.new(point) }
    end
  end
end
