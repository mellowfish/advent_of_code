package org.mellowfish.adventofcode.mmxxiv;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.nio.file.Files;
import java.nio.file.Paths;
import org.junit.jupiter.api.Test;

class DayNineTest {
    static final String SAMPLE_INPUT = """
            2333133121414131402
            """;

    String fullInput() throws Exception {
        return Files.readString(Paths.get("src/test/resources/day_nine_full_input.txt"));
    }

    @Test
    void testPartOneExample() {
        assertEquals(1928, DayNine.with(SAMPLE_INPUT).partOne());
    }

    @Test
    void testPartOneFullInput() throws Exception {
        assertEquals(41, DayNine.with(fullInput()).partOne());
    }

    @Test
    void testPartTwoExample() {
        assertEquals(42, DayNine.with(SAMPLE_INPUT).partTwo());
    }

    @Test
    void testPartTwoWithFullInput() throws Exception {
        assertEquals(42, DayNine.with(fullInput()).partTwo());
    }
}
