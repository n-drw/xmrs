use clap::Parser;
use xmrs::prelude::*;

#[derive(Parser)]
struct Cli {
    #[arg(short = 'f', long, required = true, value_name = "filename")]
    filename: Option<String>,

    /// Turn pattern informations on
    #[arg(short = 'p', long, default_value = "false")]
    patterns: bool,
}

fn main() -> Result<(), std::io::Error> {
    let cli = Cli::parse();

    match cli.filename {
        Some(filename) => {
            println!("--===~ XmRs Module Info Example ~===--");
            println!("(c) 2024 Sébastien Béchet\n");
            println!("opening {}", filename);
            let contents = std::fs::read(filename.trim())?;
            match Module::load(&contents) {
                Ok(module) => {
                    println!("{:?}", module);
                }
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }
        _ => {}
    }
    Ok(())
}
