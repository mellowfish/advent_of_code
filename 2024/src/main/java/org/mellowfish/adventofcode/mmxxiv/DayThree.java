package org.mellowfish.adventofcode.mmxxiv;

import java.util.Scanner;
import java.util.regex.MatchResult;
import java.util.regex.Pattern;

public class DayThree {
    public static final Pattern MULTIPLICATION_PATTERN = Pattern.compile("mul[(](?<left>\\d{1,3})[,](?<right>\\d{1,3})[)]");

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
        Scanner scanner = new Scanner(input);
        long total = 0;
        while (scanner.hasNext()) {
            if (scanner.hasNext(MULTIPLICATION_PATTERN)) {
                total += evaluateMultiplication(scanner.match());
            } else {
                scanner.next();
            }
        }

        return total;
    }

    private long evaluateMultiplication(MatchResult result) {
        return Long.parseLong(result.group("left")) * Long.parseLong(result.group("right"));
    }
}
