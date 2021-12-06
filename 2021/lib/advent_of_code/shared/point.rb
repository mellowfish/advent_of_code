module AdventOfCode
  module Shared
    class Point < Model
      attribute :x, type: Integer
      attribute :y, type: Integer

      def self.parse(str)
        x_str, y_str = str.split(",").map(&:strip)

        new(x: x_str.to_i, y: y_str.to_i)
      end

      def ==(other)
        return false unless other.is_a?(Point)

        x == other.x && y == other.y
      end

      def hash
        [x, y].hash
      end

      alias_method :eql?, :==

      def +(other)
        raise ArgumentError unless other.is_a?(Point)

        with(
          x: x + other.x,
          y: y + other.y
        )
      end

      def to_s
        "#{x},#{y}"
      end
    end
  end
end
