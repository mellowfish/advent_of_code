module AdventOfCode
  class DayTwo
    def self.for(input: $stdin)
      new(input.readlines)
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

        action, distance_string = command.split(" ")
        distance = distance_string.to_i

        new_position =
          case action
          when "forward"
            position.forward(distance)
          when "down"
            position.down(distance)
          when "up"
            position.up(distance)
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

        action, units_string = command.split(" ")
        units = units_string.to_i

        case action
        when "forward"
          position = position.forward(units).down(aim * units)
        when "down"
          aim += units
        when "up"
          aim -= units
        else
          raise "Action not recognized: #{action}"
        end

        submarine.with(position: position, aim: aim)
      end
    end
  end
end
