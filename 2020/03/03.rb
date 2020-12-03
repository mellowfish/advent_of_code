require_relative "../shared/day"

module AdventOfCode
  class DayThree < Day
    def do_part_one
      super
      p trees_hit(Rational(3, 1))
    end

    def do_part_two
      super
      p [Rational(1, 1), Rational(3, 1), Rational(5, 1), Rational(7, 1), Rational(1, 2)].map { |slope| trees_hit(slope) }.reduce(1, &:*)
    end

    def trees_hit(slope)
      TobogganRide.new(slope: slope, input: input).trees_hit
    end

    def input
      @input ||= ARGF.readlines
    end

    class TobogganRide
      attr_reader :slope, :input, :current_position, :positions

      def initialize(slope:, input:)
        @slope = slope
        @input = input
        @current_position = Point.origin
        @positions = []

        traverse
      end

      def terrain
        @terrain ||= input.compact.map { |line| line.strip.chars.to_enum.lazy.cycle }
      end

      def traverse
        move until off_slope?
      end

      def move
        positions << current_position
        @current_position = current_position.move_by(slope)
      end

      def off_slope?
        current_position.y >= terrain.size
      end

      def trees_hit
        positions.count { |position| tree_at?(position) }
      end

      def tree_at?(position)
        character_at(position) == "#"
      end

      def character_at(position)
        terrain[position.y].with_index { |character, index| break character if index == position.x }
      end
    end

    class Point
      def self.origin
        new(x: 0, y: 0)
      end

      attr_reader :x, :y

      def initialize(x:, y:)
        @x = x
        @y = y
      end

      def move_by(rational)
        self.class.new(x: x + rational.numerator, y: y + rational.denominator)
      end
    end
  end

  def self.run_day_three
    DayThree.call
  end
end

if __FILE__ == $0
  AdventOfCode.run_day_three
end
