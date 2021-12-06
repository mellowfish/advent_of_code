module AdventOfCode
  module Shared
    class List
      def self.of(target_type, &block)
        Class.new(List) do
          define_method(:initialize) do |items, **_ignored_params|
            super(items, type: target_type)
          end

          class_exec(&block) if block
        end
      end

      def initialize(items, type: items.first.class)
        raise(ArgumentError, "type is required") if type == NilClass

        items.each do |item|
          raise(ArgumentError, "Expected a list of items with type: #{type}, got one with type: #{item.class}") unless item.is_a?(type)
        end

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

      def empty?
        size.zero?
      end

      def at(index)
        items[index]
      end

      def first
        at(0)
      end

      def last
        at(size - 1)
      end

      def any?(&block)
        items.any?(&block)
      end

      def all?(&block)
        items.all?(&block)
      end

      def find(&block)
        items.find(&block)
      end

      def map(type: self.class, &block)
        type.new(items.map(&block))
      end

      def partition(&block)
        items.partition(&block).map { |list| with(list) }
      end

      def append(item)
        append_all([item])
      end

      def append_all(new_items)
        if new_items.is_a?(List)
          with(items + new_items.items)
        else
          with(items + new_items)
        end
      end

      def remove_at(index)
        raise IndexError if index >= items.size

        new_items = items.dup
        item = new_items.delete_at(index)

        [item, with(new_items)]
      end

      def filter(&block)
        with(items.select(&block))
      end

      def reject(&block)
        with(items.reject(&block))
      end

    protected

      attr_reader :items, :type
    end
  end
end
