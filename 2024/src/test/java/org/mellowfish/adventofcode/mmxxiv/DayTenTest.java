package org.mellowfish.adventofcode.mmxxiv;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.nio.file.Files;
import java.nio.file.Paths;
import org.junit.jupiter.api.Test;

class DayTenTest {
    static final String SAMPLE_INPUT =
            """
            89010123
            78121874
            87430965
            96549874
            45678903
            32019012
            01329801
            10456732
            """;

    String fullInput() throws Exception {
        return Files.readString(Paths.get("src/test/resources/day_ten_full_input.txt"));
    }

    @Test
    void testPartOneExample() {
        assertEquals(36, DayTen.with(SAMPLE_INPUT).partOne());
    }

    @Test
    void testPartOneFullInput() throws Exception {
        assertEquals(822, DayTen.with(fullInput()).partOne());
    }

    @Test
    void testPartTwoExample() {
        assertEquals(81, DayTen.with(SAMPLE_INPUT).partTwo());
    }

    @Test
    void testPartTwoWithFullInput() throws Exception {
        assertEquals(1801, DayTen.with(fullInput()).partTwo());
    }
}
