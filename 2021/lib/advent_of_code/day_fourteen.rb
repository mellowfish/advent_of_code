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
      stack_solution(10)
    end

    def part_two
      stack_solution(15) # 22 takes ~15 seconds, doubles every new depth
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

        new(polymer: initial_polymer, insertion_rules: insertion_rules)
      end

      attribute :polymer, type: Array
      attribute :insertion_rules, type: Hash

      def grow(*)
        with(polymer: replacement_for(polymer))
      end

      def replacement_for(segment)
        debug = false
        if insertion_rules.key?(segment)
          puts "Simple replacement from rules: #{segment.join} => #{insertion_rules[segment].join}" if debug
          return insertion_rules[segment]
        end

        segment_size = segment.size
        if segment_size < 4
          new_segment = [segment.first]
          segment.each_cons(2) do |pair|
            replacement_triplet = insertion_rules[pair]
            new_segment << replacement_triplet[1]
            new_segment << replacement_triplet[2]
          end
          puts "Pair expansion: #{segment.join} => #{new_segment.join}" if debug
          insertion_rules[segment] = new_segment
          return new_segment
        end

        chunk_size = segment_size / 2
        chunk_size += 1 if chunk_size * 2 < segment_size

        first_half, second_half = segment.each_slice(chunk_size).to_a
        puts "About to recurse on half: #{first_half.join}" if debug
        replaced_first_half = replacement_for(first_half)
        puts "About to recurse on half: #{second_half.join}" if debug
        replaced_second_half = replacement_for(second_half)
        new_segment = replaced_first_half + [replacement_for([first_half.last, second_half.first])[1]] + replaced_second_half
        puts "Recursive solve: #{segment.join} => #{new_segment.join}" if debug
        insertion_rules[segment] = new_segment
        new_segment
      end

      def score
        most_common_element_frequency - least_common_element_frequency
      end

      def tally
        @tally ||= polymer.tally
      end

      def most_common_element_frequency
        tally.values.max
      end

      def least_common_element_frequency
        tally.values.min
      end
    end

    def stack_solution(depth)
      debug = false

      tally = Hash.new { |hash, key| hash[key] = 0 }
      final = []
      rules = polymer.insertion_rules
      stack = polymer.polymer.map { |char| [char, depth] }.reverse
      p stack if debug
      until stack.empty?
        first_char, first_depth = stack.pop

        if stack.empty?
          final << first_char if debug
          tally[first_char] += 1
          next
        end

        if first_depth.zero?
          final << first_char if debug
          p final.join if debug
          tally[first_char] += 1
          next
        end

        last_char, last_depth = stack.pop
        middle_char = rules[[first_char, last_char]]
        stack << [last_char, last_depth]
        if first_depth == 1
          final << first_char if debug
          final << middle_char if debug
          tally[middle_char] += 1
          p final.join if debug
          tally[first_char] += 1
        else
          stack << [middle_char, first_depth - 1]
          stack << [first_char, first_depth - 1]
        end

        # puts stack.map(&:join).join
        p stack if debug
      end
      p final.join if debug
      p tally
      tally.values.max - tally.values.min
    end

    def big_array_solution(depth)
      debug = false

      input = polymer.polymer
      rules = polymer.insertion_rules
      size = 2**depth * (input.size - 1) + 1
      final = Array.new(size)

      input.each_with_index do |char, index|
        final[index * (size / (input.size - 1))] = char
      end

      previous_layer_size = input.size
      (1..depth).each do |layer|
        puts "Layer: #{depth} (#{Time.now})"
        puts final.join if debug
        layer_size = 2**layer * (input.size - 1) + 1
        num_to_insert = previous_layer_size - 1
        width = size / num_to_insert
        p [layer_size, num_to_insert, width] if debug
        num_to_insert.times do |index|
          left = index * width
          right = (index + 1) * width
          p [left, right] if debug
          p [final[left], final[right]] if debug
          final[index * width + (width / 2)] = rules[[final[left], final[right]]]
        end
        previous_layer_size = layer_size
      end
      puts final.join if debug

      polymer.with(polymer: final).score
    end
  end
end
