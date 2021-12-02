module AdventOfCode
  module Domain
    class Submarine
      attr_reader :position, :aim, :command_executor

      def initialize(position: Position.origin, aim: 0, command_executor: ->(sub, *) { sub })
        @position = position
        @aim = aim
        @command_executor = command_executor
      end

      def execute_command(command)
        command_executor.call(self, command)
      end

      def with(position: self.position, aim: self.aim, command_executor: self.command_executor)
        self.class.new(position: position, aim: aim, command_executor: command_executor)
      end
    end
  end
end
