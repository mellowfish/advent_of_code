module AdventOfCode
  class DayTen
    def self.for(input: $stdin)
      lines = input.readlines.map(&:chomp)

      new(lines.map { |line| NavigationSyntaxChecker.parse(line) })
    end

    attr_reader :lines

    def initialize(lines)
      @lines = lines
    end

    def part_one
      lines.select(&:corrupted?).sum(&:invalid_score)
    end

    def part_two
      lines.reject(&:corrupted?).map(&:completion_score).sort.then { |scores| scores[scores.size / 2]}
    end

    class NavigationSyntaxChecker
      ILLEGAL_CHARACTER_POINTS = {
        ")" => 3,
        "]" => 57,
        "}" => 1197,
        ">" => 25137
      }.freeze

      COMPLETION_CHARACTER_POINTS = {
        ")" => 1,
        "]" => 2,
        "}" => 3,
        ">" => 4
      }.freeze

      BRACE_PAIRS = {
        ")" => "(",
        "]" => "[",
        "}" => "{",
        ">" => "<"
      }.freeze

      OPENING_BRACES = BRACE_PAIRS.values
      CLOSING_BRACES = BRACE_PAIRS.keys

      attr_reader :characters

      def self.parse(line)
        new(line.chars)
      end

      def initialize(characters)
        @characters = characters
        process
      end

      def valid?
        @valid
      end

      def corrupted?
        !illegal_characters.empty?
      end

      def incomplete?
        !stack.empty?
      end

      def invalid_score
        ILLEGAL_CHARACTER_POINTS.fetch(illegal_characters.first)
      end

      def completion_score
        completion_characters.reduce(0) do |total, character|
          (total * 5) + COMPLETION_CHARACTER_POINTS[character]
        end
      end

    private

      attr_reader :stack, :illegal_characters, :completion_characters

      def reset
        @valid = true
        @stack = []
        @illegal_characters = []
        @completion_characters = []
      end

      def process
        reset
        characters.each do |character|
          if valid?
            parse_character(character)
          else
            illegal_characters << character
          end
        end
        return unless valid? && incomplete?

        @completion_characters = stack.reverse.map { |character| BRACE_PAIRS.key(character) }
      end

      def parse_character(character)
        if OPENING_BRACES.include?(character)
          push_character(character)
        elsif CLOSING_BRACES.include?(character)
          pop_character(character)
        else
          unknown_character(character)
        end
      end

      def push_character(character)
        stack << character
      end

      def pop_character(character)
        expected_match = BRACE_PAIRS[character]
        if stack.last == expected_match
          stack.pop
        else
          @valid = false
          illegal_characters << character
        end
      end

      def unknown_character(character)
        raise "Unknown character: #{character}"
      end
    end
  end
end
