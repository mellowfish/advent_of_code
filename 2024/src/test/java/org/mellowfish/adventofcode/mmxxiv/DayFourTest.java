package org.mellowfish.adventofcode.mmxxiv;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.nio.file.Files;
import java.nio.file.Paths;
import org.junit.jupiter.api.Test;

class DayFourTest {
    static final String SAMPLE_INPUT =
            """
            MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX
            """;

    String fullInput() throws Exception {
        return Files.readString(Paths.get("src/test/resources/day_four_full_input.txt"));
    }

    @Test
    void testPartOneExample() {
        assertEquals(18, DayFour.with(SAMPLE_INPUT).partOne());
    }

    @Test
    void testPartOneFullInput() throws Exception {
        assertEquals(2397, DayFour.with(fullInput()).partOne());
    }

    @Test
    void testPartTwoExample() {
        assertEquals(9, DayFour.with(SAMPLE_INPUT).partTwo());
    }

    @Test
    void testPartTwoWithFullInput() throws Exception {
        assertEquals(1824, DayFour.with(fullInput()).partTwo());
    }
}
