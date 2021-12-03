module AdventOfCode
  module Domain
    class DiagnosticData
      def self.parse(lines)
        new(Shared::BitMatrix.new(lines.map(&:chars).map { |chars| chars.map(&:to_i) }))
      end

      attr_reader :bit_matrix

      def initialize(bit_matrix)
        @bit_matrix = bit_matrix
      end

      def gamma_rate
        bit_matrix.most_common_bits.to_decimal
      end

      def epsilon_rate
        bit_matrix.least_common_bits.to_decimal
      end

      def oxygen_generator_rating
        possible_values = bit_matrix

        bit_matrix.width.times do |index|
          target_bit = possible_values.column_bit_list(index).most_common_bit(tie_breaker: 1)
          possible_values = possible_values.filter { |bit_list| bit_list.at(index) == target_bit }

          return possible_values.row_bit_list(0).to_decimal if possible_values.size == 1
        end

        raise "rating not found!"
      end

      def co2_scrubber_rating
        possible_values = bit_matrix

        bit_matrix.width.times do |index|
          target_bit = possible_values.column_bit_list(index).least_common_bit(tie_breaker: 0)
          possible_values = possible_values.filter { |bit_list| bit_list.at(index) == target_bit }

          return possible_values.row_bit_list(0).to_decimal if possible_values.size == 1
        end

        raise "rating not found!"
      end
    end
  end
end
