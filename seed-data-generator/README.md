# Seed Data Generator

A CLI tool for generating seed data for our application.

The generator produces deterministic output, meaning that using the same seed and arguments will produce the same dataset.

## Usage

To generate seed data, run:

```bash
cargo run -- --seed <seed> --min <min> --max <max> --start-date <YYYY-MM-DD> --output <file_name>.<file_ext>
```

### Arguments

| Argument | Description |
|---|---|
| `--seed` | Seed used for random data generation. |
| `--min` | Minimum value for data generation. |
| `--max` | Maximum value for data generation. |
| `--start-date` | Starting date in `YYYY-MM-DD` format. |
| `--output` | Output file name and extension. |