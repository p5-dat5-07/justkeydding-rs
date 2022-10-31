#[cfg(test)]
mod tests {
    use std::fs;
    use serde::{Serialize, Deserialize};
    use std::path::Path;

    use crate::process::*;
    use crate::profiles::*;
    use crate::transitions::*;
    use crate::analyze::*;
    use crate::args::Args;

    #[derive(Serialize, Deserialize, Debug)]
    struct EmissionJson {
        name: String,
        value: [[f64; 12];24]
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct TransitionJson {
        name: String,
        value: [[f64; 24];24]
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct ObservationJson {
        name: String,
        observations: Vec<u8>
    }
    const EPS: f64 = 1.0e-6;

    #[test]
    fn test_emissions() {
        let file = fs::read("./tests/emissions.json").unwrap();
        let test_data: Vec<EmissionJson> = serde_json::from_slice(&file).unwrap();
        for (i, expected) in test_data.iter().enumerate() {
            let got = create_emission_probabilities(&Vec::from(MAJOR[i]), &Vec::from(MINOR[i]));
            for j in 0..got.len() {
                for k in 0..got[j].len() {
                    assert!((got[j][k] - expected.value[j][k]).abs() < EPS, "assertion failed: `(left !== right)` \
                (left: `{:?}`, right: `{:?}`) Faild on: {:?} at row: {}", got[j],  expected.value[j],expected.name, j)
                }
            }
        }
    }

    #[test]
    fn test_transitions() {
        let file = fs::read("./tests/transitions.json").unwrap();
        let test_data: Vec<TransitionJson> = serde_json::from_slice(&file).unwrap();
        for (i, expected) in test_data.iter().enumerate() {
            let got = create_transition_probabilities(&Vec::from(TRANSITIONS[i]));
            for j in 0..got.len() {
                for k in 0..got[j].len() {
                    assert!((got[j][k] - expected.value[j][k]).abs() < EPS, "assertion failed: `(left !== right)` \
                (left: `{:?}`, right: `{:?}`) Faild on: {:?} at row: {}", got[j],  expected.value[j],expected.name, j)
                }
            }
            
        }
    }

    // Issue we get double the amount of notes
    #[test]
    fn test_observations() {
        let file = fs::read("./tests/obs.json").unwrap();
        let test_data: Vec<ObservationJson> = serde_json::from_slice(&file).unwrap();
        for expected in test_data.iter() {
            let got = get_normalized_notes(Path::new(&expected.name));
            assert_eq!(got.len(), expected.observations.len(), "Failed on {:?} Expected length {} got {}", expected.name, expected.observations.len(), got.len());
            assert_eq!(got, expected.observations, "Failed on {:?}", expected.name);
        }
    }

    #[test]
    fn test_maestro(){
        let file = fs::read("./tests/maestro.json").unwrap();
        let test_data: Vec<JsonKey> = serde_json::from_slice(&file).unwrap();
        let mut args = Args {
            input_path: "./q-maestro-v2.0.0/".to_string(),
            major_profile: ProfileMajor::Sapp,
            minor_profile: ProfileMinor::Sapp,
            transition_profile: Transition::KeyTransitionsExponential,
            major_profile_normalized: true,
            minor_profile_normalized: true,
            profile_normalized: true,
            recursive: true,
            output_file: "".to_string(),
        };
        let files = get_files(&args);
        args.recursive = false;
        for (i, expected) in test_data.iter().enumerate() {
            args.input_path = files[i].to_str().unwrap().to_string();
            let got = process(&args);
            assert_eq!(got[0].key, expected.key, "Faild on {:?} expected '{}' got '{}", expected.file_name, expected.name, got[0].name);
        }
    }
}