package org.mellowfish.adventofcode.mmxxiv.shared;

import static org.mellowfish.adventofcode.mmxxiv.shared.Utilities.gcd;

import java.util.List;

public record Point(int x, int y) {
    public boolean isWithin(int width, int height) {
        return 0 <= x && x < width && 0 <= y && y < height;
    }

    public Point smallestVectorTo(Point other) {
        int deltaX = x - other.x;
        int deltaY = y - other.y;
        int sign = (deltaX / Math.abs(deltaX)) * (deltaY / Math.abs(deltaY));
        deltaX = Math.abs(deltaX);
        deltaY = Math.abs(deltaY);
        int divisor = gcd(Math.max(deltaX, deltaY), Math.min(deltaX, deltaY));

        return new Point(sign * (deltaX / divisor), (deltaY / divisor));
    }

    public Point plus(Point vector) {
        return new Point(x + vector.x, y + vector.y);
    }

    public Point minus(Point vector) {
        return new Point(x - vector.x, y - vector.y);
    }

    public List<Point> cardinalNeighbors() {
        return List.of(new Point(x, y - 1), new Point(x + 1, y), new Point(x, y + 1), new Point(x - 1, y));
    }
}
