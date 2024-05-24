module Deskbot
  class Bitmap
    def bounds
      Area.new(_bounds)
    end

    def find(image_path)
      Point.new(_find(image_path))
    end
  end
end
