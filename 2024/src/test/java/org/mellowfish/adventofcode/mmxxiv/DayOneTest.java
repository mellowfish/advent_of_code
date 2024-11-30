package org.mellowfish.adventofcode.mmxxiv;

import static org.junit.jupiter.api.Assertions.*;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;

class DayOneTest {
    String fullInput() throws IOException {
        return Files.readString(Paths.get("../resources/day_one_full_input.txt"));
    }

    void testPartOne() {
        assertEquals(42, DayOne.with("""
                """).partOne());
    }

    void testPartTwo() throws Exception {
        assertEquals(42, DayOne.with(fullInput()).partTwo());
    }
}
