module AdventOfCode
  class DayFive
    class Point < Shared::Model
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

    class Line < Shared::Model
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

    VentList = Shared::List.of(Line)

    def self.for(input: $stdin)
      lines = input.readlines.map(&:chomp)

      new(VentList.new(lines.map { |line| Line.parse(line) }))
    end

    attr_reader :vent_list, :vent_map

    def initialize(vent_list)
      @vent_list = vent_list
    end

    def part_one
      relevant_lines = vent_list.filter { |line| line.vertical? || line.horizontal? }
      @vent_map = VentMap.new(vent_list: relevant_lines)

      vent_map.overlapping_points.size
    end

    def part_two
      @vent_map = VentMap.new(vent_list: vent_list)

      vent_map.overlapping_points.size
    end

    class VentMap < Shared::Model
      attribute :vent_list, type: VentList

      def map_data
        @map_data ||= build_map
      end

      def overlapping_points
        map_data.each_with_object([]) do |(point, lines), matching_points|
          matching_points << point if lines.size > 1
        end
      end

      def left
        @left ||= map_data.keys.min_by(&:x).x
      end

      def right
        @right ||= map_data.keys.max_by(&:x).x
      end

      def top
        @top ||= map_data.keys.min_by(&:y).y
      end

      def bottom
        @bottom ||= map_data.keys.max_by(&:y).y
      end

      def rows
        0..bottom
      end

      def columns
        0..right
      end

      def to_s
        rows.map do |row|
          columns.map do |column|
            count_lines = map_data[Point.new(x: column, y: row)].size
            count_lines.zero? ? "." : count_lines
          end.join
        end.join("\n")
      end

      def inspect
        map_data.map do |point, lines|
          "#{point}: #{lines.size}"
        end.join("\n")
      end

    private

      def build_map
        new_map = Hash.new { |hash, key| hash[key] = [] }

        vent_list.each do |line|
          line.each_point do |point|
            new_map[point] << line
          end
        end

        new_map
      end
    end
  end
end
