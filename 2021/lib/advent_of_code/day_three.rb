module AdventOfCode
  class DayThree
    def self.for(input: $stdin)
      lines = input.readlines.map(&:chomp)
      new(DiagnosticData.parse(lines))
    end

    attr_reader :diagnostic_data

    def initialize(diagnostic_data)
      @diagnostic_data = diagnostic_data
    end

    def part_one
      diagnostic_data
    end

    def part_two
      diagnostic_data
    end

    class DiagnosticData
      def self.parse(lines)
        new(lines.map(&:chars).map { |chars| chars.map(&:to_i) })
      end

      attr_reader :bit_matrix

      def initialize(bit_matrix)
        @bit_matrix = bit_matrix
      end

      def gamma_rate
        to_decimal(transposed_matrix.map { |column| most_common_bit(column, tie_breaker: 0) })
      end

      def epsilon_rate
        to_decimal(transposed_matrix.map { |column| least_common_bit(column, tie_breaker: 0) })
      end

      def oxygen_generator_rating
        possible_values = bit_matrix
        possible_transposed_matrix = bit_matrix.transpose

        bit_matrix.first.size.times do |index|
          target_bit = most_common_bit(possible_transposed_matrix[index], tie_breaker: 1)
          possible_values = possible_values.select { |bits| bits[index] == target_bit }
          possible_transposed_matrix = possible_values.transpose

          return to_decimal(possible_values.first) if possible_values.size == 1
        end

        raise "rating not found!"
      end

      def co2_scrubber_rating
        possible_values = bit_matrix
        possible_transposed_matrix = bit_matrix.transpose

        bit_matrix.first.size.times do |index|
          target_bit = least_common_bit(possible_transposed_matrix[index], tie_breaker: 0)
          possible_values = possible_values.select { |bits| bits[index] == target_bit }
          possible_transposed_matrix = possible_values.transpose

          return to_decimal(possible_values.first) if possible_values.size == 1
        end

        raise "rating not found!"
      end

    private

      def to_decimal(bits)
        bits.drop_while { |bit| bit.zero? }.join.to_i(2)
      end

      def transposed_matrix
        @transposed_matrix ||= bit_matrix.transpose
      end

      def most_common_bit(bits, tie_breaker:)
        bit_counts = bits.tally
        case bit_counts.fetch(0, 0) <=> bit_counts.fetch(1, 0)
        when -1
          1
        when 1
          0
        when 0
          tie_breaker
        else
          raise "Can't find most common bit!"
        end
      end

      def least_common_bit(bits, tie_breaker:)
        bit_counts = bits.tally
        case bit_counts.fetch(0, 0) <=> bit_counts.fetch(1, 0)
        when -1
          0
        when 1
          1
        when 0
          tie_breaker
        else
          raise "Can't find least common bit!"
        end
      end
    end
  end
end
