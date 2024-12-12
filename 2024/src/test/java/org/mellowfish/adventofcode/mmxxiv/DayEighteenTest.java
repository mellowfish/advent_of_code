package org.mellowfish.adventofcode.mmxxiv;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.nio.file.Files;
import java.nio.file.Paths;
import org.junit.jupiter.api.Test;

class DayEighteenTest {
    static final String SAMPLE_INPUT = """
            """;

    String fullInput() throws Exception {
        return Files.readString(Paths.get("src/test/resources/day_eighteen_full_input.txt"));
    }

    @Test
    void testPartOneExample() {
        assertEquals(41, DayEighteen.with(SAMPLE_INPUT).partOne());
    }

    @Test
    void testPartOneFullInput() throws Exception {
        assertEquals(41, DayEighteen.with(fullInput()).partOne());
    }

    @Test
    void testPartTwoExample() {
        assertEquals(42, DayEighteen.with(SAMPLE_INPUT).partTwo());
    }

    @Test
    void testPartTwoWithFullInput() throws Exception {
        assertEquals(42, DayEighteen.with(fullInput()).partTwo());
    }
}