module AdventOfCode
  class DayThree
    def self.for(input: $stdin)
      lines = input.readlines.map(&:chomp)
      new(Domain::DiagnosticData.parse(lines))
    end

    attr_reader :diagnostic_data

    def initialize(diagnostic_data)
      @diagnostic_data = diagnostic_data
    end
  end
end
