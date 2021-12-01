module AdventOfCode
  class DayOne
    def self.for(input: $stdin)
      new(input.readlines.map(&:to_i))
    end

    attr_reader :depths

    def initialize(depths)
      @depths = depths
    end

    def part_one
      depths.each_cons(2).reduce(0) do |increases, (previous_depth, current_depth)|
        if previous_depth < current_depth
          increases + 1
        else
          increases
        end
      end
    end

    def part_two
      depths.each_cons(3).each_cons(2).reduce(0) do |increases, (previous_depths, current_depths)|
        if previous_depths.sum < current_depths.sum
          increases + 1
        else
          increases
        end
      end
    end
  end
end
