package org.mellowfish.adventofcode.mmxxiv;

import lombok.AllArgsConstructor;

@AllArgsConstructor
public class DayOne {
    private final String input;

    public static DayOne with(String input) {
        return new DayOne(input);
    }

    public int partOne() {
        return 41;
    }

    public int partTwo() {
        return 43;
    }
}
