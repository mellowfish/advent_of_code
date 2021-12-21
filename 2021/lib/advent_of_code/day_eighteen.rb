module AdventOfCode
  class DayEighteen
    def self.for(input: $stdin)
      new(input.readlines)
    end

    def initialize(lines)
      @lines = lines
    end

    def part_one
      raise "Not yet!"
    end

    def part_two
      raise "Not yet!"
    end

    class SnailFishNumber < Shared::Model
      def self.parse(line)
        stack = []
        line.chars.each do |char|
          case char
          when "["
            stack << char
          when /[0-9]/
            stack << SnailFishLiteral.new(number: char.to_i)
          when ","
            nil
          when "]"
            right = stack.pop
            left = stack.pop
            possible_brace = stack.pop
            raise "wat" if possible_brace != "["

            stack << SnailFishPair.new(left: left, right: right)
          end
        end

        raise "wat" if stack.size != 1

        stack.first
      end

      attribute :parent, type: SnailFishNumber, default: nil

      def depth
        return 0 if root?

        parent.depth + 1
      end

      def root?
        parent.nil?
      end

      def root
        root? ? self : parent.root
      end
    end

    class SnailFishLiteral < SnailFishNumber
      attribute :number, type: Integer

      def self.zero
        new(number: 0)
      end

      def to_s
        number.to_s
      end

      def pair?
        false
      end

      def replace(old_element, new_element)
        if old_element == self
          new_element
        else
          self
        end
      end
    end

    class SnailFishPair < SnailFishNumber
      attribute :left, type: SnailFishNumber
      attribute :right, type: SnailFishNumber

      def initialize(left:, right:, parent: nil)
        super(left: left.with(parent: self), right: right.with(parent: self), parent: parent)
      end

      def pair?
        true
      end

      def can_explode?
        depth > 3
      end

      def replace(old_element, new_element)
        if old_element == self
          new_element
        else
          with(
            left: left.replace(old_element, new_element),
            right: right.replace(old_element, new_element)
          )
        end
      end

      def literals_with_index(current = 0)
        [].tap do |literals|
          if left.pair?
            literals.concat(left.literals_with_index(current))
            current = literals.last.last
          else
            literals << [left, current]
            current += 1
          end

          if right.pair?
            literals.concat(right.literals_with_index(current))
            current = literals.last.last
          else
            literals << [right, current]
            current += 1
          end
        end
      end

      def pairs
        [].tap do |pairs|
          pairs.concat(left.pairs) if left.pair?
          pairs << self unless root?
          pairs.concat(right.pairs) if right.pair?
        end
      end

      def explode
        raise "Only root pairs can explode!" unless root?

        pairs.each do |pair|
          next unless pair.can_explode?

          zero = SnailFishLiteral.zero
          new_number, replaced_index = root.replace(pair, zero)
          literals = new_number.literals_with_index
          if replaced_index.positive?
            left_literal = literals[zero_index - 1].first
            new_number = new_number.replace(left_literal, left_literal + pair.left)
          end

          if replaced_index < (literals.size - 1)
            right_literal = literals[zero_index + 1].first
            new_number = new_number.replace(right_literal, right_literal + pair.right)
          end

          return new_number
        end

        self
      end

      def to_s
        "[#{left},#{right}]"
      end
    end
  end
end
