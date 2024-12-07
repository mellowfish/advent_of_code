package org.mellowfish.adventofcode.mmxxiv;

import java.math.BigInteger;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

enum Operator {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
    CONCATENATE;

    public BigInteger apply(BigInteger a, BigInteger b) {
        return switch (this) {
            case ADD -> a.add(b);
            case SUBTRACT -> a.subtract(b);
            case MULTIPLY -> a.multiply(b);
            case DIVIDE -> a.divide(b);
            case CONCATENATE -> new BigInteger(a.toString() + b.toString());
        };
    }
}

record PartialEquation(BigInteger expectedResult, List<BigInteger> operands, List<Operator> operators) {
    public boolean evalute() {
        if (!complete()) {
            throw new RuntimeException("Evaluate called on incomplete equation");
        }

        BigInteger calculatedResult = operands.getFirst();
        for (int i = 0; i < operators.size(); i++) {
            calculatedResult = operators.get(i).apply(calculatedResult, operands.get(i + 1));
        }
        return calculatedResult.equals(expectedResult);
    }

    public boolean complete() {
        return operands.size() - operators.size() == 1;
    }

    public boolean canBeSolvedWith(List<Operator> possibleOperators) {
        if (complete()) {
            return evalute();
        }

        return possibleOperators.stream().anyMatch(operator -> {
            List<Operator> newOperators = new ArrayList<>(operators);
            newOperators.add(operator);
            return new PartialEquation(expectedResult, operands, newOperators).canBeSolvedWith(possibleOperators);
        });
    }
}

public class DaySeven {
    private final String input;
    private final ArrayList<PartialEquation> equations;

    public static DaySeven with(String input) {
        return new DaySeven(input);
    }

    DaySeven(String input) {
        this.input = input;
        this.equations = new ArrayList<>();
        parseInput();
    }

    public BigInteger partOne() {
        return equations.stream()
                .filter(equation -> equation.canBeSolvedWith(List.of(Operator.ADD, Operator.MULTIPLY)))
                .map(PartialEquation::expectedResult)
                .reduce(BigInteger.ZERO, BigInteger::add);
    }

    public BigInteger partTwo() {
        return equations.stream()
                .filter(equation ->
                        equation.canBeSolvedWith(List.of(Operator.ADD, Operator.MULTIPLY, Operator.CONCATENATE)))
                .map(PartialEquation::expectedResult)
                .reduce(BigInteger.ZERO, BigInteger::add);
    }

    void parseInput() {
        input.lines().forEach(line -> {
            String[] halves = line.split(": ");
            BigInteger result = new BigInteger(halves[0]);
            List<BigInteger> operands =
                    Arrays.stream(halves[1].split(" ")).map(BigInteger::new).toList();

            equations.add(new PartialEquation(result, operands, List.of()));
        });
    }
}
