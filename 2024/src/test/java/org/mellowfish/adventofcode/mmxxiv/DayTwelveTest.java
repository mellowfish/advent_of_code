package org.mellowfish.adventofcode.mmxxiv;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.nio.file.Files;
import java.nio.file.Paths;
import org.junit.jupiter.api.Test;

class DayTwelveTest {
    static final String SAMPLE_INPUT = """
            AAAA
            BBCD
            BBCC
            EEEC
            """;

    String fullInput() throws Exception {
        return Files.readString(Paths.get("src/test/resources/day_twelve_full_input.txt"));
    }

    @Test
    void testPartOneExample() {
        assertEquals(140, DayTwelve.with(SAMPLE_INPUT).partOne());
    }

    @Test
    void testPartOneFullInput() throws Exception {
        assertEquals(1375476, DayTwelve.with(fullInput()).partOne());
    }

    @Test
    void testPartTwoExample() {
        assertEquals(80, DayTwelve.with(SAMPLE_INPUT).partTwo());
    }

    @Test
    void testPartTwoWithFullInput() throws Exception {
        assertEquals(42, DayTwelve.with(fullInput()).partTwo());
    }
}
