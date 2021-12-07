module AdventOfCode
  class DaySeven
    PositionList = Shared::List.of(Integer)

    def self.for(input: $stdin)
      line = input.readline.chomp
      positions = line.split(",").map(&:to_i)

      new(PositionList.new(positions).sorted)
    end

    attr_reader :position_list

    def initialize(position_list)
      @position_list = position_list
    end

    def part_one
      (position_list.first..position_list.last).map do |possible_answer|
        [position_list.map { |position| (position - possible_answer).abs }.sum, possible_answer]
      end.min_by(&:first)
    end

    def part_two
      (position_list.first..position_list.last).map do |possible_answer|
        [
          position_list.map do |position|
            distance = (position - possible_answer).abs
            (distance * (distance + 1)) / 2
          end.sum,
          possible_answer
        ]
      end.min_by(&:first)
    end
  end
end
