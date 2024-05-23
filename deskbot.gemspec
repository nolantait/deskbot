# frozen_string_literal: true

require_relative "lib/deskbot/version"

Gem::Specification.new do |spec|
  spec.name = "deskbot"
  spec.version = Deskbot::VERSION
  spec.authors = ["Nolan J Tait"]
  spec.email = ["nolanjtait@gmail.com"]

  spec.summary = "Ruby dekstop automation"
  spec.description = "Ruby desktop automation"
  spec.homepage = "https://github.com/nolantait/desktbot"
  spec.license = "MIT"
  spec.required_ruby_version = ">= 3.0.0"
  spec.required_rubygems_version = ">= 3.3.11"

  spec.metadata["homepage_uri"] = spec.homepage
  spec.metadata["source_code_uri"] = spec.homepage
  spec.metadata["changelog_uri"] = spec.homepage

  # Specify which files should be added to the gem when it is released.
  # The `git ls-files -z` loads the files in the RubyGem that have been added into git.
  gemspec = File.basename(__FILE__)
  spec.files = IO.popen(
    %w[git ls-files -z],
    chdir: __dir__,
    err: IO::NULL
  ) do |ls|
    ls.readlines(
      "\x0",
      chomp: true
    ).reject do |f|
      (f == gemspec) ||
        f.start_with?(
          *%w[
            bin/
            test/
            spec/
            features/
            .git
            appveyor
            Gemfile
          ]
        )
    end
  end
  spec.bindir = "exe"
  spec.executables = spec.files.grep(%r{\Aexe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]
  spec.extensions = ["ext/deskbot/Cargo.toml"]

  # Uncomment to register a new dependency of your gem
  spec.add_dependency "dry-struct"
  spec.add_dependency "dry-types"
  spec.metadata["rubygems_mfa_required"] = "true"
end
