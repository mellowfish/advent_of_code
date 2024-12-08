package org.mellowfish.adventofcode.mmxxiv;

import java.util.*;
import java.util.concurrent.atomic.AtomicInteger;
import org.mellowfish.adventofcode.mmxxiv.shared.Point;

// Possible optimization: fork simulations instead of completely re-running them!
public class DaySix {
    enum Direction {
        NORTH('^'),
        EAST('>'),
        SOUTH('v'),
        WEST('<');

        final char value;

        Direction(char value) {
            this.value = value;
        }

        @Override
        public String toString() {
            return String.valueOf(value);
        }

        public Point move(Point location) {
            return switch (this) {
                case NORTH -> new Point(location.x(), location.y() - 1);
                case EAST -> new Point(location.x() + 1, location.y());
                case SOUTH -> new Point(location.x(), location.y() + 1);
                case WEST -> new Point(location.x() - 1, location.y());
            };
        }

        public Direction pivotRight() {
            return switch (this) {
                case NORTH -> EAST;
                case EAST -> SOUTH;
                case SOUTH -> WEST;
                case WEST -> NORTH;
            };
        }
    }

    record Guard(Point location, Direction direction) {
        public Point nextStep() {
            return direction.move(location);
        }

        public Guard pivotRight() {
            return new Guard(location, direction.pivotRight());
        }

        public Guard goForward() {
            return new Guard(nextStep(), direction);
        }
    }

    class Simulation {
        private final int width;
        private final int height;
        private final List<Point> obstacles;
        private final HashMap<Point, ArrayList<Direction>> visitedLocations;
        private Guard guard;

        Simulation(int width, int height, List<Point> obstacles, Guard guard) {
            this.width = width;
            this.height = height;
            this.obstacles = obstacles;
            this.guard = guard;

            this.visitedLocations = new HashMap<>();
        }

        public Simulation(Simulation simulation) {
            this(simulation.width, simulation.height, simulation.obstacles, simulation.guard);
        }

        public Map<Point, ArrayList<Direction>> run() {
            while (guardIsOnMap()) {
                move();
            }
            return visitedLocations;
        }

        public Point getGuardLocation() {
            return guard.location();
        }

        public Simulation withObstacle(Point point) {
            ArrayList<Point> newObstacles = new ArrayList<>(obstacles);
            newObstacles.add(point);
            return new Simulation(width, height, newObstacles, guard);
        }

        boolean guardIsOnMap() {
            return this.guard.location().x() >= 0
                    && this.guard.location().x() < width
                    && this.guard.location().y() >= 0
                    && this.guard.location().y() < height;
        }

        void move() {
            ArrayList<Direction> previousVisits =
                    visitedLocations.computeIfAbsent(guard.location(), k -> new ArrayList<>());
            if (previousVisits.contains(guard.direction())) {
                throw new RuntimeException("Loop detected!");
            }
            previousVisits.add(guard.direction());

            Point nextStep = this.guard.nextStep();
            if (obstacles.contains(nextStep)) {
                this.guard = this.guard.pivotRight();
            } else {
                this.guard = this.guard.goForward();
            }
        }

        void print() {
            System.out.println();
            for (int y = 0; y < height; y++) {
                for (int x = 0; x < width; x++) {
                    Point point = new Point(x, y);
                    if (guard.location().equals(point)) {
                        System.out.print(guard.direction());
                    } else if (obstacles.contains(point)) {
                        System.out.print("#");
                    } else if (visitedLocations.containsKey(point)) {
                        System.out.print("X");
                    } else {
                        System.out.print(".");
                    }
                }
                System.out.println();
            }
        }
    }

    private final Simulation simulation;

    public static DaySix with(String input) {
        return new DaySix(input);
    }

    DaySix(String input) {
        this.simulation = parseInput(input);
    }

    public int partOne() {
        return simulation.run().size();
    }

    public int partTwo() {
        Simulation original = new Simulation(simulation);
        AtomicInteger totalPossibilities = new AtomicInteger();
        simulation.run().forEach((location, directions) -> {
            if (location != original.getGuardLocation()) {
                try {
                    original.withObstacle(location).run();
                } catch (Exception e) {
                    totalPossibilities.incrementAndGet();
                }
            }
        });
        return totalPossibilities.get();
    }

    Simulation parseInput(String input) {
        List<String> lines = input.lines().toList();
        ArrayList<Point> obstacles = new ArrayList<>();
        int height = lines.size();
        int width = lines.getFirst().length();
        Guard guard = null;

        for (int y = 0; y < height; y++) {
            String line = lines.get(y);
            for (int x = 0; x < width; x++) {
                char character = line.charAt(x);
                if (character == '.') {
                    continue;
                }
                Point point = new Point(x, y);
                if (character == '#') {
                    obstacles.add(point);
                } else {
                    for (Direction dir : Direction.values()) {
                        if (character == dir.value) {
                            if (guard != null) {
                                throw new IllegalArgumentException("Found multiple guards!");
                            }
                            guard = new Guard(point, dir);
                        }
                    }
                }
            }
        }

        return new Simulation(width, height, obstacles, guard);
    }
}
