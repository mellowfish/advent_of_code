module AdventOfCode
  class DaySix
    class Fish < Shared::Model
      DAYS_TO_FIRST_BREEDING = 8
      DAYS_TO_NEXT_BREEDING = 6

      attribute :days_to_next_breeding, type: Integer

      def self.baby
        new(days_to_next_breeding: DAYS_TO_FIRST_BREEDING)
      end

      def self.breeder
        new(days_to_next_breeding: 0)
      end

      def breeding?
        days_to_next_breeding.zero?
      end

      def grow_older
        if breeding?
          with(days_to_next_breeding: DAYS_TO_NEXT_BREEDING)
        else
          with(days_to_next_breeding: days_to_next_breeding - 1)
        end
      end

      def to_s
        days_to_next_breeding.to_s
      end

      def hash
        days_to_next_breeding.hash
      end

      def ==(other)
        return false unless other.is_a?(Fish)

        days_to_next_breeding == other.days_to_next_breeding
      end

      alias_method :eql?, :==
    end

    class School
      def self.parse(line)
        numbers = line.split(",").map(&:to_i)
        fish = numbers.map { |number| Fish.new(days_to_next_breeding: number) }
        broods = fish.tally
        new(broods)
      end

      attr_reader :broods

      def initialize(broods)
        @broods = broods
      end

      def grow_older
        self.class.new(
          Hash.new { |hash, key| hash[key] = 0 }.tap do |new_broods|
            new_broods[Fish.baby] = broods[Fish.breeder] || 0

            broods.each do |fish, tally|
              new_broods[fish.grow_older] += tally
            end
          end
        )
      end

      def size
        broods.values.sum
      end
    end

    def self.for(input: $stdin)
      new(School.parse(input.readline.chomp))
    end

    attr_reader :school

    def initialize(school)
      @school = school
    end

    def part_one
      simulate(80)
      school.size
    end

    def part_two
      simulate(256)
      school.size
    end

    def simulate(days)
      days.times { @school = school.grow_older }
    end
  end
end
