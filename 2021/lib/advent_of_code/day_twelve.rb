module AdventOfCode
  class DayTwelve
    def self.for(input: $stdin)
      new(CaveSystem.parse(input.readlines.map(&:chomp)))
    end

    attr_reader :cave_system

    def initialize(cave_system)
      @cave_system = cave_system
    end

    def part_one
      cave_system.total_paths(1)
    end

    def part_two
      cave_system.total_paths(2)
    end

    class CaveSystem < Shared::Model
      def self.parse(lines)
        pairs = lines.map { |line| line.split("-") }
        pairs.reduce(new) do |system, (one, two)|
          system.add_tunnel(one, two)
        end
      end

      attribute :caves, type: Array, default: -> { [] }
      attribute :tunnels, type: Hash, default: -> { Hash.new { |hash, key| hash[key] = [] } }
      attribute :small_cave_max, type: Integer, default: 1

      def add_tunnel(one, two)
        new_caves = caves.dup
        new_caves |= [one, two]
        new_tunnels = tunnels.dup
        new_tunnels[one] << two
        new_tunnels[two] << one

        with(caves: new_caves, tunnels: new_tunnels)
      end

      def total_paths(small_cave_max)
        with(small_cave_max: small_cave_max).paths.size
      end

      def paths
        @paths ||= traverse!
      end

    private

      def traverse!
        raise "Missing start" unless tunnels.key?("start")
        raise "Missing end" unless tunnels.key?("end")

        [].tap do |paths|
          current = "start"
          partial_path = [current]
          paths.concat(explore(cave: current, partial_path: partial_path))
        end.uniq
      end

      def explore(cave:, partial_path:)
        # puts "Exploring #{cave}: #{partial_path.join(',')}"
        return [partial_path] if cave == "end"

        [].tap do |new_paths|
          tunnels[cave].each do |neighbor|
            next unless can_visit?(cave: neighbor, partial_path: partial_path)

            new_paths.concat(explore(cave: neighbor, partial_path: partial_path + [neighbor]))
          end
        end
      end

      def small_cave?(cave)
        cave.downcase == cave
      end

      def can_visit?(cave:, partial_path:)
        return true if cave == "end"
        return false if cave == "start"
        return true unless small_cave?(cave)
        return !partial_path.include?(cave) if small_cave_max == 1

        hit_max = partial_path.select { |target| small_cave?(target) }.tally.find do |_target, visits|
          visits == small_cave_max
        end

        if hit_max
          !partial_path.include?(cave)
        else
          true
        end
      end

      def count_visits(target, path)
        path.count(target)
      end
    end
  end
end
