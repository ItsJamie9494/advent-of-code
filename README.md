# Advent of Code

## Usage

`
$ cargo run -- [COMMAND] (DAY) (ARGS)
`

If a day is not specified, the current date is used.

Valid Commands & Args:

- `run` - Run a day
    - `part`: A valid part, either `1` or `2`. Not specifying or specifying an invalid value runs all parts.
- `scaffold` - Create a new day

```
# Examples
$ cargo run -- run 01 1     # Runs Part 1 of Day 1
$ cargo run -- run          # Runs todays code
$ cargo run -- run 03       # Runs all parts of day 3

$ cargo run -- scaffold     # Create a new project for today
$ cargo run -- scaffold 04  # Create a new project for Day 4
```