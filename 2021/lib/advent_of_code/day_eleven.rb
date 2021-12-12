module AdventOfCode
  class DayEleven
    def self.for(input: $stdin)
      new(FlashingOctopi.parse(input.readlines.map(&:chomp)))
    end

    attr_reader :pod

    def initialize(pod)
      @pod = pod
    end

    def part_one
      100.times.reduce(pod) { |current_pod| current_pod.age }.total_flashes
    end

    def part_two
      current_pod = pod
      step = 0
      loop do
        current_pod = current_pod.age
        step += 1
        break if current_pod.max_brightness?
      end
      step
    end

    class FlashingOctopi < Shared::Model
      def self.parse(lines)
        new(matrix: lines.map { |line| line.chars.map(&:to_i) }, total_flashes: 0)
      end

      attribute :matrix, type: Array
      attribute :total_flashes, type: Integer

      def print
        puts matrix.map(&:join)
        puts
      end

      def age(*)
        next_total_flashes = total_flashes
        flashes = []
        flashes_to_process = []
        next_matrix = matrix.map(&:dup)

        next_matrix.each_with_index do |octopi, row|
          octopi.each_with_index do |octopus, column|
            flashes_to_process << [row, column] if octopus == 9
            next_matrix[row][column] += 1
          end
        end

        until flashes_to_process.empty?
          flash = flashes_to_process.pop
          next if flashes.include?(flash)

          flashes << flash
          neighbors_of(*flash).each do |neighbor_row, neighbor_column|
            flashes_to_process << [neighbor_row, neighbor_column] if next_matrix[neighbor_row][neighbor_column] == 9
            next_matrix[neighbor_row][neighbor_column] += 1
          end
        end

        flashes.each { |row, column| next_matrix[row][column] = 0 }

        with(
          matrix: next_matrix,
          total_flashes: next_total_flashes + flashes.size
        )
      end

      def neighbors_of(row, column)
        [].tap do |neighbors|
          if row.positive?
            neighbors << [row - 1, column - 1] if column.positive?
            neighbors << [row - 1, column]
            neighbors << [row - 1, column + 1] if column < (width - 1)
          end

          neighbors << [row, column - 1] if column.positive?
          neighbors << [row, column + 1] if column < (width - 1)

          if row < (height - 1)
            neighbors << [row + 1, column - 1] if column.positive?
            neighbors << [row + 1, column]
            neighbors << [row + 1, column + 1] if column < (width - 1)
          end
        end
      end

      def height
        matrix.size
      end

      def width
        matrix.first.size
      end

      def max_brightness?
        matrix.all? { |octopi| octopi.all?(&:zero?) }
      end
    end
  end
end
