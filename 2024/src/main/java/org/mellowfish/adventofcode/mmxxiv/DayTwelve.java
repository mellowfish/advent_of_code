package org.mellowfish.adventofcode.mmxxiv;

import org.mellowfish.adventofcode.mmxxiv.shared.Point;

import java.util.*;

public class DayTwelve {
    record Region(Character type, ArrayList<Point> plots) {
        Region(Character type, Point firstPlot) {
            this(type, new ArrayList<>());
            plots.add(firstPlot);
        }

        long cost() {
            return area() * perimeter();
        }

        long discountCost() {
            return area() * sides();
        }

        private long sides() {
            return -1; // TODO
        }

        private long perimeter() {
            return plots.stream().mapToLong(point -> {
                return point.cardinalNeighbors().stream().filter(neighbor -> !plots.contains(neighbor)).count();
            }).sum();
        }

        private long area() {
            return plots.size();
        }

        void add(Point point) {
            plots.add(point);
        }

        public boolean contains(Point neighbor) {
            return plots.contains(neighbor);
        }
    }

    private final HashMap<Point, Character> gardenPlots;
    private final HashMap<Point, Region> gardenRegions;
    private final List<Region> regions;

    public static DayTwelve with(String input) {
        return new DayTwelve(input);
    }

    DayTwelve(String input) {
        gardenPlots = new HashMap<>();
        gardenRegions = new HashMap<>();
        regions = new ArrayList<>();

        parseInput(input);
        findRegions();
    }

    public long partOne() {
        System.out.println(Arrays.toString(regions.stream().mapToLong(Region::cost).toArray()));
        return regions.stream().mapToLong(Region::cost).sum();
    }

    public long partTwo() {
        return regions.stream().mapToLong(Region::discountCost).sum();
    }

    void findRegions() {
        for (Map.Entry<Point, Character> entry : gardenPlots.entrySet()) {
            Point point = entry.getKey();
            if (gardenRegions.containsKey(point)) {
                continue;
            }
            Region region = new Region(entry.getValue(), point);
            regions.add(region);
            gardenRegions.put(point, region);

            expandRegion(region, point);
        }
    }

    private void expandRegion(Region region, Point point) {
        point.cardinalNeighbors().forEach(neighbor -> {
            Character type = gardenPlots.get(neighbor);
            if (type == region.type && !region.contains(neighbor)) {
                gardenRegions.put(neighbor, region);
                region.add(neighbor);
                expandRegion(region, neighbor);
            }
        });
    }

    void parseInput(String input) {
        List<String> lines = input.lines().toList();
        int height = lines.size();
        int width = lines.getFirst().length();
        for (int y = 0; y < height; y++) {
            String line = lines.get(y);
            for (int x = 0; x < width; x++) {
                gardenPlots.put(new Point(x, y), line.charAt(x));
            }
        }
    }
}
