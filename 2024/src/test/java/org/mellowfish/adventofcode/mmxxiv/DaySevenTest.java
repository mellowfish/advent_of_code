package org.mellowfish.adventofcode.mmxxiv;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.math.BigInteger;
import java.nio.file.Files;
import java.nio.file.Paths;
import org.junit.jupiter.api.Test;

class DaySevenTest {
    static final String SAMPLE_INPUT =
            """
            190: 10 19
            3267: 81 40 27
            83: 17 5
            156: 15 6
            7290: 6 8 6 15
            161011: 16 10 13
            192: 17 8 14
            21037: 9 7 18 13
            292: 11 6 16 20
            """;

    String fullInput() throws Exception {
        return Files.readString(Paths.get("src/test/resources/day_seven_full_input.txt"));
    }

    @Test
    void testPartOneExample() {
        assertEquals(BigInteger.valueOf(3749), DaySeven.with(SAMPLE_INPUT).partOne());
    }

    @Test
    void testPartOneFullInput() throws Exception {
        assertEquals(new BigInteger("1289579105366"), DaySeven.with(fullInput()).partOne());
    }

    @Test
    void testPartTwoExample() {
        assertEquals(BigInteger.valueOf(11387), DaySeven.with(SAMPLE_INPUT).partTwo());
    }

    @Test
    void testPartTwoWithFullInput() throws Exception {
        assertEquals(
                new BigInteger("92148721834692"), DaySeven.with(fullInput()).partTwo());
    }
}
