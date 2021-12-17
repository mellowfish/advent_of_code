module AdventOfCode
  module Shared
    class Rectangle < Shared::Model
      class << self
        def from_points(top_left:, bottom_right:)
          new(
            left: top_left.x,
            right: bottom_right.x,
            top: top_left.y,
            bottom: bottom_right.y
          )
        end
      end

      attribute :left, type: Integer
      attribute :right, type: Integer
      attribute :top, type: Integer
      attribute :bottom, type: Integer

      def include?(point)
        raise ArgumentError unless point.is_a?(Point)

        (left..right).cover?(point.x) && (bottom..top).cover?(point.y)
      end

      def left_of?(point)
        raise ArgumentError unless point.is_a?(Point)

        right < point.x
      end

      def above?(point)
        raise ArgumentError unless point.is_a?(Point)

        bottom > point.y
      end
    end
  end
end
