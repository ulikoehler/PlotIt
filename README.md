# PlotIt
Plot everything, using liveplot-rs and Rust.

## Overview

PlotIt is a command-line tool for quickly visualizing data from CSV and Parquet files using interactive plots.

## Features

- 📊 Read CSV and Parquet files
- 🎨 Interactive plotting with liveplot-rs
- 📈 Multiple traces from multi-column data
- 🔍 Zoom, pan, and explore your data
- ⚡ Fast performance with Rust

## Building

```bash
cargo build --release
```

## Usage

```bash
plotit <FILE.csv|FILE.parquet>
```

### Examples

```bash
# Plot a CSV file
plotit data.csv

# Plot a Parquet file
plotit measurements.parquet
```

### Data Format

- The first column is used as the X-axis values
- All remaining columns are plotted as separate traces (Y-axis values)
- All columns must be numeric or convertible to numeric values
- Minimum 2 columns required (X and at least one Y)

### CSV Example

```csv
time,temperature,humidity
0.0,20.5,45.2
1.0,21.0,44.8
2.0,21.5,44.5
```

This will create two traces: "temperature" and "humidity" plotted against "time".

## Dependencies

- [liveplot-rs](https://github.com/ulikoehler/liveplot-rs) - Interactive plotting library
- [eframe](https://github.com/emilk/egui) - GUI framework
- [polars](https://github.com/pola-rs/polars) - Fast DataFrame library for reading CSV/Parquet

## License

See [LICENSE](LICENSE) file for details.
