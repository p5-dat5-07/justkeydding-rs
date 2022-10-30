use serde::{Serialize, Deserialize};
use std::collections::VecDeque;
use std::path::{PathBuf, Path};
use midly::{Smf, TrackEventKind, MidiMessage};
use std::fs;
use glob::glob;
use indicatif::{ProgressBar, ProgressStyle};

use crate::analyze::*;
use crate::args::*;
use crate::consts::*;
use crate::transitions::get_transition;
use crate::profiles::{get_profile_major, get_profile_minor};

#[derive(Serialize, Deserialize, Debug)]
struct JsonKey {
    key: u8,
    name: String,
    file_name: String,
}

pub fn process(args: &Args) {
    let transitions: Vec<f32> = Vec::from(get_transition(args.transition_profile));
    let minor_profile: VecDeque<f32> = VecDeque::from(get_profile_minor(args.minor_profile, args));
    let major_profile: VecDeque<f32> = VecDeque::from(get_profile_major(args.major_profile, args));

    let mut analyzer: Analyzer = Analyzer::init(&transitions, &major_profile, &minor_profile);

    let mut result: Vec<JsonKey> = Vec::new();
    let files = get_files(args);
    let pb = ProgressBar::new(files.len().try_into().unwrap());
    let sty = ProgressStyle::with_template(
        "[{elapsed_precise}] [{wide_bar}] {pos:>7}/{len:7}",
    ).unwrap().progress_chars("=>-");
    pb.set_style(sty);
    for file in files {
        let notes = get_normalized_notes(&file); 
        let key = analyzer.analyze(&notes);
        let file_name: String = file.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default().to_string();
        result.push(JsonKey { 
            key: key,
            name: KEY_USIZE_TO_STR[key as usize].to_string(),
            file_name: file_name
        });
        pb.inc(1);
    }
    let out = Path::new(&args.output_file);
    write_to_out_file(out, result);
    println!("Done see {} for the output.",  &out.to_string_lossy());
}

fn get_files(args: &Args) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();
    if args.recursive {
        let mut glob_str = args.input_path.clone();
        glob_str.push_str("**/*.mid*");
        for entry in glob(&glob_str).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) =>  files.push(path),
                Err(e) => println!("{:?}", e),
            }
        }
    } else {
        let file = Path::new(&args.input_path).to_path_buf();
        if file.is_dir() {
            println!("The input path is a directory not a file. Add -r to use folder as input");
            std::process::exit(1);
        } 
        files.push(file);
    }
    files
}

fn write_to_out_file(file: &Path, res: Vec<JsonKey>) {
    let serialized = serde_json::to_string(&res).unwrap();
    
    match fs::write(file, serialized){
        Ok(_) => (),
        Err(error) => panic!("Problem writing to the file: {:?}", error),
    };
}

fn get_normalized_notes(file: &Path) -> Vec<u8>{
    let bytes = match fs::read(file) {
        Ok(bytes) => bytes,
        Err(e) => {
            println!("{:?}", e);
            println!("Failed to open file: {:?}", file);
            std::process::exit(2);
        }
    };
    let mut smf = match Smf::parse(&bytes) {
        Ok(smf) => smf,
        Err(_) => {
            println!("Failed to parse file: {:?}", file);
            std::process::exit(3);
        }
    };
    let track = smf.tracks.remove(1);
    let notes = track.iter().filter_map(|event| {
        match event.kind {
            TrackEventKind::Midi { message, ..} => {
                match message {
                    MidiMessage::NoteOn { key, .. } => {
                        Some(u8::from(key) % 12)
                    },
                    _ => None
                }
            },
            _ => None
        }
    }).collect::<Vec<u8>>();
    notes
}