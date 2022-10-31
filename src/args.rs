use clap::{Parser};
use crate::profiles::*;
use crate::transitions::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Input path.
    #[arg(short='f', long)]
    pub input_path: String,

    /// Output file path.
    #[arg(short, long)]
    pub output_file: String,

    /// Major profile
    #[arg(value_enum)]
    #[arg(short='a', long, default_value_t = ProfileMajor::Sapp)]
    pub major_profile: ProfileMajor,

    /// Minor profile
    #[arg(value_enum)]
    #[arg(short='i', long, default_value_t = ProfileMinor::Sapp)]
    pub minor_profile: ProfileMinor,

    /// Transition profile
    #[arg(value_enum)]
    #[arg(short, long, default_value_t = Transition::KeyTransitionsExponential10)]
    pub transition_profile: Transition,

    /// Major profile normalized
    #[arg(long, default_value_t = true)]
    pub major_profile_normalized: bool,

    /// Major profile normalized
    #[arg(long, default_value_t = true)]
    pub minor_profile_normalized: bool,

    // Profile normalized
    #[arg(short='n', long, default_value_t = true)]
    pub profile_normalized: bool,

    /// Recursively travels through the input path finding midi files.
    #[arg(short, long, default_value_t = false)]
    pub recursive: bool,
}