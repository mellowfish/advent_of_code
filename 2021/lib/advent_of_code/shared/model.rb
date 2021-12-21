module AdventOfCode
  module Shared
    class Model
      class << self
        def attribute(name, type: nil, respond_to: nil, default: nil, required: !default.nil?)
          name = name.to_sym

          attributes[name] = {}.tap do |hash|
            hash[:validate_instance] =
              validation_lambda(name: name, type: type, respond_to: respond_to, required: required)
            hash[:required] = required
            hash[:default] = default
          end

          attr_reader name

          alias_method "#{name}?", name if type == :boolean
        end

        def attributes
          @attributes ||= {}
        end

        def inherited(subclass)
          super(subclass)

          return if self == Model

          subclass.instance_variable_set("@attributes", attributes.dup)
        end

      private

        def validation_lambda(name:, required:, type: nil, respond_to: nil)
          if !type.nil?
            build_type_check_lambda(name: name, required: required, type: type)
          elsif !respond_to.nil?
            build_respond_to_lambda(name: name, required: required, respond_to: respond_to)
          else
            raise ArgumentError, "Cannot validate attribute, expected type: or respond_to: but got neither"
          end
        end

        def build_type_check_lambda(name:, required:, type:)
          ->(instance) do
            return if !required && instance.nil?

            case type
            when :boolean
              return if instance.is_a?(TrueClass) || instance.is_a?(FalseClass)
            when Array
              return if type.any? { |type_option| instance.is_a?(type_option) }
            when Class
              return if instance.is_a?(type)
            end

            raise(ArgumentError, "Expected type #{type}, got: #{instance.class}, for attribute: #{name}")
          end
        end

        def build_respond_to_lambda(name:, required:, respond_to:)
          ->(instance) do
            return if !required && instance.nil?
            return if instance.respond_to?(respond_to)

            raise(
              ArgumentError,
              "Expected #{instance.inspect} to respond_to?(:#{respond_to}), for attribute: #{name}"
            )
          end
        end
      end

      def initialize(**params)
        validate_params!(params)
        set_given_attributes(params)
        handle_missing_attributes(params)
      end

      def with(**delta)
        self.class.new(**attributes.merge(delta))
      end

    private

      def validate_params!(params)
        params.each do |key, value|
          raise(ArgumentError, "Unknown attribute: #{key}") unless class_attributes.key?(key)

          attribute_info(key)[:validate_instance].call(value)
        end
      end

      def set_given_attributes(params) # rubocop:disable Naming/AccessorMethodName
        params.each do |key, value|
          instance_variable_set("@#{key}", value)
        end
      end

      def handle_missing_attributes(params)
        (class_attributes.keys - params.keys).each do |missing_field|
          info = attribute_info(missing_field)

          raise(ArgumentError, "Missing required field: #{missing_field}") if info[:required] && info[:default].nil?

          default_value = info[:default].respond_to?(:call) ? info[:default].call : info[:default]
          instance_variable_set("@#{missing_field}", default_value)
        end
      end

      def class_attributes
        self.class.attributes
      end

      def attribute_info(name)
        class_attributes.fetch(name)
      end

      def attributes
        class_attributes.each_with_object({}) { |(name, _data), hash| hash[name] = public_send(name) }
      end
    end
  end
end
