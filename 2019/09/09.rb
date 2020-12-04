require_relative "../shared/day"
require_relative "../shared/intcode"

module AdventOfCode
  class DayNine < Day
    def do_part_one
      super
      program.call(input: Intcode::CustomProgramInput.new([1]))
    end

    def do_part_two
      super
      program.call(input: Intcode::CustomProgramInput.new([2]))
    end

    def program
      @program ||= Intcode::Program.from_string(ARGF.read)
    end
  end

  def self.run_day_nine
    DayNine.call
  end
end

if __FILE__ == $0
  AdventOfCode.run_day_nine
end
