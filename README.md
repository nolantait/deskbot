# Deskbot

This is a dekstop automation library written using
[autopilot-rs](https://github.com/autopilot-rs) and
[rust extensions](https://bundler.io/blog/2023/01/31/rust-gem-skeleton.html)
for Ruby.

## Installation

Install the gem and add to the application's Gemfile by executing:

    $ bundle add deskbot

If bundler is not being used to manage dependencies, install the gem by executing:

    $ gem install deskbot

## Usage

Type something on the keyboard

```ruby
Deskbot.type_string("something", flags: [:shift], wpm: 60.0, noise: 0.0)
```

This would type `SOMETHING` at 60 wpm.

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
