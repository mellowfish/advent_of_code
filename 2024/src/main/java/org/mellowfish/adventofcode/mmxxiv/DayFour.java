package org.mellowfish.adventofcode.mmxxiv;

import java.util.HashMap;
import java.util.List;
import org.mellowfish.adventofcode.mmxxiv.shared.Point;

public class DayFour {
    private final String input;
    private final HashMap<Point, Character> board;
    private final int height;
    private final int width;

    public static DayFour with(String input) {
        return new DayFour(input);
    }

    DayFour(String input) {
        this.input = input;
        board = new HashMap<>();

        List<String> lines = input.lines().toList();
        height = lines.size();
        width = lines.get(0).length();

        parseInput(lines);
    }

    public int partOne() {
        int total = 0;
        for (int y = 0; y < height; y++) {
            for (int x = 0; x < width; x++) {
                total += countAllXMASes(x, y);
            }
        }
        return total;
    }

    public int partTwo() {
        int total = 0;
        for (int y = 0; y < height; y++) {
            for (int x = 0; x < width; x++) {
                if (isX_MAS(x, y)) {
                    total += 1;
                }
            }
        }
        return total;
    }

    private void parseInput(List<String> lines) {
        for (int y = 0; y < height; y++) {
            String line = lines.get(y);
            for (int x = 0; x < width; x++) {
                board.put(new Point(x, y), line.charAt(x));
            }
        }
    }

    private int countAllXMASes(int x, int y) {
        int total = 0;
        if (!checkOne('X', x, y)) {
            return total;
        }
        // Left
        if (checkOne('M', x - 1, y) && checkOne('A', x - 2, y) && checkOne('S', x - 3, y)) {
            total += 1;
        }
        // Right
        if (checkOne('M', x + 1, y) && checkOne('A', x + 2, y) && checkOne('S', x + 3, y)) {
            total += 1;
        }
        // Up
        if (checkOne('M', x, y - 1) && checkOne('A', x, y - 2) && checkOne('S', x, y - 3)) {
            total += 1;
        }
        // Down
        if (checkOne('M', x, y + 1) && checkOne('A', x, y + 2) && checkOne('S', x, y + 3)) {
            total += 1;
        }
        // Left, Up
        if (checkOne('M', x - 1, y - 1) && checkOne('A', x - 2, y - 2) && checkOne('S', x - 3, y - 3)) {
            total += 1;
        }
        // Left, Down
        if (checkOne('M', x - 1, y + 1) && checkOne('A', x - 2, y + 2) && checkOne('S', x - 3, y + 3)) {
            total += 1;
        }
        // Right, Up
        if (checkOne('M', x + 1, y - 1) && checkOne('A', x + 2, y - 2) && checkOne('S', x + 3, y - 3)) {
            total += 1;
        }
        // Right, Down
        if (checkOne('M', x + 1, y + 1) && checkOne('A', x + 2, y + 2) && checkOne('S', x + 3, y + 3)) {
            total += 1;
        }

        return total;
    }

    private boolean isX_MAS(int x, int y) {
        if (!checkOne('A', x, y)) {
            return false;
        }
        return ((checkOne('M', x - 1, y - 1) && checkOne('S', x + 1, y + 1))
                        || (checkOne('S', x - 1, y - 1) && checkOne('M', x + 1, y + 1)))
                && ((checkOne('M', x - 1, y + 1) && checkOne('S', x + 1, y - 1))
                        || (checkOne('S', x - 1, y + 1) && checkOne('M', x + 1, y - 1)));
    }

    boolean checkOne(Character targetLetter, int x, int y) {
        return targetLetter.equals(get(x, y));
    }

    private Character get(int x, int y) {
        return board.get(new Point(x, y));
    }
}
