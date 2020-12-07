require_relative "../shared/day"

module AdventOfCode
  class DaySix < Day
    def do_part_one
      super
      p groups.sum(&:score_with_any)
    end

    def do_part_two
      super
      p groups.sum(&:score_with_all)
    end

    def groups
      @groups ||= ARGF.read.strip.split("\n\n").map { |group_text| Group.parse(group_text) }
    end

    class Group
      def self.parse(group_text)
        new(group_text.split("\n").map { |line| line.chars })
      end

      attr_reader :answers_per_person

      def initialize(answers_per_person)
        @answers_per_person = answers_per_person
      end

      def answers
        @answers ||=
          answers_per_person.each_with_object(Hash.new { 0 }) do |answers, hash|
            answers.each do |answer|
              hash[answer] += 1
            end
          end
      end

      def score_with_any
        answers.keys.size
      end

      def score_with_all
        answers.count { |_key, size| size == answers_per_person.size}
      end
    end
  end

  def self.run_day_six
    DaySix.call
  end
end

if __FILE__ == $0
  AdventOfCode.run_day_six
end
