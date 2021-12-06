module AdventOfCode
  class DayFive
    def self.for(input: $stdin)
      lines = input.readlines.map(&:chomp)

      new(VentList.new(lines.map { |line| Shared::Line.parse(line) }))
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

    VentList = Shared::List.of(Shared::Line)

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
