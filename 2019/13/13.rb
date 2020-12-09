require_relative "../shared/day"
require_relative "../shared/intcode"
require_relative "../shared/point"

module AdventOfCode
  class DayThirteen < Day
    def do_part_one
      super
      display = Display.new
      program.call(input: Intcode::StandardProgramInput.piped, output: display)
      puts display
      puts "#{display.count_tiles(Display::BLOCK_TILE)} blocks"
    end

    def do_part_two
      super
      # display = Display.new
      # program.call(input: Intcode::StandardProgramInput.piped, output: display)
    end

    def program
      @program ||= Intcode::Program.from_string(ARGF.read)
    end

    class Display
      EMPTY_TILE = 0
      WALL_TILE = 1
      BLOCK_TILE = 2
      PADDLE_TILE = 3
      BALL_TILE = 4

      attr_reader :tiles, :input_buffer

      def initialize
        @tiles = Hash.new { 0 }
        reset_input_buffer
      end

      def to_s
        bottom_right = self.bottom_right
        (0..bottom_right.y).map do |row|
          (0..bottom_right.x).map do |column|
            string_for_point(Point.new(x: column, y: row))
          end.join("")
        end.join("\n")
      end

      def string_for_point(point)
        case tiles[point]
        when EMPTY_TILE then "  "
        when WALL_TILE then "XX"
        when BLOCK_TILE then "[]"
        when PADDLE_TILE then "=="
        when BALL_TILE then "()"
        else "??"
        end
      end

      def print_int(value)
        input_buffer << value
        process_buffer
      end

      def process_buffer
        return unless input_buffer.size == 3

        x, y, tile_id = input_buffer
        reset_input_buffer

        point = Point.new(x: x, y: y)

        if tile_id.zero?
          tiles.delete(point)
        else
          tiles[point] = tile_id
        end
      end

      def reset_input_buffer
        @input_buffer = []
      end

      def bottom_right
        tiles.keys.reduce(Point.origin) do |bottom_right, point|
          Point.new(
            x: [bottom_right.x, point.x].max,
            y: [bottom_right.y, point.y].max
          )
        end
      end

      def count_tiles(target_tile_id)
        tiles.count { |_, tile_id| tile_id == target_tile_id }
      end
    end
  end

  def self.run_day_thirteen
    DayThirteen.call
  end
end

if __FILE__ == $0
  AdventOfCode.run_day_thirteen
end
