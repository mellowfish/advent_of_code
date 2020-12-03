require_relative "../shared/day"

module AdventOfCode
  module_function

  class DayOne < Day
    def do_part_one
      super
      p target_pair.product
    end

    def do_part_two
      super
      p target_triplet.product
    end

    def target_pair
      find_first_example(from_source: all_pairs, where: ->(pair) { pair.sum == 2020 })
    end

    def target_triplet
      find_first_example(from_source: all_triplets, where: ->(triplet) { triplet.sum == 2020 })
    end

    def find_first_example(from_source:, where:)
      from_source.find { |item| where.call(item) }
    end

    def all_pairs
      PairIterator.for(sorted_expenses)
    end

    def all_triplets
      TripletIterator.for(sorted_expenses)
    end

    def sorted_expenses
      @sorted_expenses ||= expenses.sort
    end

    def expenses
      @expenses ||= ARGF.readlines.map(&:to_i).compact
    end

    class Pair
      attr_reader :a, :b

      def initialize(a, b)
        @a = a
        @b = b
      end

      def sum
        a + b
      end

      def product
        a * b
      end
    end

    class Triplet
      attr_reader :a, :b, :c

      def initialize(a, b, c)
        @a = a
        @b = b
        @c = c
      end

      def sum
        a + b + c
      end

      def product
        a * b * c
      end
    end

    class PairIterator
      include Enumerable

      def self.for(series)
        new(series)
      end

      attr_reader :series

      def initialize(series)
        @series = series
      end

      def each
        series.each do |a|
          series.each do |b|
            next if b <= a

            yield Pair.new(a, b)
          end
        end

        self
      end
    end

    class TripletIterator
      include Enumerable

      def self.for(series)
        new(series)
      end

      attr_reader :series

      def initialize(series)
        @series = series
      end

      def each
        series.each do |a|
          series.each do |b|
            next if b <= a

            series.each do |c|
              next if c <= b || c <= a

              yield Triplet.new(a, b, c)
            end
          end
        end

        self
      end
    end
  end

  def run_day_one
    DayOne.call
  end
end

if __FILE__ == $0
  AdventOfCode.run_day_one
end
