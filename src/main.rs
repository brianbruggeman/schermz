use std::fs;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the JSON file
    file: String,
    /// Whether to merge object types into one
    #[arg(short, long)]
    merge_objects: bool,
}

fn main() {
    let args = Args::parse();
    let data = fs::read_to_string(args.file).expect("Unable to read file");
    let json: serde_json::Value = serde_json::from_str(&data).expect("Invalid JSON");
    let schema = schermz::Schema::from_json(&json, args.merge_objects);
    let pretty = serde_json::to_string_pretty(&schema.to_json()).unwrap();
    println!("{}", pretty);
}
