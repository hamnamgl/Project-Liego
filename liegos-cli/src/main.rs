use anyhow::Result;
use clap::{Parser, Subcommand};
use liegos_core::{decode_meaning, encode_meaning};

#[derive(Debug, Parser)]
#[command(name = "liegos")]
#[command(about = "Ping meaning packets from the command line")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Ping {
        text: String,
        #[arg(short, long, default_value = "default")]
        ontology: String,
    },
    Decode {
        json: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Ping { text, ontology } => {
            let packet = encode_meaning(ontology, &text)?;
            println!("{}", serde_json::to_string_pretty(&packet)?);
        }
        Command::Decode { json } => {
            let packet = decode_meaning(&json)?;
            println!("{packet:#?}");
        }
    }

    Ok(())
}
