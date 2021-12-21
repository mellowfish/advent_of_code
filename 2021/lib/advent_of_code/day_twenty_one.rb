module AdventOfCode
  class DayTwentyOne
    def self.for(input: $stdin)
      new(Game.parse(input.readlines(chomp: true)))
    end

    attr_reader :game

    def initialize(game)
      @game = game
    end

    def part_one
      @game = game.take_turn until game.won?
      game.score
    end

    def part_two
      raise "Not yet!"
    end

    class Player < Shared::Model
      attribute :position, type: Integer
      attribute :points, type: Integer, default: 0

      def move(distance)
        new_position = ((position + distance - 1) % 10) + 1

        with(
          position: new_position,
          points: points + new_position
        )
      end

      def won?
        points >= 1000
      end
    end

    class DeterministicDie
      def initialize
        @value = 0
      end

      def roll
        @value += 1
        @value = 1 if @value == 101
        @value
      end
    end

    class Game < Shared::Model
      ROLLS_PER_TURN = 3

      def self.parse(lines)
        one, two = lines.map { |line| line[-1].to_i }
        new(
          players: [Player.new(position: one), Player.new(position: two)],
          die: DeterministicDie.new
        )
      end

      attribute :players, type: Array
      attribute :die, respond_to: :roll
      attribute :rolls, type: Integer, default: 0

      def take_turn
        with(
          players: other_players + [current_player_next],
          rolls: rolls + die_values.size
        )
      end

      def won?
        players.any?(&:won?)
      end

      def losers
        players.reject(&:won?)
      end

      def score
        losers.sum(&:points) * rolls
      end

      def print
        puts "Player #{current_player_number} rolls #{die_values.join('+')} and moves to space #{current_player_next.position} for a total score of #{current_player_next.points}."
      end

    private

      def current_player_next
        @current_player_next ||= current_player.move(die_values.sum)
      end

      def die_values
        @die_values ||= Array.new(ROLLS_PER_TURN) { die.roll }
      end

      def current_player_number
        (rolls / 3) % players.size + 1
      end

      def current_player
        players.first
      end

      def other_players
        players[1..]
      end
    end
  end
end
