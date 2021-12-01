# Advent of Code 2021

Rules for this year:

- Use latest ruby (3.0.3)
- Use no gems other than: 
  - `pry` for debugging
  - `ramsey_cop` for style
  - `rspec` for tests

  I am specifically trying to avoid using `dry-*`, `active-*`, etc as crutches.
  Go from first principles instead of using libraries!
- TDD (as much as possible!)
- No copy-pasting code, extract immediately!
- One namespace (`AdventOfCode`), for easier code sharing
- Name everything at every level of abstraction
  - absolutely microscopic methods
  - local variables for intermediate steps
  - obnoxiously specific names are fine if they make things clearer
