require "bundler"
Bundler.setup

require "personal_computing"

class String
  def strip_heredoc
    gsub(/^#{scan(/^[ \t]*(?=\S)/).min}/, "").tap do |stripped|
      stripped.freeze if frozen?
    end
  end
end

# rubocop:disable Performance/RegexpMatch
module Literate
  Token = Data.define(:type, :lexeme, :literal, :line) do
    def inspect
      [type, lexeme, literal].join
    end
  end

  class Scanner
    DIGITS = "0".."9"
    LOWER_LETTERS = "a".."z"
    UPPER_LETTERS = "A".."Z"
    WORD_PUNCTUATION = %w[' -]
    NULL_BYTE = "\0"

    attr_reader :source, :tokens
    attr_accessor :start, :current, :line

    def initialize(source)
      @source = source
      @tokens = []
      @start = 0
      @current = 0
      @indent_level = 0
      @start_of_line = 0
      @line = 1
    end

    def call
      until at_end?
        self.start = current

        scan_token
      end

      tokens << Token.new(:eof, "", nil, line)
      tokens
    end

    def length
      source.length
    end

    private

    def at_end?
      current >= length
    end

    def scan_token
      character = advance
      case character
      when "(" then add_simple_token(:left_paren)
      when ")" then add_simple_token(:right_paren)
      when "{" then add_simple_token(:left_brace)
      when "}" then add_simple_token(:right_brace)
      when "," then add_simple_token(:comma)
      when "." then add_simple_token(:dot)
      when "-"
        if start_of_line?
          add_simple_token(:dash)
        else
          add_simple_token(:minus)
        end
      when "+" then add_simple_token(:plus)
      when ";" then add_simple_token(:semicolon)
      when "*" then add_simple_token(:star)
      when ":" then add_simple_token(:colon)
      when "!" then add_simple_token(match("=") ? :bang_equal : :bang)
      when "=" then add_simple_token(match("=") ? :equal_equal : :equal)
      when "<" then add_simple_token(match("=") ? :less_equal : :less)
      when ">" then add_simple_token(match("=") ? :greater_equal : :greater)
      when "/" then add_simple_token(:slash)
      when "\"" then string
      when "`"
        if peek_next == "`" && peek_next(2) == "`"
          code_block
        else
          raise "malformed code block start on line #{line}"
        end
      when " "
        indent if start_of_line?
      when "\n"
        @start_of_line = current + 1
        @indent_level = 0
        self.line += 1
      when "\t"
        raise "Line #{line}, tabs are illegal, use spaces instead"
      when "\r"
        nil # ignored
      else
        if digit?(character)
          number
        elsif letter?(character)
          word
        else
          raise "illegal character: '#{character}' at line #{line}"
        end
      end
    end

    def advance(offset = 1)
      self.current += offset

      peek
    end

    def peek
      source[current]
    end

    def peek_next(offset = 1)
      source[current + offset] || NULL_BYTE
    end

    def add_simple_token(type)
      add_token(type, nil)
    end

    def digit?(character)
      DIGITS.cover?(character)
    end

    def letter?(character)
      LOWER_LETTERS.cover?(character) || UPPER_LETTERS.cover?(character)
    end

    def match(character)
      return false if at_end?
      return false if peek != character

      self.current += 1
      true
    end

    def indent
      while peek == " "
        @indent_level += 1
        advance
      end

      add_token(:indent, @indent_level)
    end

    def string
      while peek != "\"" && !at_end?
        self.line += 1 if peek == '\n'
        advance
      end

      raise "Unterminated string on line #{line}." if at_end?

      # the last ".
      advance

      # Trim the surrounding quotes.
      value = source[(start + 1)..(current - 1)]
      add_token(:string, value)
    end

    def number
      # TODO
      # private void number() {
      #     while (isDigit(peek())) advance();
      #
      #     // Look for a fractional part.
      #     if (peek() == '.' && isDigit(peekNext())) {
      #       // Consume the "."
      #       advance();
      #
      #       while (isDigit(peek())) advance();
      #     }
      #
      #     addToken(NUMBER,
      #         Double.parseDouble(source.substring(start, current)));
      #   }
    end

    def code_block
      advance(3) # ```

      start_line = line
      label = ""
      while letter?(peek)
        label << peek
        advance
      end
      if match("\n")
        self.line += 1
      else
        raise "unexpected character in code block start: '#{peek}', line #{line}"
      end

      start_of_body = current
      end_of_body = nil

      target_end = (" " * @indent_level) + "```\n"
      loop do # loop until end of source or end of code block
        start_of_line = current

        loop do # loop until end of line
          character = peek
          if character == NULL_BYTE
            raise "unterminated code block starting on line #{start_line}"
          end
          break if at_end?
          if character == "\n"
            self.line += 1
            break
          end

          advance
        end

        if source[start_of_line..current] == target_end
          end_of_body = start_of_line - 1
          break
        end
        if match("\n")
          self.line += 1
        else
          raise "expected newline at #{line}"
        end

        raise "unterminated code block starting on line #{start_line}" if at_end?
      end
      if match("\n")
        self.line += 1
        @start_of_line = current
        @indent_level = 0
      else
        raise "expected newline at #{line}"
      end

      add_token(:string, source[start_of_body..end_of_body].strip_heredoc.strip)
    end

    def word
      advance while word_character?(peek)

      add_token(:word, source[start..current].downcase)
    end

    def word_character?(character)
      letter?(character) || word_punctuation?(character)
    end

    def word_punctuation?(character)
      WORD_PUNCTUATION.include?(character)
    end

    def add_token(type, literal)
      text = source[start..current]
      tokens << Token.new(type, text, literal, line)
    end

    def start_of_line?
      @start_of_line
    end
  end

  def self.parse(path)
    tokens = Scanner.new(File.read(path)).call
  end
end
# rubocop:enable Performance/RegexpMatch
