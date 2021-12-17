module AdventOfCode
  class DaySeventeen
    def self.for(input: $stdin)
      new(ProbeLauncher.new(initial_probe: Probe.parse(input.readline(chomp: true))))
    end

    attr_reader :launcher

    def initialize(launcher)
      @launcher = launcher
    end

    def part_one
      launcher.launch_solutions.map { |steps| steps.map { |probe| probe.position.y }.max }.max
    end

    def part_two
      launcher.launch_solutions.size
    end

    class Probe < Shared::Model
      def self.parse(str)
        str = str.delete_prefix("target area: ")
        x_str, y_str = str.split(", ")
        left, right = x_str.delete_prefix("x=").split("..").map(&:to_i).sort
        top, bottom = y_str.delete_prefix("y=").split("..").map(&:to_i).sort.reverse

        new(
          position: Shared::Point.origin,
          velocity: Shared::Point.origin,
          target: Shared::Rectangle.new(left: left, right: right, top: top, bottom: bottom)
        )
      end

      attribute :position, type: Shared::Point
      attribute :target, type: Shared::Rectangle
      attribute :velocity, type: Shared::Point

      def in_target?
        target.include?(position)
      end

      def past_target?
        target.left_of?(position) || target.above?(position)
      end

      def step
        with(
          position: position + velocity,
          velocity: Shared::Point.new(
            x: velocity.x < 2 ? 0 : velocity.x - 1,
            y: velocity.y - 1
          )
        )
      end

      def aim(x: velocity.x, y: velocity.y)
        with(velocity: Shared::Point.new(x: x, y: y))
      end
    end

    class ProbeLauncher < Shared::Model
      attribute :initial_probe, type: Probe

      def launch_solutions
        return @launch_solutions if defined?(@launch_solutions)

        @launch_solutions = []
        (minimum_x_velocity..maximum_x_velocity).each do |x_velocity|
          started_hitting = false
          (minimum_y_velocity..maximum_x_velocity).each do |y_velocity|
            probe = initial_probe.aim(x: x_velocity, y: y_velocity)
            fire_probe(probe).then do |steps|
              if steps.last.in_target?
                started_hitting = true
                @launch_solutions << steps
              end
              break if started_hitting && steps.last.past_target?

              nil
            end
          end
        end

        @launch_solutions
      end

      def minimum_x_velocity
        @minimum_x_velocity ||= (1..target.left).find { |n| (n * (n + 1)) / 2.0 >= target.left }
      end

      def maximum_x_velocity
        target.right
      end

      def minimum_y_velocity
        target.bottom
      end

      def maximum_y_velocity
        100
      end

      def fire_probe(probe)
        [probe].tap do |steps|
          until probe.in_target? || probe.past_target?
            probe = probe.step
            steps << probe
          end
        end
      end

      def target
        initial_probe.target
      end

      def print(steps)
        puts
        puts "Velocity: #{steps.first.velocity}"

        x_coordinates = [0, target.left, target.right, *steps.map { |probe| probe.position.x }].sort
        y_coordinates = [0, target.top, target.bottom, *steps.map { |probe| probe.position.y }].sort

        left = x_coordinates.min
        right = x_coordinates.max
        top = y_coordinates.max
        bottom = y_coordinates.min

        (bottom..top).reverse_each do |row|
          puts(
            (left..right).map do |column|
              point = Shared::Point.new(x: column, y: row)
              if point == Shared::Point.origin
                "S"
              elsif steps.map(&:position).include?(point)
                "#"
              elsif target.include?(point)
                "T"
              else
                "."
              end
            end.join
          )
        end
      end
    end
  end
end
