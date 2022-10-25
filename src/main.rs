use std::path::PathBuf;
use std::collections::VecDeque;
use midly::{Smf, TrackEventKind, MidiMessage};
use std::fs;
use glob::glob;
use std::path::Path;
use serde::{Serialize, Deserialize};
use clap::{Parser};
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Input path.
   #[arg(short, long)]
   input_path: String,

   /// Output file path.
   #[arg(short, long)]
   output_file: String,

   /// Recursively travels through the input path finding midi files.
   #[arg(short, long, default_value_t = false)]
   recursive: bool,
}

#[derive(Clone, Debug)]
struct Key {
    key: usize,
    prob: f32,
    prev: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonKey {
    key: u8,
    name: String,
    file_path: String,
}

const STATES: &'static [&str] = &[
    "C",
    "Db",
    "D",
    "Eb",
    "E",
    "F",
    "F#",
    "G",
    "Ab",
    "A",
    "Bb",
    "B",
    "c",
    "c#",
    "d",
    "eb",
    "e",
    "f",
    "f#",
    "g",
    "ab",
    "a",
    "bb",
    "b"
];

fn shifted_lookup(vec: &Vec<f32>, rotation: i32, index: i32, size: i32) -> f32 {
    let i = (index + rotation) % size;
    vec[i.abs() as usize]
}

fn create_transition_probabilities(key_transitions: &Vec<f32>) -> Vec<Vec<f32>> { 
    let mut res = vec![vec![0.0; 24]; 24];
    let mut tonic: Vec<f32> = key_transitions.to_vec();
    let relative: Vec<f32> = tonic.split_off(12);
    if key_transitions.len() != 24 {
        println!("Invalid size");
    }
    for idx in 0..24 {
        let tonic_rotation: i32 = -(idx as i32 % 12);
        let mut relative_rotation: i32 = -(idx as i32 % 12);
        if idx >= 12 {
            relative_rotation = (relative_rotation - 3) % 12;
        }
        
        for idy in 0..24 {
            if idy >= 12 {
                res[idx][idy] = shifted_lookup(&relative, relative_rotation, idy as i32, 12);
            } else {
                res[idx][idy] = shifted_lookup(&tonic, tonic_rotation, idy as i32, 12);
            }
        }
    }
    res
}

fn create_emission_probabilities(major: VecDeque<f32>, minor: VecDeque<f32>) -> Vec<Vec<f32>> {
    let mut res = vec![vec![0.0; 12]; 24];
    let mut profile = major.clone();
    for idx in 0..24 {
        let roatation = -(idx as i32 % 12);
        if idx >= 12 {
            profile = minor.clone();
        }
         
        if roatation > 0 {
            profile.rotate_left(roatation as usize);
        } else {
            profile.rotate_right((-roatation) as usize);
        }
        
        for idy in 0..12 {
            res[idx][idy] = profile[idy].clone() as f32;
        }
        if roatation > 0 {
            profile.rotate_right(roatation as usize);
        } else {
            profile.rotate_left((-roatation) as usize);
        }
    }
    res
}

fn viterbi(notes: &Vec<u8>, start_p: &Vec<f32>, trans_p: &Vec<Vec<f32>>, emit_p: &Vec<Vec<f32>>) -> (Vec<u8>, f32) {
    let mut v = vec![vec![Key{ key: 0, prob: 0.0, prev: 0}; 24]; notes.len()];
    for i in 0..24 {
        let prob: f32 =start_p[i].ln()+emit_p[i][notes[0] as usize].ln();
        v[0][i].key = i;
        v[0][i].prob = prob;
    }

    for t in 1..notes.len() {
        for st in 0..24 {
            let mut max_tr_prob: f32 = 0.0;
            for prev_st in 0..24 {

                let a = v[t-1][prev_st].prob + trans_p[prev_st][st].ln();
                if a > max_tr_prob  {
                    max_tr_prob = a;
                } else if prev_st == 0 {
                    max_tr_prob = a;
                }
            }
            for prev_st in 0..24 {
                let a = v[t-1][prev_st].prob;
                let b = trans_p[prev_st][st].ln();
                if a + b == max_tr_prob {
                    let max_prob = max_tr_prob + emit_p[st][notes[t] as usize].ln();
                    v[t][st].key = st;
                    v[t][st].prob = max_prob;
                    v[t][st].prev = prev_st;
                    break;
                }
            }
        }
    }


    let mut opt: Vec<u8> = Vec::new();

    let mut max_prob: f32 = 0.0;
    //println!("{}", max_prob);
    let mut previous = 0;
    for x in 0..v.len() {
        for y in 0..24 {
            let a = v[x][y].prob;
            if a > max_prob {
                max_prob = a;
            }
        }
    }

    for x in 0..v.len() {
        for y in 0..24 {
            let a = v[x][y].prob;
            if a == max_prob {
                opt.push(y as u8);
                previous = y;
                break;
            }
        }
    }

    for t in (0..v.len()-1).rev() {
        opt.insert(0, v[t + 1][previous].prev as u8);
        previous = v[t + 1][previous].prev;
    }


    return (opt, max_prob)
}

