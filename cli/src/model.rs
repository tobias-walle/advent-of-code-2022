use std::path::PathBuf;

use clap::{command, Parser, Subcommand};
use serde::Deserialize;

#[derive(Debug, Clone, Parser)]
#[command()]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
    #[arg(short, long, default_value = "./aoc.toml")]
    pub config: PathBuf,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Command {
    Download {
        #[arg(short, long)]
        example: bool,
        #[arg(short, long)]
        year: Option<u32>,
        #[arg(short, long)]
        day: Option<u32>,
    },
    Submit {
        result: String,
        #[arg(short, long)]
        year: Option<u32>,
        #[arg(short, long)]
        day: Option<u32>,
        #[arg(short, long)]
        level: Option<u32>,
    },
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub year: u32,
    pub day: u32,
}
