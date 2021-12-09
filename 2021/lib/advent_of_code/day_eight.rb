module AdventOfCode
  class DayEight
    module SevenSegmentDisplay
      #   0:      1:      2:      3:      4:
      #  aaaa    ....    aaaa    aaaa    ....
      # b    c  .    c  .    c  .    c  b    c
      # b    c  .    c  .    c  .    c  b    c
      #  ....    ....    dddd    dddd    dddd
      # e    f  .    f  e    .  .    f  .    f
      # e    f  .    f  e    .  .    f  .    f
      #  gggg    ....    gggg    gggg    ....
      #
      #   5:      6:      7:      8:      9:
      #  aaaa    aaaa    aaaa    aaaa    aaaa
      # b    .  b    .  .    c  b    c  b    c
      # b    .  b    .  .    c  b    c  b    c
      #  dddd    dddd    ....    dddd    dddd
      # .    f  e    f  .    f  e    f  .    f
      # .    f  e    f  .    f  e    f  .    f
      #  gggg    gggg    ....    gggg    gggg

      # rubocop:disable Layout/SpaceInsidePercentLiteralDelimiters, Layout/SpaceInsideArrayPercentLiteral
      SEGMENTS_FOR_DIGIT = {
        "0" => %w(a b c   e f g), # 6
        "1" => %w(    c     f  ), # 2 ######
        "2" => %w(a   c d e   g), # 5
        "3" => %w(a   c d   f g), # 5
        "4" => %w(  b c d   f  ), # 4 ######
        "5" => %w(a b   d   f g), # 5
        "6" => %w(a b   d e f g), # 6
        "7" => %w(a   c     f  ), # 3 ######
        "8" => %w(a b c d e f g), # 7 ######
        "9" => %w(a b c d   f g)  # 6
      }.freeze
      # rubocop:enable Layout/SpaceInsidePercentLiteralDelimiters, Layout/SpaceInsideArrayPercentLiteral

      UNIQUE_DIGITS = %w(1 4 7 8).freeze
      ALL_DIGITS = SEGMENTS_FOR_DIGIT.keys

      def self.digit_for_segments(segments)
        SEGMENTS_FOR_DIGIT.key(segments.sort)
      end
    end

    class Trial
      SIZE_TO_DIGIT = {
        2 => "1",
        3 => "7",
        4 => "4",
        7 => "8"
      }.freeze

      def self.parse(str)
        sample_digit_segments, inputs =
          str.split(" | ")
             .map(&:strip)
             .map do |part|
               part.split(" ").map { |segments_str| segments_str.chars.sort }
             end

        new(
          sample_digit_segments: sample_digit_segments,
          inputs: inputs
        )
      end

      attr_reader :sample_digit_segments, :inputs, :decoder

      def initialize(sample_digit_segments:, inputs:)
        @sample_digit_segments = sample_digit_segments
        @inputs = inputs
        @decoder = build_decoder
      end

      def count_simple_digits
        decoded_digits = inputs.map { |input| decode_segments_by_size(input) }.compact
        decoded_digits.count { |digit| SevenSegmentDisplay::UNIQUE_DIGITS.include?(digit) }
      end

      def decoded_input
        decoded_digits.join.to_i
      end

    private

      def decoded_digits
        inputs.map { |input| SevenSegmentDisplay.digit_for_segments(input.map { |segment| decoder[segment] }) }
      end

      def build_decoder # rubocop:disable Metrics/AbcSize, Metrics/MethodLength
        one = sample_digit_segments.find { |segments| segments.size == 2 }
        seven = sample_digit_segments.find { |segments| segments.size == 3 }
        four = sample_digit_segments.find { |segments| segments.size == 4 }
        zero_six_nine = sample_digit_segments.select { |segments| segments.size == 6 }
        eight = sample_digit_segments.find { |segments| segments.size == 7 }

        just_encoded_a = seven - one
        encoded_b_and_d = four - one
        encoded_c_d_and_e = zero_six_nine.flat_map { |segments| (eight - segments) }
        just_encoded_d = encoded_c_d_and_e & encoded_b_and_d
        just_encoded_b = encoded_b_and_d - just_encoded_d
        encoded_c_and_e = encoded_c_d_and_e - just_encoded_d
        encoded_e_and_g = eight - seven - encoded_b_and_d
        just_encoded_e = encoded_c_and_e & encoded_e_and_g
        just_encoded_c = encoded_c_and_e - just_encoded_e
        just_encoded_g = encoded_e_and_g - just_encoded_e
        just_encoded_f = one - just_encoded_c

        {
          just_encoded_a.first => "a",
          just_encoded_b.first => "b",
          just_encoded_c.first => "c",
          just_encoded_d.first => "d",
          just_encoded_e.first => "e",
          just_encoded_f.first => "f",
          just_encoded_g.first => "g"
        }
      end

      def decode_segments_by_size(segments)
        SIZE_TO_DIGIT[segments.size]
      end
    end

    TrialList = Shared::List.of(Trial)

    def self.for(input: $stdin)
      lines = input.readlines
      trial_list = TrialList.new(lines.map { |line| Trial.parse(line) })

      new(trial_list)
    end

    attr_reader :trial_list

    def initialize(trial_list)
      @trial_list = trial_list
    end

    def part_one
      trial_list.sum_by(&:count_simple_digits)
    end

    def part_two
      trial_list.sum_by(&:decoded_input)
    end
  end
end
