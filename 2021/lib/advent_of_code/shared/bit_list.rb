module AdventOfCode
  module Shared
    class BitList
      attr_reader :bits

      def initialize(bits)
        @bits = bits

        validate!
      end

      def least_common_bit(tie_breaker: nil)
        if total_zeros < total_ones
          0
        elsif total_ones < total_zeros
          1
        else
          raise "Couldn't find least common bit, and no tie breaker specified" unless tie_breaker

          tie_breaker
        end
      end

      def most_common_bit(tie_breaker: nil)
        if total_zeros > total_ones
          0
        elsif total_ones > total_zeros
          1
        else
          raise "Couldn't find most common bit, and no tie breaker specified" unless tie_breaker

          tie_breaker
        end
      end

      def to_decimal
        bits.drop_while(&:zero?).join.to_i(2)
      end

      def at(index)
        bits[index]
      end

    private

      def validate!
        bits.each { |bit| raise(ArgumentError, "invalid bit value: #{bit}") unless [0, 1].include?(bit) }
      end

      def bit_tally
        @bit_tally ||= bits.tally
      end

      def total_zeros
        bit_tally.fetch(0, 0)
      end

      def total_ones
        bit_tally.fetch(1, 0)
      end
    end
  end
end
