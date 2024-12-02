package org.mellowfish.adventofcode.mmxxiv;

import static org.junit.jupiter.api.Assertions.*;

import java.nio.file.Files;
import java.nio.file.Paths;
import org.junit.jupiter.api.Test;

class DayTwoTest {
    static final String SAMPLE_INPUT =
            """
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
            """;

    String fullInput() throws Exception {
        return Files.readString(Paths.get("src/test/resources/day_two_full_input.txt"));
    }

    @Test
    void testPartOneExample() {
        assertEquals(2, DayTwo.with(SAMPLE_INPUT).partOne());
    }

    @Test
    void testPartOneFullInput() throws Exception {
        assertEquals(559, DayTwo.with(fullInput()).partOne());
    }

    @Test
    void testPartTwoExample() {
        assertEquals(4, DayTwo.with(SAMPLE_INPUT).partTwo());
    }

    @Test
    void testPartTwoWithFullInput() throws Exception {
        assertEquals(601, DayTwo.with(fullInput()).partTwo());
    }
}
