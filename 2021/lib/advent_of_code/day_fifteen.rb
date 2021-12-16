module AdventOfCode
  class DayFifteen
    def self.for(input: $stdin)
      new(ChitonousCave.parse(input.readlines(chomp: true)))
    end

    attr_reader :cave

    def initialize(cave)
      @cave = cave
    end

    def part_one
      cave.print
      cave.safest_path_score
    end

    def part_two
      new_cave = cave.expand
      new_cave.print
      new_cave.safest_path_score
    end

    class ChitonousCave < Shared::Model
      def self.parse(lines)
        new(matrix: lines.map { |line| line.chars.map(&:to_i) })
      end

      attribute :matrix, type: Array

      def safest_path_score
        distance_to_nodes[node_count - 1]
      end

      def expand(multiplier = 5)
        new_matrix = Array.new(width * multiplier) { Array.new(width * multiplier, nil) }
        multiplier.times do |y|
          multiplier.times do |x|
            delta = x + y
            matrix.each_with_index do |cells, row|
              cells.each_with_index do |cell, column|
                new_cell = cell + delta
                new_cell -= 9 if new_cell > 9
                new_matrix[width * y + row][width * x + column] = new_cell
              end
            end
          end
        end
        with(matrix: new_matrix)
      end

      def print(destination = node_count - 1)
        path = find_shortest_path(destination)

        matrix.each_with_index do |cells, row|
          puts(
            cells.each_with_index.map do |cell, column|
              index = row * width + column
              if path.include?(index)
                "#{cell}*"
              else
                "#{cell} "
              end
            end.join
          )
        end
      end

    private

      def node_count
        width * height
      end

      def find_shortest_path(destination, path = [0])
        return path if shortest_previous_neighbor[destination] == -1

        find_shortest_path(shortest_previous_neighbor[destination], path)
        path << destination
      end

      def distance_to_nodes
        return @distance_to_nodes if defined?(@distance_to_nodes)

        compute_paths

        @distance_to_nodes
      end

      def shortest_previous_neighbor
        return @shortest_previous_neighbor if defined?(@shortest_previous_neighbor)

        compute_paths

        @shortest_previous_neighbor
      end

      def compute_paths(source = 0)
        infinity = node_count * 10
        @distance_to_nodes = Array.new(node_count, infinity)
        @shortest_previous_neighbor = Array.new(node_count, -1)
        @distance_to_nodes[source] = 0
        unvisited_nodes = (0...node_count).to_a

        puts unvisited_nodes.size if node_count > 10_000
        until unvisited_nodes.empty?
          puts unvisited_nodes.size if node_count > 10_000 && (unvisited_nodes.size % 1_000).zero?
          node_to_visit = nil

          unvisited_nodes.find do |minimum_node_index|
            if node_to_visit.nil? || (@distance_to_nodes[minimum_node_index] < @distance_to_nodes[node_to_visit])
              node_to_visit = minimum_node_index
            end
          end

          if @distance_to_nodes[node_to_visit] == infinity
            binding.pry
            break
          end

          unvisited_nodes.delete(node_to_visit)

          neighbors_of(node_to_visit / width, node_to_visit % width).each do |neighbor_row, neighbor_column|
            target_index = neighbor_row * width + neighbor_column
            known_distance = matrix[neighbor_row][neighbor_column]

            new_distance = distance_to_nodes[node_to_visit] + known_distance

            if new_distance < distance_to_nodes[target_index]
              @distance_to_nodes[target_index] = new_distance
              @shortest_previous_neighbor[target_index] = node_to_visit
            end
          end
        end
      end

      def neighbors_of(row, column)
        [].tap do |neighbors|
          neighbors << [row - 1, column] if row.positive?
          neighbors << [row + 1, column] if row < (height - 1)

          neighbors << [row, column - 1] if column.positive?
          neighbors << [row, column + 1] if column < (width - 1)
        end
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
