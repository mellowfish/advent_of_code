module AdventOfCode
  module Shared
    class NumberValue < Value
      class << self
        def parse(str)
          new(str.to_i)
        end

        def zero
          new(0)
        end

        def one
          new(1)
        end
      end


      def +(other)
        raise ArgumentError unless instance_of?(other.class)

        with(value + other.__send__(:value))
      end
    end
  end
end
