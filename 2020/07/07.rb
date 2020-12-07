require_relative "../shared/day"

module AdventOfCode
  class DaySeven < Day
    def do_part_one
      super

      matching_colors = colors_to_check = ["shiny gold"]
      until colors_to_check.empty?
        next_colors_to_check = []

        colors_to_check.each do |color|
          new_matches = bags_that_directly_contain(color)
          new_matches.each do |new_match|
            unless matching_colors.include?(new_match)
              next_colors_to_check << new_match
              matching_colors << new_match
            end
          end
        end

        colors_to_check = next_colors_to_check
      end
      matching_colors -= ["shiny gold"]
      p matching_colors.size
    end

    def do_part_two
      super
      rules # ensure loaded

      p Rule.by_container["shiny gold"].total_bags - 1
    end

    def rules
      @rules ||= ARGF.readlines.compact.map { |line| Rule.parse(line.strip) }
    end

    def bags_that_directly_contain(target)
      rules.select { |rule| rule.contains?(target) }.map(&:container)
    end

    class Rule
      def self.by_container
        @all ||= {}
      end

      def self.parse(line)
        line.delete!(".")
        container, contents_string = line.split(" bags contain ")
        content_rules =
          contents_string.split(", ").map do |string|
            number_string, color = string.sub(/ bags?/, "").split(" ", 2)

            if number_string == "no" && color == "other"
              nil
            else
              [number_string.to_i, color]
            end
          end.compact

        by_container[container] ||= new(container: container, content_rules: content_rules)
      end

      attr_reader :container, :content_rules

      def initialize(container:, content_rules:)
        @container = container
        @content_rules = content_rules
      end

      def contains?(color)
        content_rules.any? { |_content_count, content_color| content_color == color }
      end

      def empty?
        content_rules.empty?
      end

      def to_s
        if empty?
          "A #{container} bag contains no other bags."
        else
          "A #{container} bag contains #{content_string}."
        end
      end

      def content_string
        content_rules.map do |content_count, content_color|
          if content_count == 1
            "1 #{content_color} bag"
          else
            "#{content_count} #{content_color} bag"
          end
        end.join(",")
      end

      def total_bags
        return @total_bags if defined?(@total_bags)

        @total_bags = 1

        return @total_bags if empty?

        content_rules.each do |content_count, content_color|
          content_rule = self.class.by_container[content_color]
          @total_bags += content_count * content_rule.total_bags
        end

        @total_bags
      end
    end
  end

  def self.run_day_seven
    DaySeven.call
  end
end

if __FILE__ == $0
  AdventOfCode.run_day_seven
end
