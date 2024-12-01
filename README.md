# Advent of Code

## Usage

`
$ cargo run --bin $DAY $PART
`

Replace `$DAY` with a day (in the format of XX), and $PART with a
part (in the format of X)

The only valid parts are `1` and `2`, not including a part runs both.

```
# Examples
$ cargo run --bin 01 1    # Runs Part 1 of Day 1
$ cargo run --bin 02 1    # Runs Part 2 of Day 2
$ cargo run --bin 01 2    # Runs Part 2 of Day 1
$ cargo run --bin 01      # Runs Day 1, Parts 1 and 2
```

When using an IDE, `default-run` in Cargo.toml may be adjusted to the current day.