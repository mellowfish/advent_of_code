module AdventOfCode
  module Shared
    class Model
      class << self
        def attribute(name, type: nil, respond_to: nil, default: nil, required: !default.nil?)
          name = name.to_sym

          attributes[name] = {}.tap do |hash|
            hash[:validate_instance] =
              if !type.nil?
                build_type_check_lambda(name: name, type: type)
              elsif !respond_to.nil?
                build_respond_to_lambda(name: name, respond_to: respond_to)
              else
                raise ArgumentError, "Cannot validate attribute, expected type: or respond_to: but got neither"
              end

            hash[:required] = required
            hash[:default] = default
          end

          attr_reader name
        end

        def attributes
          @attributes ||= {}
        end

      private

        def build_type_check_lambda(name:, type:)
          ->(instance) do
            return if instance.is_a?(type)

            raise(ArgumentError, "Expected type #{type}, got: #{instance.class}, for attribute: #{name}")
          end
        end

        def build_respond_to_lambda(name:, respond_to:)
          ->(instance) do
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

          instance_variable_set("@#{missing_field}", class_attributes[missing_field][:default])
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
