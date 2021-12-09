module AdventOfCode
  class DaySeven
    class Crab < Shared::Model
      include Comparable

      attribute :position, type: Integer

      def linear_fuel_to(other)
        (position - other.position).abs
      end

      def triangular_fuel_to(other)
        distance = linear_fuel_to(other)

        (distance * (distance + 1)) / 2
      end

      def <=>(other)
        return nil unless other.is_a?(Crab)

        position <=> other.position
      end
    end

    CrabList = Shared::List.of(Crab)

    def self.for(input: $stdin)
      line = input.readline.chomp
      positions = line.split(",").map(&:to_i)
      new(CrabList.new(positions.map { |position| Crab.new(position: position) }).sorted)
    end

    attr_reader :crab_fleet

    def initialize(crab_fleet)
      @crab_fleet = crab_fleet
    end

    def part_one
      possible_crab_positions.map(type: Shared::List.of(Array)) do |possible_lead_crab|
        [
          crab_fleet.map(type: Shared::List.of(Integer)) { |crab| crab.linear_fuel_to(possible_lead_crab) }.sum,
          possible_lead_crab.position
        ]
      end.min_by(&:first)
    end

    def part_two
      possible_crab_positions.map(type: Shared::List.of(Array)) do |possible_lead_crab|
        [
          crab_fleet.map(type: Shared::List.of(Integer)) { |crab| crab.triangular_fuel_to(possible_lead_crab) }.sum,
          possible_lead_crab.position
        ]
      end.min_by(&:first)
    end

    def possible_crab_positions
      CrabList.new(
        (crab_fleet.first.position..crab_fleet.last.position).map { |position| Crab.new(position: position) }
      )
    end
  end
end
