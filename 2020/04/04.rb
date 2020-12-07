require "bundler/inline"

gemfile do
  source "https://rubygems.org"

  gem "activemodel"
  gem "awesome_print"
end

require "active_support/all"
require "active_model"
require "awesome_print"

require_relative "../shared/day"

module AdventOfCode
  class DayFour < Day
    def do_part_one
      super
      p passports.count(&:required_fields?)
    end

    def do_part_two
      super
      ap passports.reject(&:valid?).map(&:errors).map(&:to_h)
      p passports.count(&:valid?)
    end

    def passports
      @passports ||= ARGF.read.strip.split("\n\n").map { |string| Passport.parse(string) }
    end

    class Passport
      include ActiveModel::Model

      REQUIRED_FIELDS = %i(byr iyr eyr hgt hcl ecl pid)
      OPTIONAL_FIELDS = %i(cid)

      attr_accessor *REQUIRED_FIELDS
      attr_accessor *OPTIONAL_FIELDS
      attr_accessor :hgt_unit

      validates :byr, presence: true, inclusion: 1920..2002
      validates :iyr, presence: true, inclusion: 2010..2020
      validates :eyr, presence: true, inclusion: 2020..2030
      validates :hgt, presence: true
      validates :hgt, inclusion: 150..193, if: :height_in_cms?
      validates :hgt, inclusion: 59..76, if: :height_in_inches?
      validates :hgt_unit, inclusion: %w(in cm)
      validates :hcl, presence: true, format: /\A#[0-9a-f]{6}\Z/
      validates :ecl, presence: true, inclusion: %w(amb blu brn gry grn hzl oth)
      validates :pid, presence: true, format: /\A\d{9}\Z/

      def self.parse(string)
        new(
          **string
              .split("\n")
              .map(&:strip)
              .flat_map { |line| line.split(" ") }
              .each_with_object({}) do |pair, hash|
                key, value = pair.split(":", 2)
                hash[key.to_sym] = value
              end
        )
      end

      def required_fields?
        REQUIRED_FIELDS.all? { |attr| public_send(attr).present? }
      end

      def byr_with_coercion=(value)
        value = value.to_i if value.to_i.to_s == value.to_s
        self.byr_without_coercion = value
      end
      alias_method :byr_without_coercion=, :byr=
      alias_method :byr=, :byr_with_coercion=

      def iyr_with_coercion=(value)
        value = value.to_i if value.to_i.to_s == value.to_s
        self.iyr_without_coercion = value
      end
      alias_method :iyr_without_coercion=, :iyr=
      alias_method :iyr=, :iyr_with_coercion=

      def eyr_with_coercion=(value)
        value = value.to_i if value.to_i.to_s == value.to_s

        self.eyr_without_coercion = value
      end
      alias_method :eyr_without_coercion=, :eyr=
      alias_method :eyr=, :eyr_with_coercion=

      def hgt_with_coercion=(value)
        /^(?<number>\d+)(?<unit>\w+)$/.match(value) do |match_data|
          self.hgt_without_coercion = match_data[:number].to_i
          self.hgt_unit = match_data[:unit]
          return
        end

        self.hgt_without_coercion = value
      end
      alias_method :hgt_without_coercion=, :hgt=
      alias_method :hgt=, :hgt_with_coercion=


      def height_in_inches?
        hgt_unit == "in"
      end

      def height_in_cms?
        hgt_unit == "cm"
      end
    end
  end

  def self.run_day_four
    DayFour.call
  end
end

if __FILE__ == $0
  AdventOfCode.run_day_four
end
