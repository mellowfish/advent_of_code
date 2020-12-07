require_relative "../shared/day"

module AdventOfCode
  class DayFive < Day
    def do_part_one
      super
      p boarding_passes.last.seat_id
    end

    def do_part_two
      super
      p boarding_passes.each_cons(2).find { |left, right| left.seat_id != right.seat_id - 1 }.first.seat_id + 1
    end

    def boarding_passes
      @boarding_passes ||= ARGF.readlines.compact.map { |line| BoardingPass.new(line) }.sort_by(&:seat_id)
    end

    class BoardingPass
      attr_reader :raw_data

      def initialize(raw_data)
        @raw_data = raw_data
      end

      def to_i
        @integer_value ||= raw_data.tr("BFRL", "1010").to_i(2)
      end

      def row
        to_i >> 3
      end

      def column
        to_i & 0b111
      end

      def seat_id
        (row * 8) + column
      end
    end
  end

  def self.run_day_five
    DayFive.call
  end
end

if __FILE__ == $0
  AdventOfCode.run_day_five
end
