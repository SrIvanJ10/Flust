use clap::{Parser, Subcommand};
use flust_core::parser;
use flust_codegen::generator;
use std::path::PathBuf;
use std::fs;
use std::process::Command;
use anyhow::Result;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile a flow file
    Compile {
        /// Input flow file (YAML/JSON)
        #[arg(short, long)]
        input: PathBuf,

        /// Output directory for the generated project
        #[arg(short, long)]
        output: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Compile { input, output } => {
            println!("Compiling {:?} to {:?}", input, output);

            // 1. Parse Flow
            let flow = parser::parse_file(input)?;
            println!("Parsed flow with {} nodes", flow.nodes.len());

            // 2. Generate Code
            let rust_code = generator::generate_rust(&flow)?;

            // 3. Create Output Project
            if !output.exists() {
                fs::create_dir_all(output)?;
                Command::new("cargo")
                    .arg("init")
                    .arg("--bin")
                    .current_dir(output)
                    .output()?;
            }

            // 4. Write Main.rs
            let main_rs = output.join("src").join("main.rs");
            fs::write(main_rs, rust_code)?;

            println!("Compilation successful!");
        }
    }

    Ok(())
}
