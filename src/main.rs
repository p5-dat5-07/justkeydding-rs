mod process;
mod analyze;
mod consts;
mod profiles;
mod transitions;
mod args;

use crate::args::Args;
use clap::{Parser};

fn main() {
    let args = Args::parse();
    process::process(&args);
}