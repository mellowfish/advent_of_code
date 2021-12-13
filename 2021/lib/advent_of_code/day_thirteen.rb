module AdventOfCode
  class DayThirteen
    def self.for(input: $stdin)
      new(Instructions.parse(input.readlines.map(&:chomp)))
    end

    attr_reader :instructions

    def initialize(instructions)
      @instructions = instructions
    end

    def part_one
      instructions.fold.total_dots
    end

    def part_two
      current_fold = instructions
      current_fold = current_fold.fold until current_fold.finished?
      StringIO.new.tap { |output| current_fold.print(output) }.string
    end

    class Instructions < Shared::Model
      def self.parse(lines)
        split_index = lines.index { |line| line.strip.empty? }
        coordinate_lines = lines[0...split_index]
        fold_lines = lines[(split_index + 1)..-1]

        coordinates = coordinate_lines.map { |line| line.split(",").map(&:to_i) }
        dots = coordinates.each_with_object(build_new_dots_hash) do |(column, row), hash|
          hash[row][column] = 1
        end

        folds = fold_lines.map do |line|
          axis, value_str = line.delete_prefix("fold along ").split("=")

          [axis, value_str.to_i]
        end

        new(dots: dots, folds: folds)
      end

      def self.build_new_dots_hash
        Hash.new { |hash, row| hash[row] = Hash.new { |row_hash, column| row_hash[column] = 0 } }
      end

      attribute :dots, type: Hash
      attribute :folds, type: Array

      def finished?
        folds.empty?
      end

      def fold
        new_dots = self.class.build_new_dots_hash
        axis, value = folds.first

        dots.each do |row, row_hash|
          row_hash.each do |column, count|
            if axis == "x"
              if column < value
                new_dots[row][column] += count
              else
                new_dots[row][(2 * value) - column] += count
              end
            else
              if row < value
                new_dots[row][column] += count
              else
                new_dots[(2 * value) - row][column] += count
              end
            end
          end
        end
        with(dots: new_dots, folds: folds[1..-1])
      end

      def total_dots
        dots.values.flat_map(&:values).count(&:positive?)
      end

      def print(output = $stdout)
        axis, value = folds.first
        height = dots.keys.max
        width = dots.values.flat_map(&:keys).max

        (0..height).each do |row|
          line =
            (0..width).map do |column|
              if axis == "x" && column == value
                "|"
              elsif axis == "y" && row == value
                "-"
              elsif dots[row][column].zero?
                "."
              else
                "#"
              end
            end.join

          output.puts(line)
        end
      end
    end
  end
end
