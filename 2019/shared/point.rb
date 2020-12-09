module AdventOfCode
  class Point
    def self.origin
      new(x: 0, y: 0)
    end

    attr_reader :x, :y

    def initialize(x:, y:)
      @x = x
      @y = y
    end

    def hash
      [x, y].hash
    end

    def ==(other)
      return false unless other.is_a?(Point)

      x == other.x && y == other.y
    end
    alias_method :eql?, :==
  end
end
