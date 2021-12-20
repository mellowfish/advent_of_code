module AdventOfCode
  class DayTwenty
    def self.for(input: $stdin)
      new(ImageEnhancer.parse(input.readlines(chomp: true)))
    end

    attr_reader :image_enhancer

    def initialize(image_enhancer)
      @image_enhancer = image_enhancer
    end

    def part_one
      2.times.reduce(image_enhancer, &:enhance).lit_pixels
    end

    def part_two
      50.times.reduce(image_enhancer, &:enhance).lit_pixels
    end

    class ImageEnhancer < Shared::Model
      BUFFER = 1

      def self.parse(lines)
        algorithm = lines.shift.chars.map { |char| char == "#" ? 1 : 0 }
        lines.shift

        new(
          background: 0,
          algorithm: algorithm,
          image: lines.map { |line| line.chars.map { |char| char == "#" ? 1 : 0 } }
        )
      end

      attribute :background, type: Integer
      attribute :algorithm, type: Array
      attribute :image, type: Array

      def enhance(*)
        new_background = background.zero? ? algorithm.first : algorithm.last

        new_image = Array.new(height + BUFFER * 2) { Array.new(width + BUFFER * 2, new_background) }
        new_image.size.times do |new_row|
          new_image.first.size.times do |new_column|
            new_image[new_row][new_column] = algorithm[value_at(new_row - BUFFER, new_column - BUFFER)]
          end
        end

        with(background: new_background, image: new_image)
      end

      def lit_pixels
        image.sum { |pixels| pixels.count(&:positive?) }
      end

      def print
        image.each do |pixels|
          puts(pixels.map { |pixel| pixel.zero? ? "." : "#" }.join)
        end
      end

    private

      def alternating?
        background.zero? ? algorithm.first.positive? : algorithm.last.zero?
      end

      def height
        image.size
      end

      def width
        image.first.size
      end

      def pixel_at(row, column)
        return background if row.negative? || row >= height || column.negative? || column >= width

        image[row][column]
      end

      # rubocop:disable Layout/SpaceBeforeComma, Metrics/AbcSize
      def value_at(row, column)
        [
          pixel_at(row - 1, column - 1), pixel_at(row - 1, column), pixel_at(row - 1, column + 1),
          pixel_at(row    , column - 1), pixel_at(row    , column), pixel_at(row    , column + 1),
          pixel_at(row + 1, column - 1), pixel_at(row + 1, column), pixel_at(row + 1, column + 1)
        ].join.to_i(2)
      end
      # rubocop:enable Layout/SpaceBeforeComma, Metrics/AbcSize
    end
  end
end
