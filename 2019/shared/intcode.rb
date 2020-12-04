module AdventOfCode
  module Intcode
    class Program
      def self.from_string(raw_string)
        Program.new(raw_string.split(",").map(&:to_i))
      end

      attr_reader :raw_sequence

      def initialize(raw_sequence)
        @raw_sequence = raw_sequence
      end

      def call(input: StandardProgramInput.new, output: StandardProgramOutput.new)
        execution =
          Execution.new(
            program: self,
            instruction_pointer: 0,
            relative_base: 0,
            input: input,
            output: output
          )
        execution = execution.step while execution.incomplete?
        execution.program
      end

      def blockable_executor(input: StandardProgramInput.new, output: StandardProgramOutput.new)
        BlockableExecutor.new(
          Execution.new(
            program: self,
            instruction_pointer: 0,
            relative_base: 0,
            input: input,
            output: output
          )
        )
      end

      def instruction
        get(0)
      end

      def get(index)
        return 0 if index >= raw_sequence.size

        raw_sequence[index]
      end
      alias_method :[], :get

      def set(index, value)
        new_sequence = raw_sequence.dup
        if index >= new_sequence.size
          ((new_sequence.size)..(index + 1)).each do |new_index|
            new_sequence[new_index] = 0
          end
        end

        new_sequence[index] = value

        Program.new(new_sequence)
      end
      alias_method :[]=, :set
    end

    class BlockableExecutor
      attr_reader :current_execution

      def initialize(current_execution)
        @current_execution = current_execution
      end

      def execute_until_block
        @current_execution = current_execution.step while current_execution.incomplete? && current_execution.non_blocking?

        current_execution.output
      end

      def execute_until_block_with_input(input)
        @current_execution = current_execution.with_input(input)

        execute_until_block
      end

      def halted?
        current_execution.halted?
      end
    end

    class Execution
      attr_reader :program, :instruction_pointer, :relative_base, :input, :output
      attr_reader :instruction, :opcode, :args

      def initialize(
        program:,
        instruction_pointer: 0,
        relative_base: 0,
        input: StandardProgramInput.new,
        output: StandardProgramOutput.new
      )
        @program = program
        @instruction_pointer = instruction_pointer
        @relative_base = relative_base
        @input = input
        @output = output

        @instruction = program[instruction_pointer]
        @opcode = instruction % 100
        @args = (instruction - opcode) / 100
      end

      def next_execution(next_program:, next_instruction_pointer:)
        Execution.new(
          program: next_program,
          instruction_pointer: next_instruction_pointer,
          relative_base: relative_base,
          input: input,
          output: output
        )
      end

      def with_input(new_input)
        Execution.new(
          program: program,
          instruction_pointer: instruction_pointer,
          relative_base: relative_base,
          input: new_input,
          output: output
        )
      end

      def non_blocking?
        case opcode
        when 3 then input.ready?
        else true
        end
      end

      def step
        # puts self # DEBUG
        case opcode
        when 1 then add
        when 2 then multiply
        when 3 then read_input
        when 4 then write_output
        when 5 then jump_if_true
        when 6 then jump_if_false
        when 7 then less_than
        when 8 then equal_to
        when 9 then adjust_relative_base
        when 99 then self
        else raise "Invalid opcode: #{opcode}"
        end
      end

      def arg_mode(arg_index)
        ((args % (10.0 ** (arg_index + 1))) - (args % (10.0 ** arg_index))) / (10.0 ** arg_index)
      end

      def arg_value(arg_index:, mode: arg_mode(arg_index))
        raw_arg = program[instruction_pointer + arg_index + 1]
        # puts(format("Argmode %d for index %d", arg_mode(arg_index), arg_index)))
        case (mode)
        when 0 then program[raw_arg]
        when 1 then raw_arg
        when 2 then program[relative_base + raw_arg]
        else raise Exception(format("Invalid arg mode %d for arg index %d", arg_mode(arg_index), arg_index))
        end
      end

      def arg_string(arg_index:, mode: arg_mode(arg_index))
        raw_arg = program[instruction_pointer + arg_index + 1]
        value = arg_value(arg_index: arg_index, mode: mode)
        # puts(format("Argmode %d for index %d", arg_mode(arg_index), arg_index))
        case mode
        when 0 then format("[%d](%d)", raw_arg, value)
        when 1 then format("!(%d)", value)
        when 2 then format("[%d + %d](%d)", relative_base, raw_arg, value)
        else raise Exception(format("Invalid arg mode %d for arg index %d", arg_mode(arg_index), arg_index))
        end
      end

      def target_value(arg_index:, mode: arg_mode(arg_index))
        raw_arg = program[instruction_pointer + arg_index + 1]

        case mode
        when 0, 1 then raw_arg
        else relative_base + raw_arg
        end
      end

      # Operations

      # Opcode XXX01
      def add
        next_execution(
          next_program: program.set(target_value(arg_index: 2), arg_value(arg_index: 0) + arg_value(arg_index: 1)),
          next_instruction_pointer: instruction_pointer + 4
        )
      end

      # Opcode XXX02
      def multiply
        next_execution(
          next_program: program.set(target_value(arg_index: 2), arg_value(arg_index: 0) * arg_value(arg_index: 1)),
          next_instruction_pointer: instruction_pointer + 4
        )
      end

      # Opcode XXX03
      def read_input
        next_execution(
          next_program: program.set(target_value(arg_index: 0), input.read_int()),
          next_instruction_pointer: instruction_pointer + 2
        )
      end

      # Opcode XXX04
      def write_output
        output.print_int(arg_value(arg_index: 0))
        next_execution(next_program: program, next_instruction_pointer: instruction_pointer + 2)
      end

      # Opcode XXX05
      def jump_if_true
        if arg_value(arg_index: 0).zero?
          next_execution(next_program: program, next_instruction_pointer: instruction_pointer + 3)
        else
          next_execution(next_program: program, next_instruction_pointer: arg_value(arg_index: 1))
        end
      end

      # Opcode XXX06
      def jump_if_false
        if arg_value(arg_index: 0).zero?
          next_execution(next_program: program, next_instruction_pointer: arg_value(arg_index: 1))
        else
          next_execution(next_program: program, next_instruction_pointer: instruction_pointer + 3)
        end
      end

      # Opcode XXX07
      def less_than
        next_execution(
          next_program:
            program.set(
              target_value(arg_index: 2),
              (arg_value(arg_index: 0) < arg_value(arg_index: 1)) ? 1 : 0
            ),
          next_instruction_pointer: instruction_pointer + 4
        )
      end

      # Opcode XXX08
      def equal_to
        next_execution(
          next_program:
            program.set(
              target_value(arg_index: 2),
              arg_value(arg_index: 0) == arg_value(arg_index: 1) ? 1 : 0
            ),
          next_instruction_pointer: instruction_pointer + 4
        )
      end

      # Opcode XXX09
      def adjust_relative_base
        # println(format("Adjusting relative base to %d", relative_base + arg_value(arg_index: 0)))
        Execution.new(
          program: program,
          instruction_pointer: instruction_pointer + 2,
          relative_base: relative_base + arg_value(arg_index: 0),
          input: input,
          output: output
        )
      end

      def halted?
        opcode == 99
      end

      def incomplete?
        !halted?
      end

      # Other

      def to_s
        instruction_string = format("[%2d](%04d)", instruction_pointer, instruction)
        full_instruction_string =
          format(
            "%s,%4d,%4d,%4d",
            instruction_string,
            program[instruction_pointer + 1],
            program[instruction_pointer + 2],
            program[instruction_pointer + 3]
          )

        case opcode
        when 1
          format(
            "%10s # %s + %s -> %s",
            full_instruction_string, arg_string(arg_index: 0), arg_string(arg_index: 1), arg_string(arg_index: 2)
          )
        when 2
          format(
            "%10s # %s * %s -> %s",
            full_instruction_string, arg_string(arg_index: 0), arg_string(arg_index: 1), arg_string(arg_index: 2)
          )
        when 3
          format(
            "%10s # input -> %s",
            full_instruction_string, arg_string(arg_index: 0)
          )
        when 4
          format(
            "%10s # %s -> output",
            full_instruction_string, arg_string(arg_index: 0)
          )
        when 5
          format(
            "%10s # if %s != 0 jump to %s",
            full_instruction_string, arg_string(arg_index: 0), arg_string(arg_index: 1)
          )
        when 6
          format(
            "%10s # if %s == 0 jump to %s",
            full_instruction_string, arg_string(arg_index: 0), arg_string(arg_index: 1)
          )
        when 7
          format(
            "%10s # %s < %s -> %s",
            full_instruction_string, arg_string(arg_index: 0), arg_string(arg_index: 1), arg_string(arg_index: 2)
          )
        when 8
          format(
            "%10s # %s == %s -> %s",
            full_instruction_string, arg_string(arg_index: 0), arg_string(arg_index: 1), arg_string(arg_index: 2)
          )
        when 9
          format("%10s # !(%d) + %s -> relative", full_instruction_string, relative_base, arg_string(arg_index: 0))
        when 99
          format("%10s # halt", full_instruction_string)
        else raise Exception(format("Unknown opcode %s", full_instruction_string))
        end
      end
    end

    class StandardProgramInput
      attr_reader :display_prompt

      def initialize(display_prompt: true)
        @display_prompt = display_prompt
      end

      def read_int
        puts("?: ") if display_prompt

        read_line.to_i
      end

      def ready?
        # TODO...
        true
      end
    end

    class CustomProgramInput
      attr_reader :input_list

      def initialize(input_list)
        @input_list = input_list
        @input_iterator = input_list.to_enum
      end

      def read_int
        @input_iterator.next
      end

      def ready?
        @input_iterator.peek
        true
      rescue StopIteration
        false
      end
    end

    class StandardProgramOutput
      def print_int(value)
        puts value
      end
    end

    class CapturedProgramOutput
      attr_reader :output

      def initialize(output = [])
        @output = output
      end
      def print_int(value)
        output << value
      end
    end
  end
end
