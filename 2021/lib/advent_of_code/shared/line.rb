module AdventOfCode
  module Shared
    class Line < Model
      attribute :start_point, type: Point
      attribute :end_point, type: Point

      def self.parse(str)
        start_str, end_str = str.split("->").map(&:strip)

        new(start_point: Point.parse(start_str), end_point: Point.parse(end_str))
      end

      def vertical?
        start_point.x == end_point.x
      end

      def horizontal?
        start_point.y == end_point.y
      end

      def slope
        return @slope if defined?(@slope)

        dx = end_point.x - start_point.x
        dy = end_point.y - start_point.y
        if dx.zero?
          dy /= dy.abs
        elsif dy.zero?
          dx /= dx.abs
        elsif dx.abs == dy.abs
          dx /= dx.abs
          dy /= dy.abs
        else
          raise "Sloped lines not supported yet!"
        end

        @slope =
          Point.new(
            x: dx,
            y: dy
          )
      end

      def each_point
        point = start_point
        while point != end_point
          yield point

          point += slope
        end
        yield end_point
      end
    end
  end
end
