module AdventOfCode
  class DaySix
    class Fish < Shared::Model
      attribute :age, type: Integer

      def self.baby
        new(age: 8)
      end

      def self.breeder
        new(age: 0)
      end

      def reproducing?
        age.zero?
      end

      def grow_older
        if age.zero?
          with(age: 6)
        else
          with(age: age - 1)
        end
      end

      def to_s
        age.to_s
      end

      def hash
        age.hash
      end

      def ==(other)
        return false unless other.is_a?(Fish)

        age == other.age
      end

      alias_method :eql?, :==
    end

    class School
      def self.parse(line)
        ages = line.split(",").map(&:to_i)
        fish = ages.map { |age| Fish.new(age: age) }
        age_groups = fish.tally
        new(age_groups)
      end

      attr_reader :age_groups

      def initialize(age_groups)
        @age_groups = age_groups
      end

      def grow_older
        self.class.new(
          Hash.new { |hash, key| hash[key] = 0 }.tap do |new_age_groups|
            new_age_groups[Fish.baby] = age_groups[Fish.breeder] || 0

            age_groups.each do |fish, tally|
              new_age_groups[fish.grow_older] += tally
            end
          end
        )
      end

      def size
        age_groups.values.sum
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