fn analyze_track(notes: &Vec<u8>, start_probabilities: &Vec<f32>,  start_transition_probabilities: &Vec<Vec<f32>>, transition_probabilities: &Vec<Vec<f32>>, emission_probabilities: &Vec<Vec<f32>>) -> u8 {
    let (local_keys, _) = viterbi(notes,
        start_probabilities, 
        transition_probabilities,
        emission_probabilities);
    //let trans_p = create_transition_probabilities(Vec::from(vec![0.04166666666; 24]));
    let (key, _) = viterbi(&local_keys,  start_probabilities, &start_transition_probabilities, &transition_probabilities);
    key[0]
}

fn analyze(args: &Args) {
    // key_transitions_exponential_10
    let key_transitions: Vec<f32> = Vec::from([0.6923183492172971, 6.923183492172971e-06, 0.000692318349217297, 0.000692318349217297, 6.923183492172971e-05, 0.06923183492172971, 6.9231834921729705e-09, 0.06923183492172971, 6.923183492172971e-05, 0.000692318349217297, 0.000692318349217297, 6.923183492172971e-06, 0.06923183492172971, 6.923183492172971e-08, 0.006923183492172971, 6.923183492172971e-07, 0.006923183492172971, 0.006923183492172971, 6.923183492172971e-07, 0.006923183492172971, 6.923183492172971e-08, 0.06923183492172971, 6.923183492172971e-05, 6.923183492172971e-05]);
   
    // sapp_minor
    let key_profile_minor: VecDeque<f32> = VecDeque::from([
        0.2222222222222222, 0.0, 0.1111111111111111, 0.1111111111111111,
        0.0, 0.1111111111111111, 0.0, 0.2222222222222222,
        0.1111111111111111, 0.0, 0.05555555555555555, 0.05555555555555555
    ]);
    // sapp_major
    let key_profile_major: VecDeque<f32> = VecDeque::from([
        0.2222222222222222, 0.0, 0.1111111111111111, 0.0,
        0.1111111111111111, 0.1111111111111111, 0.0, 0.2222222222222222,
        0.0, 0.1111111111111111, 0.0, 0.1111111111111111
    ]);

    let transition_probabilities = create_transition_probabilities(&key_transitions);
    let emission_probabilities = create_emission_probabilities(key_profile_major, key_profile_minor); 
    let start_probabilities = &vec![0.04166666666; 24];
    let start_transition_probabilities = create_transition_probabilities(&start_probabilities);

    let mut result: Vec<JsonKey> = Vec::new();
    let files = get_files(args);
    let pb = ProgressBar::new(files.len().try_into().unwrap());
    let sty = ProgressStyle::with_template(
        "[{elapsed_precise}] [{wide_bar}] {pos:>7}/{len:7}",
    ).unwrap().progress_chars("=>-");
    pb.set_style(sty);
    for file in files {
        let notes = get_normalized_notes(&file); 
        let key = analyze_track(
            &notes,
            &start_probabilities,
            &start_transition_probabilities,
            &transition_probabilities,
            &emission_probabilities);
        result.push(JsonKey { 
            key: key,
            name: STATES[key as usize].to_string(),
            file_path: file.to_string_lossy().to_string()
        });
        pb.inc(1);
    }
    let out = Path::new(&args.output_file);
    write_to_out_file(out, result);
    println!("Done see {} for output",  &out.to_string_lossy());
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
        files.push(Path::new(&args.input_path).to_path_buf());
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
    let bytes = fs::read(file).unwrap();
    let mut smf = Smf::parse(&bytes).unwrap();
    let track = smf.tracks.remove(1);
    let mut notes: Vec<u8> =  Vec::new();
    for event in track {
        match event.kind {
            TrackEventKind::Midi { message, ..} => {
                match message {
                    MidiMessage::NoteOn { key, .. } => {
                        let k = u8::from(key) % 12;
                        notes.push(k);
                    },
                    _ => (),
                }
            },
            _ => (),
        }
    }
    notes
}

fn main() {
    let args = Args::parse();
    analyze(&args);
}