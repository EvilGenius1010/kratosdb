mod constants;
mod errors;
mod parser;
mod storage;
use clap::Parser;
use parser::*;

fn main() {
    let cli = CLICommands::parse();
    println!("{:?}", cli.command);
    // match &cli.command{
    //     // get =>
    // }
}
