use std::collections::VecDeque;
use crate::consts::*;
#[derive(Clone, Debug)]
struct Key {
    key: usize,
    prob: f32,
    prev: usize,
}

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
        let sp = Vec::from(START_STATE);
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
        let stp = create_transition_probabilities(&sp);
        Self {
            tp: tp,
            ep: ep,
            sp: sp,
            stp: stp,
        }
    }

    fn viterbi(& self, notes: &Vec<u8>, tp: &Vec<Vec<f32>>, ep: &Vec<Vec<f32>>) -> (Vec<u8>, f32) {
        let mut v = vec![vec![Key{ key: 0, prob: 0.0, prev: 0}; 24]; notes.len()];
        for i in 0..24 {
            let prob: f32 = self.sp[i].ln() + ep[i][notes[0] as usize];
            v[0][i].key = i;
            v[0][i].prob = prob;
        }
        for t in 1..notes.len() {
            for st in 0..24 {
                let mut max_tr_prob: f32 = 0.0;
                let mut prev_st_store: usize = 0;
                // Gets max value
                for prev_st in 0..24 {
                    let b = tp[prev_st][st];
                    let a = v[t-1][prev_st].prob + b;
                    if a > max_tr_prob  {
                        max_tr_prob = a;
                        prev_st_store = prev_st;
                    } else if prev_st == 0 {
                        max_tr_prob = a;
                    }
                }
                let b = ep[st][notes[t] as usize];
                v[t][st].key = st;
                v[t][st].prob = max_tr_prob + b;
                v[t][st].prev = prev_st_store;
            }
        }
    
    
        let mut opt: Vec<u8> = vec![0;v.len()];
    
        let mut max_prob: f32 = 0.0;
        let mut previous = 0;
        
        for x in 0..v.len() {
            for y in 0..24 {
                let a = v[x][y].prob;
                if a > max_prob {
                    max_prob = a;
                    previous = y;
                }
            }
        }
        
    
        for t in (0..v.len()-1).rev() {
            opt[t] = v[t + 1][previous].prev as u8;
            previous = v[t + 1][previous].prev;
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
    let i = (index + rotation) % size;
    vec[i.abs() as usize]
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
                res[idx][idy] = shifted_lookup(&relative, relative_rotation, idy as i32, 12);//.ln();
            } else {
                res[idx][idy] = shifted_lookup(&tonic, tonic_rotation, idy as i32, 12);//.ln();
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