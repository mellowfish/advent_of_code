module AdventOfCode
  class DayTwo
    def self.for(input: $stdin)
      lines = input.readlines
      commands = lines.map { |line| Command.parse(line) }
      new(commands)
    end

    attr_reader :commands

    def initialize(commands)
      @commands = commands
    end

    def part_one
      commands.reduce(Domain::Submarine.new(command_executor: CartesianCommands)) do |sub, command|
        sub.execute_command(command)
      end
    end

    def part_two
      commands.reduce(Domain::Submarine.new(command_executor: AimableCommands)) do |sub, command|
        sub.execute_command(command)
      end
    end

    module CartesianCommands
      def self.call(submarine, command)
        position = submarine.position

        new_position =
          case command.action
          when "forward"
            position.forward(command.units)
          when "down"
            position.down(command.units)
          when "up"
            position.up(command.units)
          else
            raise "Action not recognized: #{action}"
          end

        submarine.with(position: new_position)
      end
    end

    module AimableCommands
      def self.call(submarine, command)
        position = submarine.position
        aim = submarine.aim

        case command.action
        when "forward"
          position = position.forward(command.units).down(aim * command.units)
        when "down"
          aim += command.units
        when "up"
          aim -= command.units
        else
          raise "Action not recognized: #{action}"
        end

        submarine.with(position: position, aim: aim)
      end
    end

    class Command < Shared::Model
      attribute :action, type: String
      attribute :units, type: Integer

      def self.parse(str)
        parts = str.split(" ")
        raise(ArgumentError, "Failed to parse command: #{str}") unless parts.size == 2

        action, units_string = *parts
        units = units_string.to_i
        raise(ArgumentError, "Invalid units: #{units_string}") unless units.to_s == units_string

        new(action: action, units: units)
      end
    end
  end
end
