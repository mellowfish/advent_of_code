package org.mellowfish.adventofcode.mmxxiv;

import java.util.*;

public class DayTwo {
    private final String input;
    private final List<Report> reports;

    class Report {
        private final List<Integer> data;
        private final List<Integer> deltas;

        public Report(List<Integer> data) {
            this.data = data;
            this.deltas = new ArrayList<>();
            populateDeltas();
        }

        public boolean isSafeWithDampening() {
            if (isSafe()) {
                return true;
            }

            for (int i = 0; i < data.size(); i++) {
                ArrayList<Integer> alternateData = new ArrayList<>(data);
                alternateData.remove(i);
                if (new Report(alternateData).isSafe()) {
                    return true;
                }
            }

            return false;
        }

        public boolean isSafe() {
            //            System.out.println(data);
            //            System.out.printf("Increasing: %b%n", isIncreasing());
            //            System.out.printf("Decreasing: %b%n", isDecreasing());
            //            System.out.printf("Deltas: %d %d%n", minDelta(), maxDelta());
            return (isIncreasing() || isDecreasing()) && 1 <= minDelta() && maxDelta() <= 3;
        }

        boolean isIncreasing() {
            return deltas.stream().allMatch(delta -> delta > 0);
        }

        boolean isDecreasing() {
            return deltas.stream().allMatch(delta -> delta < 0);
        }

        int minDelta() {
            return deltas.stream().map(Math::abs).min(Integer::compareTo).get();
        }

        int maxDelta() {
            return deltas.stream().map(Math::abs).max(Integer::compareTo).get();
        }

        void populateDeltas() {
            for (int i = 1; i < data.size(); i++) {
                deltas.add(data.get(i) - data.get(i - 1));
            }
        }
    }

    public static DayTwo with(String input) {
        return new DayTwo(input);
    }

    public DayTwo(String input) {
        this.input = input;
        this.reports = new ArrayList<>();
        parseInput();
    }

    public long partOne() {
        return reports.stream().filter(Report::isSafe).count();
    }

    public long partTwo() {
        return reports.stream().filter(Report::isSafeWithDampening).count();
    }

    private void parseInput() {
        Scanner scanner = new Scanner(input);
        while (scanner.hasNextLine()) {
            String line = scanner.nextLine();
            reports.add(new Report(
                    Arrays.stream(line.split(" ")).map(Integer::parseInt).toList()));
        }
    }
}
