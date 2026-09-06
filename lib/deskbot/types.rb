# frozen_string_literal: true

module Deskbot
  module Types
    include Dry.Types(default: :strict)

    Flag = Types::Coercible::String.enum(
      "shift",
      "control",
      "alt",
      "meta",
      "help"
    )

    Button = Types::Coercible::String.enum(
      "left",
      "middle",
      "right"
    )

    Flags = Types::Array.of(Flag)

    Character = Types::Coercible::String.constrained(size: 1)

    ScrollDirection = Types::Coercible::String.enum(
      "up",
      "down"
    )
  end
end
