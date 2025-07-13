use std::{
    fs::{self, File},
    io::{BufReader, Error, Read},
};

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author="hk", version="1.001", about, long_about = None)]
pub struct CLICommands {
    #[command(subcommand)]
    pub command: Commands,
}

// apply correct linting rule to disable warning.
//#[warn(non_camel_case_types)]

#[derive(Subcommand, Debug)]
pub enum Commands {
    get { value: String },
    set { key: String, value: String },
    del { key: String },
}

//key can be dropped after read.
pub fn get_item(key: String) -> Result<(), Error> {
    // read .dbdata/primindex which is gonna be an index

    //propagate
    let read_index = File::open(".dbdata/.primindex")?;
    //why mut?
    let mut index_buf_reader = BufReader::new(read_index);

    // space separated numbers containing the indexes from where the indexes start.
    let mut keystartindices: Vec<u32> = Vec::new();

    for tokens in index_buf_reader.bytes() {
        println!("{:?}", tokens);
    }
    Ok(())
}
