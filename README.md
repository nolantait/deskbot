# Deskbot

This is a dekstop automation library written using
[autopilot-rs](https://github.com/autopilot-rs) and
[rust extensions](https://bundler.io/blog/2023/01/31/rust-gem-skeleton.html)
for Ruby.

This library uses [dry-types](https://dry-rb.org/gems/dry-types/1.7/) so the
arguments should be well documented in
the `lib/deskbot/screen.rb` and `lib/deskbot/bitmap.rb` files

## Installation

Install the gem and add to the application's Gemfile by executing:

    $ bundle add deskbot

If bundler is not being used to manage dependencies, install the gem by executing:

    $ gem install deskbot

## Usage

To start you can require the gem in your script and initialize a screen:

```ruby
require "deskbot"

screen = Deskbot.screen
```

### Typing

Type something on the keyboard

```ruby
# Type SOMETHING at 60 words per minute
screen.type("something", flags: [:shift], wpm: 60.0, noise: 0.0)
```

You can also tap a key:

```ruby
# Tap shift + a after a 1 second delay
screen.tap_key("a", flags: [:shift], delay_ms: 1.0)
```

And even more primitively you can toggle a key:

```ruby
# Press the "a" key down
screen.toggle_key("a", down: true)
```

### Alerts

You can make alerts

```ruby
screen.alert("Hello")
```

### Mouse movement

You can teleport your mouse somewhere on the screen:

```ruby
# Teleport the mouse to coordinates x: 100, y: 100
screen.move_mouse(100, 100)
```

You can also move the mouse smoothly somewhere:

```ruby
# Move the mouse smoothly to x: 100, y: 100 over 2 seconds
screen.smooth_move_mouse(100, 100, duration: 2)
```

You can click:

```ruby
# Left click
screen.click

# Right click
screen.click(:right)
```

Or even scroll:

```ruby
# Scroll 1 click up
screen.scroll

# Scroll 5 clicks up
screen.scroll(clicks: 5)

# Scroll 5 clicks down
screen.scroll(:down, clicks: 5)
```

### Screen introspection

You can query the color of a specific pixel:

```ruby
# Get the color of a specific pixel at x: 100, y: 100
color = screen.color_at(100, 100)
```

This returns a `Deskbot::Color` object with `red`, `green`, `blue` and `alpha`
attributes.

You can query the size of the screen:

```ruby
size = screen.size
scale = screen.scale
```

The size would be a `Deskbot::Size` with `width` and `height` attributes. The
scale would simply be a float.

You can query if a point is visible on the screen:

```ruby
screen.point_visible?(100, 100)
screen.area_visible?(x: 100, y: 100, width: 400, height: 400)
```

### Bitmaps

You can capture your screen:

```ruby
# We can capture the whole screen
bitmap = screen.capture

# Or we can capture part of our screen
bitmap = screen.capture_area(x: 100, y: 100, width: 400, height: 400)
```

This returns a `Deskbot::Bitmap` which you can use to find areas that match
images you provide.

You can query the bounds of the bitmap:

```ruby
bitmap.bounds
```

Which would return a `Deskbot::Area` with `x`, `y`, `width` and `height`
attributes.

We can check for the colors of points on the bitmap:

```ruby
bitmap.color_at(100, 100)
```

Which returns a `Deskbot::Color`.

### Comparing images

You can compare images to your bitmap with a given tolerance (optional):

```ruby
bitmap.find("images/test.jpg")
bitmap.all("images/test.jpg", tolerance: 4.0)
```

`find` and `find_color` both return `Dry::Monad::Result` objects meaning you
need to unwrap their values:

```ruby
result = bitmap.find("images/test.jpg")
point = result.success
```

They are also wrapped with an optional matcher so you can use a much nicer
syntax to grab their values:

```ruby
bitmap.find("images/test.jpg") do |on|
  on.success do |point|
    # Do something with the point, no need to call point.success
  end

  on.failure do
    # Handle you error here
  end
end
```

These return `Deskbot::Point` objects with `x` and `y` attributes.

You can ask higher level questions about the image such as:

```ruby
bitmap.eql?("images/test.jpg", tolerance: 2.0)
bitmap.count("images/test.jpg")
```

You can also query for positions of colors:

```ruby
bitmap.find_color([255, 255, 255, 0], tolerance: 0.0)
bitmap.all_color([255, 255, 255, 0])
```

These will return `Deskbot::Point` objects with `x` and `y` attributes.

## Development

After checking out the repo, run `bin/setup` to install dependencies.
Then, run `rake spec` to run the tests. You can also run `bin/console` for
an interactive prompt that will allow you to experiment.

To install this gem onto your local machine, run `bundle exec rake install`.
To release a new version, update the version number in `version.rb`, and then
run `bundle exec rake release`, which will create a git tag for the version,
push git commits and the created tag, and push the `.gem` file
to [rubygems.org](https://rubygems.org).

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/nolantait/deskbot.

## License

The gem is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).
