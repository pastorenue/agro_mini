use clap::{Arg, Command, Parser, Subcommand};


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct AgroCLI {
    #[arg(short, long)]
    name: String,
    #[arg(short, long)]
    is_good: Option<bool>,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
   Crop {

       #[arg(short, long)]
       list: bool,

       #[arg(short, long)]
       get: Option<String>,
   }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FarmLand {
    width: u32,
    length: u32,
    soil_type: String,
    crops: Vec<String>,
}
