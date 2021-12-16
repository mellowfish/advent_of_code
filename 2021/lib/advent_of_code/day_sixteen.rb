module AdventOfCode
  class DaySixteen
    def self.for(input: $stdin)
      new(Packets.parse(input.readline))
    end

    attr_reader :packet

    def initialize(packet)
      @packet = packet
    end

    def part_one
      packet.version_sum
    end

    def part_two
      packet.value
    end

    module Bits
      HEX_DIGIT_TO_BITS = {
        "0" => [0, 0, 0, 0],
        "1" => [0, 0, 0, 1],
        "2" => [0, 0, 1, 0],
        "3" => [0, 0, 1, 1],
        "4" => [0, 1, 0, 0],
        "5" => [0, 1, 0, 1],
        "6" => [0, 1, 1, 0],
        "7" => [0, 1, 1, 1],
        "8" => [1, 0, 0, 0],
        "9" => [1, 0, 0, 1],
        "A" => [1, 0, 1, 0],
        "B" => [1, 0, 1, 1],
        "C" => [1, 1, 0, 0],
        "D" => [1, 1, 0, 1],
        "E" => [1, 1, 1, 0],
        "F" => [1, 1, 1, 1]
      }.freeze

      class Stream
        def self.parse(hex_string)
          new(Bits.hex_string_to_bits(hex_string))
        end

        attr_reader :bits

        def initialize(bits = [])
          @bits = Array(bits)
        end

        def size
          bits.size
        end

        def read_bit
          raise IndexError if empty_except_padding?

          bits.shift
        end

        def read_bits(size)
          raise IndexError if empty_except_padding? || size > bits.size

          Array(bits.shift(size))
        end

        def read_number(size)
          Bits.bits_to_number(read_bits(size))
        end

        def empty_except_padding?
          bits.empty? || bits.all?(&:zero?)
        end

        def write_bits(new_bits)
          bits.concat(new_bits)
        end
      end

      class << self
        def hex_string_to_bits(hex_string)
          hex_string.chars.flat_map { |hex_digit| HEX_DIGIT_TO_BITS[hex_digit] }
        end

        def bits_to_number(bits)
          bits.join.to_i(2)
        end
      end
    end

    module Packets
      class << self
        def parse(hex_string)
          bit_stream = Bits::Stream.parse(hex_string)
          read_packet(bit_stream)
        end

        def read_packet(bit_stream)
          version = bit_stream.read_number(3)
          type_id = bit_stream.read_number(3)

          if type_id == 4
            NumberLiteral.read_packet(version: version, type_id: type_id, bit_stream: bit_stream)
          else
            Operator.read_packet(version: version, type_id: type_id, bit_stream: bit_stream)
          end
        end
      end

      class Operator < Shared::Model
        class << self
          def read_packet(version:, type_id:, bit_stream:)
            sub_packets = []
            length_type_id = bit_stream.read_bit
            if length_type_id.zero?
              total_body_size = bit_stream.read_number(15)
              expected_remaining_bits = bit_stream.size - total_body_size
              sub_packets << Packets.read_packet(bit_stream) while bit_stream.size > expected_remaining_bits
            else
              sub_packet_count = bit_stream.read_number(11)
              sub_packet_count.times do
                sub_packets << Packets.read_packet(bit_stream)
              end
            end

            new(version: version, type_id: type_id, sub_packets: sub_packets)
          end
        end

        attribute :version, type: Integer
        attribute :type_id, type: Integer
        attribute :sub_packets, type: Array

        def version_sum
          version + sub_packets.sum(&:version_sum)
        end

        def value
          case type_id
          when 0
            sub_packets.map(&:value).reduce(:+)
          when 1
            sub_packets.map(&:value).reduce(:*)
          when 2
            sub_packets.map(&:value).min
          when 3
            sub_packets.map(&:value).max
          when 5
            sub_packets.first.value > sub_packets.last.value ? 1 : 0
          when 6
            sub_packets.first.value < sub_packets.last.value ? 1 : 0
          when 7
            sub_packets.first.value == sub_packets.last.value ? 1 : 0
          else
            raise "Unknown packet type: #{type_id}"
          end
        end
      end

      class NumberLiteral < Shared::Model
        def self.read_packet(version:, type_id:, bit_stream:)
          value_bit_stream = Bits::Stream.new
          loop do
            sentinel_bit = bit_stream.read_bit
            value_bit_stream.write_bits(bit_stream.read_bits(4))
            break if sentinel_bit.zero?
          end

          value = value_bit_stream.read_number(value_bit_stream.size)
          new(version: version, type_id: type_id, value: value)
        end

        attribute :version, type: Integer
        attribute :type_id, type: Integer
        attribute :value, type: Integer

        alias_method :version_sum, :version
      end
    end
  end
end
