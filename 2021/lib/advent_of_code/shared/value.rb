module AdventOfCode
  module Shared
    class Value
      include Comparable

      def initialize(value)
        @value = value
      end

      def <=>(other)
        return nil unless instance_of?(other.class)

        value <=> other.__send__(:value)
      end

      def with(new_value)
        self.class.new(new_value)
      end

    private

      attr_reader :value
    end
  end
end
