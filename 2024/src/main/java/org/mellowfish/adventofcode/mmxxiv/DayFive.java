package org.mellowfish.adventofcode.mmxxiv;

import static java.lang.System.exit;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class DayFive {
    private final String input;
    private final List<String> lines;
    private final ArrayList<Rule> rules;
    private final ArrayList<Book> books;

    public static DayFive with(String input) {
        return new DayFive(input);
    }

    DayFive(String input) {
        this.input = input;
        this.lines = input.lines().toList();
        this.rules = new ArrayList<>();
        this.books = new ArrayList<>();
        parseInput();
    }

    public int partOne() {
        return books.stream().filter(this::bookIsValid).map(Book::middlePage).reduce(0, Integer::sum);
    }

    public int partTwo() {
        return books.stream()
                .filter(this::bookIsInvalid)
                .map(this::fixBook)
                .map(Book::middlePage)
                .reduce(0, Integer::sum);
    }

    void parseInput() {
        int i = 0;
        for (; i < lines.size(); i++) {
            String line = lines.get(i);
            if (line.isBlank()) {
                i++;
                break;
            }
            String[] parts = line.split("\\|");
            rules.add(new Rule(Integer.parseInt(parts[0]), Integer.parseInt(parts[1])));
        }

        for (; i < lines.size(); i++) {
            String line = lines.get(i);
            books.add(new Book(
                    Arrays.stream(line.split(",")).map(Integer::parseInt).toList()));
        }
    }

    boolean bookIsValid(Book book) {
        return rules.stream().allMatch(book::compliesToRule);
    }

    boolean bookIsInvalid(Book book) {
        return !bookIsValid(book);
    }

    Book fixBook(Book book) {
        Book previousBook;
        int limit = 100;
        while (!bookIsValid(book)) {
            if (limit-- == 0) {
                break;
            }
            previousBook = book;
            book = rules.stream().reduce(book, Book::applyRule, (originalBook, newBook) -> newBook);
            if (previousBook.equals(book)) {
                exit(1);
            }
        }
        return book;
    }

    record Rule(int before, int after) {}

    record Book(List<Integer> pages) {
        public boolean compliesToRule(Rule rule) {
            if (canIgnoreRule(rule)) {
                return true; // "complies" by not applying
            }

            return pages.indexOf(rule.before) <= pages.indexOf(rule.after);
        }

        public Integer middlePage() {
            return pages.get(pages.size() / 2);
        }

        public boolean canIgnoreRule(Rule rule) {
            return !pages.contains(rule.before) || !pages.contains(rule.after);
        }

        public Book applyRule(Rule rule) {
            if (compliesToRule(rule)) {
                return this;
            }

            ArrayList<Integer> newPages = new ArrayList<>(pages);
            newPages.remove(newPages.indexOf(rule.before));
            newPages.add(newPages.indexOf(rule.after), rule.before);
            return new Book(newPages);
        }
    }
}
