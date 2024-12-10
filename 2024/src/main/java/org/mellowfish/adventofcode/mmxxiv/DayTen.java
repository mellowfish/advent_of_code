package org.mellowfish.adventofcode.mmxxiv;

import java.util.*;
import org.mellowfish.adventofcode.mmxxiv.shared.Point;

public class DayTen {
    private int height;
    private int width;
    private final Map<Point, Integer> map;
    private final List<Point> trailheads;

    public static DayTen with(String input) {
        return new DayTen(input);
    }

    DayTen(String input) {
        map = new HashMap<>();
        trailheads = new ArrayList<>();

        parseInput(input);
    }

    public long partOne() {
        return trailheads.stream().mapToLong(this::score).sum();
    }

    public long partTwo() {
        return trailheads.stream().mapToLong(this::rating).sum();
    }

    private long score(Point trailhead) {
        ArrayList<List<Point>> trails = new ArrayList<>();
        trailhead
                .cardinalNeighbors()
                .forEach(nextPoint -> trails.addAll(findTrailEnds(List.of(trailhead), nextPoint, 0)));
        return trails.stream().map(List::getLast).distinct().count();
    }

    private long rating(Point trailhead) {
        ArrayList<List<Point>> trails = new ArrayList<>();
        trailhead
                .cardinalNeighbors()
                .forEach(nextPoint -> trails.addAll(findTrailEnds(List.of(trailhead), nextPoint, 0)));
        return trails.stream().distinct().count();
    }

    private List<List<Point>> findTrailEnds(List<Point> trail, Point point, int previousHeight) {
        Integer elevation = map.get(point);
        if (elevation == null || elevation < 0) {
            return List.of();
        }
        if (elevation - previousHeight != 1) {
            return List.of();
        }
        List<Point> newTrail = extendTrail(trail, point);
        if (elevation == 9) {
            return List.of(newTrail);
        }

        ArrayList<List<Point>> trailEnds = new ArrayList<>();
        point.cardinalNeighbors().forEach(nextPoint -> trailEnds.addAll(findTrailEnds(newTrail, nextPoint, elevation)));
        return trailEnds;
    }

    private List<Point> extendTrail(List<Point> trail, Point point) {
        ArrayList<Point> newTrail = new ArrayList<>(trail);
        newTrail.add(point);
        return newTrail;
    }

    private void parseInput(String input) {
        List<String> lines = input.lines().toList();
        height = lines.size();
        width = lines.getFirst().length();

        for (int y = 0; y < height; y++) {
            String line = lines.get(y);
            for (int x = 0; x < width; x++) {
                Point point = new Point(x, y);
                String node = line.substring(x, x + 1);
                int elevation;
                if (node.equals(".")) {
                    elevation = -1;
                } else {
                    elevation = Integer.parseInt(node);
                }
                map.put(point, elevation);
                if (elevation == 0) {
                    trailheads.add(point);
                }
            }
        }
    }
}
