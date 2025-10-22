# VS Code Problems Filtering

A Rust CLI application to filter problems exported from VS Code's Problems view.

## Features

- Filter by inclusion terms (all must be present)
- Filter by exclusion terms (none must be present)
- Case-insensitive comparison support
- Formatted table display
- Robust error handling

## Installation

```bash
cargo build --release
```

## Usage

```bash
# Display help
cargo run -- --help

# Filter problems containing "deprecated"
cargo run -- -f deprecated.json -i "deprecated"

# Filter problems containing "deprecated" but excluding "ActionError"
cargo run -- -f deprecated.json -i "deprecated" -e "ActionError"

# Case-insensitive filtering
cargo run -- -f deprecated.json -i "DEPRECATED" --ignore-case

# Multiple inclusion and exclusion terms
cargo run -- -f deprecated.json -i "deprecated" -i "warning" -e "test" -e "mock"

# Count results only (without displaying table)
cargo run -- -f deprecated.json -i "deprecated" --count-only

# JSON output
cargo run -- -f deprecated.json -i "constructor" -e "sonarqube" --json
```

## Options

- `-f, --input <FILE>`: Input JSON file (required)
- `-i, --include <TERM>`: Term to include (repeatable)
- `-e, --exclude <TERM>`: Term to exclude (repeatable)
- `--ignore-case`: Ignore case in comparisons
- `-c, --count-only`: Display only the number of results
- `--json`: Output in JSON format

## Input File Format

The JSON file must contain an array of objects representing VS Code problems, with at least these fields:
- `resource`: file path
- `message`: problem message
- `startLineNumber`: line number

## Tests

```bash
cargo test
```

## Code Coverage

This project is configured with `cargo-llvm-cov` for code coverage and VS Code Coverage Gutters extension for line-by-line display.

### Generating Reports

```bash
# Convenience script (recommended)
./coverage.sh all          # Generate all reports
./coverage.sh summary      # Display summary
./coverage.sh html        # HTML report only
./coverage.sh lcov        # LCOV file for VS Code

# Direct cargo commands
cargo llvm-cov --html --output-dir target/coverage
cargo llvm-cov --lcov --output-path target/coverage/lcov.info
cargo llvm-cov --summary-only
```

### Display in VS Code

1. Generate the LCOV file: `./coverage.sh lcov`
2. Open a Rust file in VS Code
3. Use the "Coverage Gutters: Display Coverage" command (Ctrl+Shift+P)
4. Covered/uncovered lines will appear with colors in the editor

### VS Code Tasks

Use `Ctrl+Shift+P` > "Tasks: Run Task" and select:
- **Coverage: Complete Report (HTML + LCOV)** - Generate all reports (default)
- **Coverage: Generate HTML Report** - HTML report
- **Coverage: Generate LCOV Report** - LCOV file for the extension
- **Coverage: Show Summary** - Summary in terminal

## Libraries Used

- `clap` - CLI argument parsing with derive macros
- `serde` & `serde_json` - JSON serialization/deserialization
- `anyhow` - Ergonomic error handling
- `tabled` - Formatted table display

## License

MIT
