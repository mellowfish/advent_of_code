package org.mellowfish.adventofcode.mmxxiv;

import java.util.ArrayList;
import java.util.List;
import java.util.Scanner;
import lombok.AllArgsConstructor;

@AllArgsConstructor
public class DayOne {
    private final String input;
    private final List<Integer> listOne;
    private final List<Integer> listTwo;

    public static DayOne with(String input) {
        return new DayOne(input);
    }

    DayOne(String input) {
        this.input = input;
        this.listOne = new ArrayList<>();
        this.listTwo = new ArrayList<>();
        extractLists();
    }

    public int partOne() {
        var iteratorOne = listOne.iterator();
        var iteratorTwo = listTwo.iterator();
        int difference = 0;
        int valueOne;
        int valueTwo;
        while (iteratorOne.hasNext() && iteratorTwo.hasNext()) {
            valueOne = iteratorOne.next();
            valueTwo = iteratorTwo.next();
            difference += Math.abs(valueOne - valueTwo);
        }

        return difference;
    }

    public int partTwo() {
        return listOne.stream()
                .reduce(
                        0,
                        (similarity, valueOne) -> similarity
                                + listTwo.stream().reduce(0, (acc, valueTwo) -> {
                                    if (valueOne.equals(valueTwo)) {
                                        return acc + valueOne;
                                    }
                                    return acc;
                                }));
    }

    private void extractLists() {
        Scanner scanner = new Scanner(input);
        while (scanner.hasNextLine()) {
            String line = scanner.nextLine();
            String[] parts = line.split("\s+");
            listOne.add(Integer.parseInt(parts[0]));
            listTwo.add(Integer.parseInt(parts[1]));
        }
        listOne.sort(Integer::compareTo);
        listTwo.sort(Integer::compareTo);
    }
}
