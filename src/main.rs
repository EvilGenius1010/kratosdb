mod constants;
mod errors;
// mod crate::super::levels;
mod parser;
mod storage;
use core::error::Error;

use clap::Parser;
use parser::*;

fn main() {
    let cli = CLICommands::parse();

    println!("{:?} running...", cli.command);

    get_item("man".to_string());

    // match cli.command {
    //     Commands::get { value } => get_item(value),
    //     Commands::set { key, value } => {
    //         println!("In progress!");
    //         Ok(Result<(),Error>);
    //     }
    //     Commands::del { key } => {
    //         println!("In progress!");
    //         Ok(Result<(),Error>);
    //     }
    // }
}
