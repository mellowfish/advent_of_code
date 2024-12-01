package org.mellowfish.adventofcode.mmxxiv;

import static org.junit.jupiter.api.Assertions.*;

import java.nio.file.Files;
import java.nio.file.Paths;
import org.junit.jupiter.api.Test;

class DayOneTest {
    static final String SAMPLE_INPUT =
            """
                3   4
                4   3
                2   5
                1   3
                3   9
                3   3
                """;

    String fullInput() throws Exception {
        return Files.readString(Paths.get("src/test/resources/day_one_full_input.txt"));
    }

    @Test
    void testPartOneExample() {
        assertEquals(11, DayOne.with(SAMPLE_INPUT).partOne());
    }

    @Test
    void testPartOneFullInput() throws Exception {
        assertEquals(2066446, DayOne.with(fullInput()).partOne());
    }

    @Test
    void testPartTwoExample() {
        assertEquals(31, DayOne.with(SAMPLE_INPUT).partTwo());
    }

    @Test
    void testPartTwoWithFullInput() throws Exception {
        assertEquals(24931009, DayOne.with(fullInput()).partTwo());
    }
}
