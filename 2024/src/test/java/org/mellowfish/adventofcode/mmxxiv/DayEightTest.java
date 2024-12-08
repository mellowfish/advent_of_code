package org.mellowfish.adventofcode.mmxxiv;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.nio.file.Files;
import java.nio.file.Paths;
import org.junit.jupiter.api.Test;

class DayEightTest {
    static final String SAMPLE_INPUT =
            """
            ............
            ........0...
            .....0......
            .......0....
            ....0.......
            ......A.....
            ............
            ............
            ........A...
            .........A..
            ............
            ............
            """;

    String fullInput() throws Exception {
        return Files.readString(Paths.get("src/test/resources/day_eight_full_input.txt"));
    }

    @Test
    void testPartOneExample() {
        assertEquals(14, DayEight.with(SAMPLE_INPUT).partOne());
    }

    @Test
    void testPartOneFullInput() throws Exception {
        assertEquals(354, DayEight.with(fullInput()).partOne());
    }

    @Test
    void testPartTwoExample() {
        assertEquals(34, DayEight.with(SAMPLE_INPUT).partTwo());
    }

    @Test
    void testPartTwoWithFullInput() throws Exception {
        assertEquals(1263, DayEight.with(fullInput()).partTwo());
    }
}
