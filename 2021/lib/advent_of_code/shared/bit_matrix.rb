module AdventOfCode
  module Shared
    class BitMatrix
      def initialize(matrix, trusted: false)
        @matrix = matrix

        validate! unless trusted
      end

      def rotate
        with(matrix.transpose)
      end

      def most_common_bits
        BitList.new(rotate.map(&:most_common_bit))
      end

      def least_common_bits
        BitList.new(rotate.map(&:least_common_bit))
      end

      def size
        matrix.size
      end
      alias_method :length, :size

      def width
        matrix.first.size
      end

      def row_bit_list(index)
        BitList.new(matrix[index])
      end

      def column_bit_list(index)
        rotate.row_bit_list(index)
      end

      def filter
        with(matrix.select { |bits| yield BitList.new(bits) })
      end

    protected

      def map
        matrix.map { |row| yield BitList.new(row) }
      end

    private

      attr_reader :matrix

      def validate!
        matrix.each do |row|
          row.each do |bit|
            raise(ArgumentError, "invalid bit value: #{bit}") unless [0, 1].include?(bit)
          end
        end
      end

      def with(new_matrix)
        self.class.new(new_matrix, trusted: true)
      end
    end
  end
end
