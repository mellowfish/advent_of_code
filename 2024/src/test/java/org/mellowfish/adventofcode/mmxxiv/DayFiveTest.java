package org.mellowfish.adventofcode.mmxxiv;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.nio.file.Files;
import java.nio.file.Paths;
import org.junit.jupiter.api.Test;

class DayFiveTest {
    static final String SAMPLE_INPUT =
            """
            47|53
            97|13
            97|61
            97|47
            75|29
            61|13
            75|53
            29|13
            97|29
            53|29
            61|53
            97|53
            61|29
            47|13
            75|47
            97|75
            47|61
            75|61
            47|29
            75|13
            53|13

            75,47,61,53,29
            97,61,53,29,13
            75,29,13
            75,97,47,61,53
            61,13,29
            97,13,75,29,47
            """;

    String fullInput() throws Exception {
        return Files.readString(Paths.get("src/test/resources/day_five_full_input.txt"));
    }

    @Test
    void testPartOneExample() {
        assertEquals(143, DayFive.with(SAMPLE_INPUT).partOne());
    }

    @Test
    void testPartOneFullInput() throws Exception {
        assertEquals(6951, DayFive.with(fullInput()).partOne());
    }

    @Test
    void testPartTwoExample() {
        assertEquals(123, DayFive.with(SAMPLE_INPUT).partTwo());
    }

    @Test
    void testPartTwoWithFullInput() throws Exception {
        assertEquals(4121, DayFive.with(fullInput()).partTwo());
    }
}
