# Advent of Code

Collection of [Advent of Code](https://adventofcode.com/) solutions written in
Rust. Each year lives in its own package, with a small CLI.

## Getting started

Make sure you have Rust installed. Then clone the repo and you're good to go:

```bash
cargo build
```

## Running Solutions

To run a specific day's solution, you'll need to edit the `main.rs` file of the
corresponding year and call the solve function you want. For example, in
`aoc-<year>/src/main.rs`:

```rust
fn main() {
    solutions::d15::solve_a();
}
```

Then run it with:

```bash
cargo run -p aoc-<year>
```

## CLI

There's a helper CLI that saves you from the boring boilerplate. It has two
commands:

### Creating a new day

```bash
cargo run -p scripts -- new <day>
```

This generates a fresh solution file at `aoc-{year}/src/solutions/d{day}.rs`
with a basic template and registers it in the module. By default it uses the
current year, but you can specify one:

```bash
cargo run -p scripts -- new 7 --year 2023
```

### Downloading input data

```bash
cargo run -p scripts -- data --day <day>
```

This fetches your puzzle input directly from adventofcode.com and saves it to
the right spot. You'll need your session cookie from the site (grab it from your
browser's dev tools after logging in).

The cookie can be provided via the `COOKIE` environment variable, or in an
`.env` file:

```bash
COOKIE=your_cookie cargo run -p scripts -- data --day 7
```

You can also pass the cookie directly with `--cookie`, or specify a different year:

```bash
cargo run -p scripts -- data --cookie abc123 --day 7 --year 2023
```

If you don't pass `--year`, it defaults to the current year.
