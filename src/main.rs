use liveplot::{channel_plot, LivePlotApp};
use std::env;
use std::path::Path;
use polars::prelude::*;

fn read_csv_data(path: &Path) -> Result<Vec<(String, Vec<[f64; 2]>)>, Box<dyn std::error::Error>> {
    let df = CsvReadOptions::default()
        .try_into_reader_with_file_path(Some(path.to_path_buf()))?
        .finish()?;
    
    dataframe_to_traces(df)
}

fn read_parquet_data(path: &Path) -> Result<Vec<(String, Vec<[f64; 2]>)>, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(path)?;
    let df = ParquetReader::new(file).finish()?;
    
    dataframe_to_traces(df)
}

fn dataframe_to_traces(df: DataFrame) -> Result<Vec<(String, Vec<[f64; 2]>)>, Box<dyn std::error::Error>> {
    if df.width() < 2 {
        return Err("File must have at least 2 columns".into());
    }
    
    // Get first column as X values
    let x_column = &df.get_columns()[0];
    let x_values: Vec<f64> = x_column.cast(&DataType::Float64)?
        .f64()?
        .into_iter()
        .filter_map(|v| v)
        .collect();
    
    // Create traces for remaining columns
    let mut traces = Vec::new();
    
    for column in df.get_columns().iter().skip(1) {
        let trace_name = column.name().to_string();
        
        // Convert column to f64
        let y_values: Vec<f64> = column.cast(&DataType::Float64)?
            .f64()?
            .into_iter()
            .filter_map(|v| v)
            .collect();
        
        // Create trace data
        let mut trace_data = Vec::new();
        for (i, &y_val) in y_values.iter().enumerate() {
            if i < x_values.len() {
                trace_data.push([x_values[i], y_val]);
            }
        }
        
        traces.push((trace_name, trace_data));
    }
    
    Ok(traces)
}

struct PlotItApp {
    plot: LivePlotApp,
    title: String,
}

impl PlotItApp {
    fn new(filename: &str, traces: Vec<(String, Vec<[f64; 2]>)>) -> Self {
        // Create a plot app with an unused channel (no live data needed)
        let (_sink, rx) = channel_plot();
        let mut plot = LivePlotApp::new(rx);
        plot.time_window = 10.0;
        plot.max_points = 100_000;
        plot.show_legend = true;
        
        // Set trace data using the new API
        for (name, data) in traces {
            plot.set_trace_data(&name, data);
        }
        
        Self {
            plot,
            title: format!("PlotIt - {}", filename),
        }
    }
}

impl eframe::App for PlotItApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.heading(&self.title);
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            self.plot.ui_embed(ui);
        });
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: plotit <FILE.csv|FILE.parquet>");
        std::process::exit(1);
    }
    
    let filepath = &args[1];
    let path = Path::new(filepath);
    
    if !path.exists() {
        eprintln!("Error: File '{}' does not exist", filepath);
        std::process::exit(1);
    }
    
    let extension = path.extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");
    
    let traces = match extension.to_lowercase().as_str() {
        "csv" => read_csv_data(path)?,
        "parquet" => read_parquet_data(path)?,
        _ => {
            eprintln!("Error: Unsupported file format. Use .csv or .parquet");
            std::process::exit(1);
        }
    };
    
    if traces.is_empty() {
        eprintln!("Error: No data traces found in file");
        std::process::exit(1);
    }
    
    // Print data summary
    println!("Loaded {} trace(s):", traces.len());
    for (name, data) in &traces {
        println!("  - {}: {} data points", name, data.len());
    }
    
    let filename = path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(filepath);
    
    let app = PlotItApp::new(filename, traces);
    
    let title = format!("PlotIt - {}", filename);
    eframe::run_native(
        &title,
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(app))),
    )?;
    
    Ok(())
}
