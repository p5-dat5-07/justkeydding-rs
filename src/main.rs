mod process;
mod consts;
mod profiles;
mod transitions;
mod args;
mod analyze;
mod tests;

use crate::args::Args;
use clap::{Parser};

fn main() {
    let args = Args::parse();
    process::process(&args);
}