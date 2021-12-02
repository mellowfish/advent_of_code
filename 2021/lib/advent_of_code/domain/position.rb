module AdventOfCode
  module Domain
    class Position < Shared::Model
      attribute :depth, type: Integer
      attribute :horizontal, type: Integer

      def self.origin
        new(depth: 0, horizontal: 0)
      end

      def forward(distance)
        with(horizontal: self.horizontal + distance)
      end

      def down(distance)
        with(depth: self.depth + distance)
      end

      def up(distance)
        new_depth = self.depth - distance

        raise(ArgumentError, "This tub can't fly, captain!") if new_depth.negative?

        with(depth: new_depth)
      end
    end
  end
end
