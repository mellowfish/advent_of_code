require_relative "../shared/day"

module AdventOfCode
  class DayTwo < Day
    def do_part_one
      super
      p count_valid_sled_passwords
    end

    def do_part_two
      super
      p count_valid_toboggan_passwords
    end

    def count_valid_sled_passwords
      valid_sled_passwords.size
    end

    def valid_sled_passwords
      password_rules.map { |password| SledRentalValidator.new(password) }.select(&:valid?)
    end

    def count_valid_toboggan_passwords
      valid_toboggan_passwords.size
    end

    def valid_toboggan_passwords
      password_rules.map { |password| TobogganRentalPasswordValidator.new(password) }.select(&:valid?)
    end

    def password_rules
      @password_rules ||= ARGF.readlines.compact.map { |line| PasswordRule.parse(line) }
    end

    class PasswordRule
      REGEX = /^(?<min>\d+)-(?<max>\d+) (?<character>.): (?<password>.+)$/

      def self.parse(line)
        line.match(REGEX) do |match_data|
          return new(
            min: match_data[:min].to_i,
            max: match_data[:max].to_i,
            character: match_data[:character],
            password: match_data[:password]
          )
        end
      end

      attr_reader :min, :max, :character, :password

      def initialize(min:, max:, character:, password:)
        @min = min
        @max = max
        @character = character
        @password = password
      end
    end

    class SledRentalValidator < SimpleDelegator
      def valid?
        range.cover?(password.count(character))
      end

      def range
        min..max
      end
    end

    class TobogganRentalPasswordValidator < SimpleDelegator
      def valid?
        (position_one_matches? && !position_two_matches?) || (!position_one_matches? && position_two_matches?)
      end

      def position_one_matches?
        password[min - 1] == character
      end

      def position_two_matches?
        password[max - 1] == character
      end
    end
  end

  def self.run_day_two
    DayTwo.call
  end
end

if __FILE__ == $0
  AdventOfCode.run_day_two
end
