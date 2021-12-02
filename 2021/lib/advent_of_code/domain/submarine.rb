module AdventOfCode
  module Domain
    class Submarine < Shared::Model
      attribute :position, type: Position, default: Position.origin
      attribute :aim, type: Integer, default: 0
      attribute :command_executor, respond_to: :call, default: ->(sub, *) { sub }

      def execute_command(command)
        command_executor.call(self, command)
      end
    end
  end
end
