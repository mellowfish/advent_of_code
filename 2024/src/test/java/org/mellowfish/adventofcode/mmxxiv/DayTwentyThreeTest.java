package org.mellowfish.adventofcode.mmxxiv;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.nio.file.Files;
import java.nio.file.Paths;
import org.junit.jupiter.api.Test;

class DayTwentyThreeTest {
    static final String SAMPLE_INPUT = """
            """;

    String fullInput() throws Exception {
        return Files.readString(Paths.get("src/test/resources/day_twenty_three_full_input.txt"));
    }

    @Test
    void testPartOneExample() {
        assertEquals(41, DayTwentyThree.with(SAMPLE_INPUT).partOne());
    }

    @Test
    void testPartOneFullInput() throws Exception {
        assertEquals(41, DayTwentyThree.with(fullInput()).partOne());
    }

    @Test
    void testPartTwoExample() {
        assertEquals(42, DayTwentyThree.with(SAMPLE_INPUT).partTwo());
    }

    @Test
    void testPartTwoWithFullInput() throws Exception {
        assertEquals(42, DayTwentyThree.with(fullInput()).partTwo());
    }
}
