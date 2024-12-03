package org.mellowfish.adventofcode.mmxxiv;

import java.util.Scanner;
import java.util.regex.MatchResult;
import java.util.regex.Pattern;

public class DayThree {
    public static final Pattern MULTIPLICATION_PATTERN =
            Pattern.compile("mul\\((?<left>\\d{1,3}),(?<right>\\d{1,3})\\)");
    public static final Pattern MULTIPLE_PATTERN =
            Pattern.compile("(?<do>do\\(\\))|(?<dont>don't\\(\\))|mul\\((?<left>\\d{1,3}),(?<right>\\d{1,3})\\)");

    private final String input;

    public static DayThree with(String input) {
        return new DayThree(input);
    }

    public DayThree(String input) {
        this.input = input;
    }

    public long partOne() {
        return new Scanner(input)
                .findAll(MULTIPLICATION_PATTERN)
                .mapToLong(this::evaluateMultiplication)
                .sum();
    }

    public long partTwo() {
        var ref = new Object() {
            boolean multiplying = true;
            long total = 0;
        };
        new Scanner(input).findAll(MULTIPLE_PATTERN).forEach(matchResult -> {
            String matchedText = matchResult.group();
            switch (matchedText) {
                case "do()":
                    ref.multiplying = true;
                    break;
                case "don't()":
                    ref.multiplying = false;
                    break;
                default:
                    if (ref.multiplying) {
                        ref.total += evaluateMultiplication(matchResult);
                    }
            }
        });

        return ref.total;
    }

    private long evaluateMultiplication(MatchResult result) {
        return Long.parseLong(result.group("left")) * Long.parseLong(result.group("right"));
    }
}
