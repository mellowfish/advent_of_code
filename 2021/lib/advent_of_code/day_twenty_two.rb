class Range
  def intersect_with(two)
    one = self
    return { intersection: one } if one == two

    one, two = [two, one] if two.begin < one.begin || (one.begin == two.begin && one.end < two.end)

    return {} if one.end < two.begin

    points = [one.begin, one.end, two.begin, two.end].sort
    { intersection: (points[1]..points[2]) }.tap do |parts|
      parts[:leading] = (points[0]..(points[1] - 1)) if points[0] < points[1]
      parts[:trailing] = ((points[2] + 1)..points[3]) if points[2] < points[3]
    end
  end
end

module AdventOfCode
  class DayTwentyTwo
    def self.for(input: $stdin)
      new(ReactorCore.parse_instructions(input.readlines(chomp: true)))
    end

    attr_reader :core

    def initialize(core)
      @core = core
    end

    def part_one
      core.boot.active_cube_count
    end

    def part_two
      raise "Not yet!"
    end

    class ReactorCore < Shared::Model
      class Instruction < Shared::Model
        def self.parse(str)
          action_str, ranges_str = str.split(" ")
          action = action_str == "on"
          x_range, y_range, z_range =
            ranges_str.split(",").map do |range_str|
              min, max = range_str[2..].split("..").map(&:to_i)
              min..max
            end

          new(action: action, x_range: x_range, y_range: y_range, z_range: z_range)
        end

        attribute :action, type: :boolean

        attribute :x_range, type: Range
        attribute :y_range, type: Range
        attribute :z_range, type: Range

        def in_scope?
          return @in_scope if defined?(@in_scope)

          @in_scope = valid_range?(x_range) && valid_range?(y_range) && valid_range?(z_range)
        end

        def cover?(x:, y:, z:) # rubocop:disable Naming/MethodParameterName
          x_range.cover?(x) && y_range.cover?(y) && z_range.cover?(z)
        end

        def evaluate_for_cube(current_value:, x:, y:, z:) # rubocop:disable Naming/MethodParameterName
          return current_value unless cover?(x: x, y: y, z: z)

          action
        end

        def collide(other)

        end

      private

        def valid_range?(range)
          range.end < 51 && range.begin > -51
        end
      end

      def self.parse_instructions(lines)
        new(startup_instructions: lines.map { |line| Instruction.parse(line) })
      end

      attribute :startup_instructions, type: Array, default: []
      attribute :active_cube_count, type: Integer, default: 0

      def boot
        new_active_cube_count = 0
        (-50..50).each do |x|
          (-50..50).each do |y|
            (-50..50).each do |z|
              valid_instructions.reduce(false) do |current_value, instruction|
                instruction.evaluate_for_cube(current_value: current_value, x: x, y: y, z: z)
              end.tap { |active| new_active_cube_count += 1 if active }
            end
          end
        end

        with(active_cube_count: new_active_cube_count)
      end

      def valid_instructions
        @valid_instructions ||= startup_instructions.select(&:in_scope?)
      end
    end
  end
end
