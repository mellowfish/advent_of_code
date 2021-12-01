module AdventOfCode
  module Shared
    class List
      def self.of(target_type, &block)
        Class.new(List) do
          define_method(:initialize) do |items, **_ignored_params|
            super(items, type: target_type)
          end

          class_exec(&block)
        end
      end

      def initialize(items, type: items.first.class)
        raise ArgumentError if type == NilClass

        @items = items
        @type = type
      end

      def each(&block)
        items.each(&block)
      end

      def with(new_items)
        self.class.new(new_items, type: type)
      end

      def each_pair(&block)
        sliding_window(size: 2, &block)
      end

      def sliding_window(size:)
        if block_given?
          items.each_cons(size) { |window| yield with(window) }
        else
          items.each_cons(size).map { |window| with(window) }
        end
      end

      def increasing?
        items.first < items.last
      end

      def count(&block)
        items.count(&block)
      end

      def size
        items.size
      end

    private

      attr_reader :items, :type
    end
  end
end
