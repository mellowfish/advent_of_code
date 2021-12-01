module AdventOfCode
  class DayOne
    def self.for(input: $stdin)
      lines = input.readlines
      depths = lines.map { |line| Depth.parse(line) }
      new(DepthSeries.new(depths))
    end

    attr_reader :depth_series

    def initialize(depth_series)
      @depth_series = depth_series
    end

    def part_one
      depth_series.each_pair.count(&:increasing?)
    end

    def part_two
      depth_sliding_windows.each_pair.count(&:increasing?)
    end

    def depth_sliding_windows
      Shared::List.new(depth_series.sliding_window(size: 3))
    end

    class Depth < Shared::NumberValue; end

    DepthSeries = Shared::List.of(Depth) do
      include Comparable

      def <=>(other)
        return nil unless instance_of?(other.class)

        sum <=> other.sum
      end

      def sum
        items.sum(Depth.zero)
      end
    end
  end
end
