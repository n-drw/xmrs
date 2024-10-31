use clap::Parser;
use xmrs::it::it_module::ItModule;

#[derive(Parser)]
struct Cli {
    #[arg(
        short = 'f',
        long,
        default_value = "example.it",
        value_name = "filename"
    )]
    filename: Option<String>,

    /// Turn pattern informations on
    #[arg(short = 'p', long, default_value = "false")]
    patterns: bool,
}

fn main() -> Result<(), std::io::Error> {
    let cli = Cli::parse();

    match cli.filename {
        Some(filename) => {
            println!("--===~ XmRs It Module Info Example ~===--");
            println!("(c) 2024 Sébastien Béchet\n");
            println!("opening {}", filename);
            let contents = std::fs::read(filename.trim())?;
            match ItModule::load(&contents) {
                Ok(it) => {
                    println!("{:?}", it);
                    // let module = it.to_module();
                    // println!("{:?}", module);
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
