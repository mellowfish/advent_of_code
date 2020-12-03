require_relative "../shared/day"

module AdventOfCode
  class DayTwelve < Day
    def do_part_one
      super
      system = initial_system
      p system
      1000.times do
        system = system.tick
        p system
      end
    end

    def do_part_two
      super
      system = initial_system
      x_dup_index = -1
      y_dup_index = -1
      z_dup_index = -1

      loop do
        system = system.tick

        x_dup_index = system.ticks if x_dup_index.negative? && system.matches_on_plane(plane: :x, other: initial_system)
        y_dup_index = system.ticks if y_dup_index.negative? && system.matches_on_plane(plane: :y, other: initial_system)
        z_dup_index = system.ticks if z_dup_index.negative? && system.matches_on_plane(plane: :z, other: initial_system)

        break if x_dup_index.positive? && y_dup_index.positive? && z_dup_index.positive?
      end

      p [x_dup_index, y_dup_index, z_dup_index]
      p [x_dup_index, y_dup_index, z_dup_index].reduce(1, &:lcm)
    end

    def initial_system
      @initial_system ||=
        System.new(
          moons:
            ARGF
              .readlines
              .compact
              .map
              .with_index { |line, index| Moon.new(id: index, position: Position.parse(line.strip)) }
        )
    end

    class Position
      REGEX = /^<x=(?<x>-?\d+), y=(?<y>-?\d+), z=(?<z>-?\d+)>$/

      def self.parse(line)
        line.match(REGEX) do |match_data|
          return new(
            x: match_data[:x].to_i,
            y: match_data[:y].to_i,
            z: match_data[:z].to_i
          )
        end
      end

      attr_reader :x, :y, :z

      def initialize(x:, y:, z:)
        @x = x
        @y = y
        @z = z
      end

      def inspect
        "<x=#{x}, y=#{y}, z=#{z}>"
      end

      def to_i
        x.abs + y.abs + z.abs
      end
    end

    class Velocity < Position
      def self.zero
        new(x: 0, y: 0, z: 0)
      end
    end

    class Moon
      attr_reader :id, :position, :velocity

      def initialize(id:, position:, velocity: Velocity.zero)
        @id = id
        @position = position
        @velocity = velocity
      end

      def potential_energy
        position.to_i
      end

      def kinetic_energy
        velocity.to_i
      end

      def total_energy
        potential_energy * kinetic_energy
      end

      def inspect
        "{id=#{id} pos=#{position.inspect}, vel=#{velocity.inspect}}"
      end

      def hash
        id.hash
      end

      def eql?(other)
        return false unless other.is_a?(Moon)

        id == other.id
      end

      def apply_gravity(other)
        raise(ArgumentError, "Expected other to be a Moon, not a #{other.class.name}") unless other.is_a?(Moon)

        self.class.new(
          id: id,
          position: position,
          velocity: Velocity.new(
            x: velocity.x + delta_v_for_plane(plane: :x, other: other),
            y: velocity.y + delta_v_for_plane(plane: :y, other: other),
            z: velocity.z + delta_v_for_plane(plane: :z, other: other)
          )
        )
      end

      def delta_v_for_plane(plane:, other:)
        raise(ArgumentError) unless other.is_a?(Moon)

        other.position.public_send(plane) <=> position.public_send(plane)
      end

      def apply_velocity
        self.class.new(
          id: id,
          position: Position.new(
            x: position.x + velocity.x,
            y: position.y + velocity.y,
            z: position.z + velocity.z
          ),
          velocity: velocity
        )
      end

      def matches_on_plane?(plane:, other:)
        raise(ArgumentError) unless other.is_a?(Moon)

        position.public_send(plane) == other.position.public_send(plane) &&
          velocity.public_send(plane) == other.velocity.public_send(plane)
      end
    end

    class System
      attr_reader :moons, :ticks

      def initialize(moons:, ticks: 0)
        @moons = moons
        @ticks = ticks
      end

      def total_energy
        moons.map(&:total_energy).sum
      end

      def tick
        self.class.new(
          moons: new_moons,
          ticks: ticks + 1
        )
      end

      def new_moons
        new_moons = moons.each_with_object({}) { |moon, hash| hash[moon] = moon }

        moon_pairs.each do |a, b|
          new_a, new_b = apply_gravity(new_moons[a], new_moons[b])
          new_moons[a] = new_a
          new_moons[b] = new_b
        end

        new_moons.values.map(&:apply_velocity)
      end

      def moon_pairs
        moons.product(moons).reject { |a, b| a.eql?(b) }.map { |pair| pair.sort_by(&:id) }.uniq
      end

      def apply_gravity(a, b)
        [a.apply_gravity(b), b.apply_gravity(a)]
      end

      def inspect
        ["After #{tick_count}:"].concat(moons.map(&:inspect)).append("Total energy: #{total_energy}").join("\n")
      end

      def tick_count
        if ticks == 1
          "1 tick"
        else
          "#{ticks} steps"
        end
      end

      def matches_on_plane(plane:, other:)
        raise(ArgumentError) unless other.is_a?(System)

        moons.zip(other.moons).all? { |a, b| a.matches_on_plane?(plane: plane, other: b) }
      end
    end
  end

  def self.run_day_twelve
    DayTwelve.call
  end
end


if __FILE__ == $0
  AdventOfCode.run_day_twelve
end
