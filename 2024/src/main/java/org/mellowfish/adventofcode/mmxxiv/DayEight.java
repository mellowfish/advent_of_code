package org.mellowfish.adventofcode.mmxxiv;

import java.util.*;
import org.mellowfish.adventofcode.mmxxiv.shared.Point;

public class DayEight {
    private int height;
    private int width;
    private final HashMap<Character, List<Point>> locationsForAntennas;
    private final HashMap<Point, Set<Character>> antiNodes;

    public static DayEight with(String input) {
        return new DayEight(input);
    }

    DayEight(String input) {
        locationsForAntennas = new HashMap<>();
        antiNodes = new HashMap<>();
        parseInput(input);
    }

    public int partOne() {
        locationsForAntennas.forEach((frequency, points) -> {
            for (int i = 0; i < points.size(); i++) {
                Point a = points.get(i);
                for (int j = i + 1; j < points.size(); j++) {
                    Point b = points.get(j);
                    Point deltaVector = a.minus(b);

                    registerAntiNode(frequency, a.plus(deltaVector));
                    registerAntiNode(frequency, b.minus(deltaVector));
                }
            }
        });
        return antiNodes.size();
    }

    public int partTwo() {
        locationsForAntennas.forEach((frequency, points) -> {
            for (int i = 0; i < points.size(); i++) {
                Point a = points.get(i);
                for (int j = i + 1; j < points.size(); j++) {
                    Point smallestVector = a.smallestVectorTo(points.get(j));
                    Point antiNode = a.minus(smallestVector);
                    while (registerAntiNode(frequency, antiNode)) {
                        antiNode = antiNode.minus(smallestVector);
                    }
                    antiNode = a;
                    while (registerAntiNode(frequency, antiNode)) {
                        antiNode = antiNode.plus(smallestVector);
                    }
                }
            }
        });
        return antiNodes.size();
    }

    private boolean registerAntiNode(Character frequency, Point antiNode) {
        if (!antiNode.isWithin(width, height)) {
            return false;
        }
        antiNodes.computeIfAbsent(antiNode, k -> new HashSet<>()).add(frequency);
        return true;
    }

    private void parseInput(String input) {
        List<String> lines = input.lines().toList();
        height = lines.size();
        width = lines.getFirst().length();

        for (int y = 0; y < height; y++) {
            String line = lines.get(y);
            for (int x = 0; x < width; x++) {
                char character = line.charAt(x);
                if (character == '.') {
                    continue;
                }
                Point point = new Point(x, y);
                if (validFrequency(character)) {
                    locationsForAntennas
                            .computeIfAbsent(character, k -> new ArrayList<Point>())
                            .add(point);
                } else {
                    throw new RuntimeException("wat");
                }
            }
        }
    }

    private boolean validFrequency(Character frequency) {
        return ('0' <= frequency && frequency <= '9')
                || ('a' <= frequency && frequency <= 'z')
                || ('A' <= frequency && frequency <= 'Z');
    }
}
