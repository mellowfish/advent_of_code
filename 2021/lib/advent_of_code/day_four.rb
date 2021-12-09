module AdventOfCode
  class DayFour
    def self.for(input: $stdin)
      new(Bingo::Game.parse(input.readlines.map(&:strip)))
    end

    attr_reader :games

    def initialize(game)
      @games = [game]
    end

    def part_one
      games << games.last.call_next_number until games.last.winner?

      games.last.winning_score
    end

    def part_two
      games << games.last.call_next_number while games.last.unfinished?

      games.last.losing_score
    end

    module Bingo
      class Board
        class Cell < Shared::Model
          attribute :number, type: Integer
          attribute :called, type: :boolean, default: false

          def call(called_number)
            if called_number == number
              with(called: true)
            else
              self
            end
          end
        end

        def self.parse(lines)
          new(lines.map { |line| line.split(/\s+/).map { |cell| Cell.new(number: cell.to_i) } })
        end

        attr_reader :matrix

        def initialize(matrix)
          @matrix = matrix
        end

        def play(number)
          with(
            matrix.each_with_object([]) do |row, new_matrix|
              new_matrix << row.each_with_object([]) do |cell, new_row|
                new_row << cell.call(number)
              end
            end
          )
        end

        def with(new_matrix)
          self.class.new(new_matrix)
        end

        def won?
          full_row? || full_column?
        end

        def full_row?
          matrix.any? { |row| row.all?(&:called?) }
        end

        def full_column?
          matrix.transpose.any? { |column| column.all?(&:called?) }
        end

        def score
          matrix.flatten.reject(&:called?).sum(&:number)
        end
      end

      CallList = Shared::List.of(Integer) do
        def call
          remove_at(0)
        end
      end
      BoardList = Shared::List.of(Board)

      class Game < Shared::Model
        def self.parse(lines)
          call_list_line = lines.shift
          call_list = CallList.new(call_list_line.split(",").map(&:to_i))

          boards = BoardList.new([])
          until lines.empty?
            lines.shift while lines.first.empty? # gap
            board_lines = lines.shift(5)
            boards = boards.append(Board.parse(board_lines)) # rubocop:disable Style/RedundantSelfAssignment
          end

          new(call_list: call_list, boards: boards)
        end

        attribute :last_called_number, type: Integer, required: false
        attribute :call_list, type: CallList
        attribute :boards, type: BoardList
        attribute :won_boards, type: BoardList, default: -> { BoardList.new([]) }

        def winner?
          won_boards.any?
        end

        def winning_score
          winner.score * last_called_number
        end

        def winner
          won_boards.first
        end

        def losing_score
          loser.score * last_called_number
        end

        def loser
          boards.first || won_boards.last
        end

        def finished?
          boards.empty?
        end

        def unfinished?
          !finished?
        end

        def call_next_number
          next_number, next_call_list = call_list.call
          next_boards = boards.map { |board| board.play(next_number) }

          next_won_boards, next_playable_boards = next_boards.partition(&:won?)
          with(
            last_called_number: next_number,
            call_list: next_call_list,
            boards: next_playable_boards,
            won_boards: won_boards.append_all(next_won_boards)
          )
        end
      end
    end
  end
end
