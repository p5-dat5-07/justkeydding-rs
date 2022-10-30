use std::collections::VecDeque;
use crate::consts::*;

pub struct Analyzer {
    ep: Vec<Vec<f32>>,
    tp: Vec<Vec<f32>>,
    stp: Vec<Vec<f32>>,
    sp: Vec<f32>,
}

impl Analyzer {
    pub fn init(transitions: &Vec<f32>, major_profile: &VecDeque<f32>, minor_profile: &VecDeque<f32>) -> Self {
        let mut tp = create_transition_probabilities(&transitions);
        let mut ep = create_emission_probabilities(major_profile, minor_profile); 
        let mut sp = Vec::from(START_STATE);
        // Calculating the natural log is expensive; therefore its calculated one when the Analyzer is initialized.
        for x in 0..ep.len() {
            for y in 0..ep[x].len() {
                ep[x][y] = ep[x][y].ln();
            }
        }
        for x in 0..tp.len() {
            for y in 0..tp[x].len() {
                tp[x][y] = tp[x][y].ln();
            }
        }
        for x in 0..sp.len() {
            sp[x] = sp[x].ln();
        }

        let stp = create_transition_probabilities(&sp);
        Self {
            tp: tp,
            ep: ep,
            sp: sp,
            stp: stp,
        }
    }

    fn viterbi(& self, notes: &Vec<u8>, tp: &Vec<Vec<f32>>, ep: &Vec<Vec<f32>>) -> (Vec<u8>, f32) {
        let mut v = vec![vec![0; 24]; notes.len()];
        let prev_keys = &mut vec![0f32; 24];
        let keys = &mut vec![0f32; 24];

        // Get initial key probabilities.
        for i in 0..24 {
            prev_keys[i] = self.sp[i] + ep[i][notes[0] as usize];
        }

        // Get the list of most likely keys.
        let mut max_prob = 0f32;
        let mut previous = 0usize;
        for t in 1..notes.len() {
            for key in 0..24 {
                let mut max_tb: f32 = 0.0;
                let mut max_prev_key: usize = 0;
                // moving out the first iteration of the loop
                // removes a else `if prev_key == 0` check saving 20 seconds.
                let b = tp[0][key];
                let a = prev_keys[0] + b;
                if a > max_tb  {
                    max_tb = a;
                    max_prev_key = 0;
                } else {
                    max_tb = a;
                }

                // Getting most likely key from previous step
                for prev_key in 1..24 {
                    // a: Chance of transitioning from the previous key to the current key.
                    // b: Probability of previous key + chance to transition to current key.
                    let a = tp[prev_key][key];
                    let b = prev_keys[prev_key] + a;
                    // Get the max transition probability and corresponding key.
                    if b > max_tb  {
                        max_tb = b;
                        max_prev_key = prev_key;
                    }
                }

                // Get the current key probability.
                // a: Chance of the current key emitting the current note.
                let a = ep[key][notes[t] as usize]; 
                keys[key] = max_tb + a;
                v[t][key] = max_prev_key; 

                // Get the most likely key at the end of sequence of notes.
                if keys[key] > max_prob {
                    max_prob = keys[key];
                    previous = max_prev_key;
                }
            }
            std::mem::swap(prev_keys, keys);
        }

        // Go back through all the previous keys starting at the most likely.
        // This generates the list of local keys.
        let mut opt: Vec<u8> = vec![0;v.len()];
        for t in (0..v.len()-1).rev() {
            opt[t] = v[t + 1][previous] as u8;
            previous = opt[t] as usize;
        }
        opt.push(previous as u8);
    
        return (opt, max_prob)
    }

    pub fn analyze(&mut self, notes: &Vec<u8>) -> u8 {
        let (local_keys, _) = self.viterbi(notes, &self.tp, &self.ep);
        let (key, _) = self.viterbi(&local_keys, &self.stp, &self.tp);
        key[0]
    }
}

fn shifted_lookup(vec: &Vec<f32>, rotation: i32, index: i32, size: i32) -> f32 {
    vec[((index + rotation).abs() % size) as usize]
}

fn create_transition_probabilities(key_transitions: &Vec<f32>) -> Vec<Vec<f32>> { 
    let mut res = vec![vec![0.0; 24]; 24];
    let mut tonic: Vec<f32> = key_transitions.clone();
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

fn create_emission_probabilities(major: &VecDeque<f32>, minor: &VecDeque<f32>) -> Vec<Vec<f32>> {
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
            res[idx][idy] = profile[idy];//.ln();
        }
        if roatation > 0 {
            profile.rotate_right(roatation as usize);
        } else {
            profile.rotate_left((-roatation) as usize);
        }
    }
    res
}