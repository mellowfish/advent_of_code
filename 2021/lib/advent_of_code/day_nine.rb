module AdventOfCode
  class DayNine
    def self.for(input: $stdin)
      new(HeightMap.parse(input.readlines.map(&:chomp)))
    end

    attr_reader :height_map

    def initialize(height_map)
      @height_map = height_map
    end

    def part_one
      height_map.low_points.sum { |cell| cell.value + 1 }
    end

    def part_two
      height_map.basins.map(&:size).sort.last(3).reduce(&:*)
    end

    class Cell < Shared::Model
      include Comparable

      attribute :value, type: Integer
      attribute :location, type: Shared::Point

      def row
        location.y
      end

      def column
        location.x
      end

      def <=>(other)
        return nil unless other.is_a?(Cell)

        value <=> other.value
      end

      def ==(other)
        return false unless other.is_a?(Cell)

        hash == other.hash
      end
      alias_method :eql?, :==

      def hash
        [value, row, column].hash
      end

      def peak?
        value == 9
      end
    end

    class HeightMap
      def self.parse(lines)
        new(
          matrix: lines.map.with_index do |line, row|
            line.chars.map.with_index do |digit, column|
              Cell.new(value: digit.to_i, location: Shared::Point.new(x: column, y: row))
            end
          end
        )
      end

      attr_reader :matrix

      def initialize(matrix:)
        @matrix = matrix
      end

      def cell_at(row, column)
        matrix.fetch(row).fetch(column)
      end

      def basins
        @basins ||= filter_out_basin_duplicates(build_basins)
      end

      def build_basins # rubocop:disable Metrics/MethodLength
        low_points.map do |low_point|
          [].tap do |basin|
            cells_to_check = [low_point]

            until cells_to_check.empty?
              cell = cells_to_check.pop
              next if basin.include?(cell)

              basin << cell

              valid_neighbors =
                neighbors_of(cell).reject { |neighbor| basin.include?(neighbor) || neighbor.peak? || cell >= neighbor }
              cells_to_check.concat(valid_neighbors)
            end
          end
        end
      end

      def print_with_basin(basin)
        p basin
        puts
        matrix.each do |cells|
          puts cells.map { |cell| basin.include?(cell) ? "*" : cell.value }.join
        end
      end

      def filter_out_basin_duplicates(basins)
        matrix.each do |cells|
          cells.each do |cell|
            basins.each { |basin| basin.delete(cell) } if basins.count { |basin| basin.include?(cell) } > 1
          end
        end

        basins
      end

      def low_points
        @low_points ||=
          matrix.flat_map do |cells|
            cells.map do |cell|
              low_point_at?(cell) ? cell : nil
            end
          end.compact
      end

      def neighbors_of(cell) # rubocop:disable Metrics/AbcSize
        row = cell.row
        column = cell.column
        [].tap do |neighbors|
          neighbors << cell_at(row - 1, column) if row.positive?
          neighbors << cell_at(row, column + 1) if column < (width - 1)
          neighbors << cell_at(row + 1, column) if row < (height - 1)
          neighbors << cell_at(row, column - 1) if column.positive?
        end
      end

      def low_point_at?(cell)
        neighbors_of(cell).all? { |neighbor| cell < neighbor }
      end

      def height
        matrix.size
      end

      def width
        matrix.first.size
      end
    end
  end
end
