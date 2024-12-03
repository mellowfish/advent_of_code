package org.mellowfish.adventofcode.mmxxiv;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.nio.file.Files;
import java.nio.file.Paths;
import org.junit.jupiter.api.Test;

class DayThreeTest {
    static final String PART_ONE_SAMPLE_INPUT =
            """
            xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
            """;
    static final String PART_TWO_SAMPLE_INPUT =
            """
            xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
            """;

    String fullInput() throws Exception {
        return Files.readString(Paths.get("src/test/resources/day_three_full_input.txt"));
    }

    @Test
    void testPartOneExample() {
        assertEquals(161, DayThree.with(PART_ONE_SAMPLE_INPUT).partOne());
    }

    @Test
    void testPartOneFullInput() throws Exception {
        assertEquals(159892596, DayThree.with(fullInput()).partOne());
    }

    @Test
    void testPartTwoExample() {
        assertEquals(48, DayThree.with(PART_TWO_SAMPLE_INPUT).partTwo());
    }

    @Test
    void testPartTwoWithFullInput() throws Exception {
        assertEquals(92626942, DayThree.with(fullInput()).partTwo());
    }
}
