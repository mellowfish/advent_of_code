module AdventOfCode
  class DayFourteen
    def self.for(input: $stdin)
      new(Polymer.parse(input.readlines(chomp: true)))
    end

    attr_reader :polymer

    def initialize(polymer)
      @polymer = polymer
    end

    def part_one
      10.times.reduce(polymer, &:grow).score
    end

    def part_two
      40.times.reduce(polymer, &:grow).score
    end

    class Polymer < Shared::Model
      def self.parse(lines)
        initial_polymer = lines.shift.chars
        lines.shift # blank
        insertion_rules =
          lines.each_with_object({}) do |line, hash|
            pair_str, insertion = line.split(" -> ")
            hash[pair_str.chars] = insertion
          end
        pairs = initial_polymer.each_cons(2).tally

        new(initial_polymer: initial_polymer, pairs: pairs, insertion_rules: insertion_rules)
      end

      attribute :initial_polymer, type: Array
      attribute :pairs, type: Hash
      attribute :insertion_rules, type: Hash

      def grow(*)
        new_pairs = Hash.new { |hash, key| hash[key] = 0 }
        pairs.each do |pair, count|
          new_char = insertion_rules[pair]
          new_pairs[[pair.first, new_char]] += count
          new_pairs[[new_char, pair.last]] += count
        end
        with(pairs: new_pairs)
      end

      def score
        most_common_element_frequency - least_common_element_frequency
      end

      def tally
        @tally ||= build_tally
      end

      def most_common_element_frequency
        tally.values.max
      end

      def least_common_element_frequency
        tally.values.min
      end

    private

      def build_tally
        duplicated_tally_from_pairs.transform_values { |value| (value / 2) + (value % 2) }
      end

      def duplicated_tally_from_pairs
        pairs.each_with_object(Hash.new { |hash, key| hash[key] = 0 }) do |(pair, count), hash|
          hash[pair.first] += count
          hash[pair.last] += count
        end
      end
    end
  end
end
