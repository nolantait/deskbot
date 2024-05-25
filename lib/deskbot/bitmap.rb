# frozen_string_literal: true

module Deskbot
  class Bitmap
    include Dry::Monads[:result]
    include Dry::Matcher.for(
      :find_color,
      :find,
      with: Dry::Matcher::ResultMatcher
    )

    Rgba = Types::Array
      .of(Types::Integer.constrained(gteq: 0, lteq: 255))
      .constrained(size: 4)

    def initialize(provider)
      @provider = provider
    end

    def bounds
      Area.new(@provider.bounds)
    end

    def color_at(x, y) # rubocop:disable Naming/MethodParameterName
      red, green, blue, alpha = @provider.color_at(x, y)
      Color.new(red:, green:, blue:, alpha:)
    end

    def eql?(image_path, tolerance: nil)
      @provider.bitmap_eq(
        Types::String[image_path],
        Types::Float.optional[tolerance]
      )
    end

    def find(image_path, tolerance: nil)
      result = @provider.find(
        Types::String[image_path],
        Types::Float.optional[tolerance]
      )

      if result
        Success(Point.new(result))
      else
        Failure(:not_found)
      end
    end

    def find_color(color, tolerance: nil)
      found = @provider.find_color(
        Rgba[color],
        Types::Float.optional[tolerance]
      )

      if found
        Success(Point.new(found))
      else
        Failure(:not_found)
      end
    end

    def all(image_path, tolerance: nil)
      @provider.all(
        Types::String[image_path],
        Types::Float.optional[tolerance]
      )
        .map { |point| Point.new(point) }
    end

    def all_color(color, tolerance: nil)
      @provider.all_color(
        Rgba[color],
        Types::Float.optional[tolerance]
      )
        .map { |point| Point.new(point) }
    end

    def count(image_path, tolerance: nil)
      @provider.count(
        Types::String[image_path],
        Types::Float.optional[tolerance]
      )
    end
  end
end
