use crate::consts::*;


#[derive(Debug, Clone)]
pub struct Key {
    prob: f64,
    prev: usize,
}
pub struct Analyzer {
    ep: Vec<Vec<f64>>,
    tp: Vec<Vec<f64>>,
    ntp: Vec<Vec<f64>>,
    sp: Vec<f64>,
}

impl Analyzer {
    pub fn init(transitions: &Vec<f64>, null_transitions: &Vec<f64>, major_profile: &Vec<f64>, minor_profile: &Vec<f64>) -> Self {
        let tp = create_transition_probabilities(transitions);
        let ep = create_emission_probabilities(major_profile, minor_profile); 
        let sp = Vec::from(START_STATE);
        let ntp = create_transition_probabilities(&null_transitions);
        // Calculating the natural log is expensive; therefore its calculated one when the Analyzer is initialized.
        /*for x in 0..ep.len() {
            for y in 0..ep[x].len() {
                ep[x][y] = ep[x][y].ln();
            }
        }
        for x in 0..tp.len() {
            for y in 0..tp[x].len() {
                tp[x][y] = tp[x][y].ln();
            }
        }
        for x in 0..ntp.len() {
            for y in 0..ntp[x].len() {
                ntp[x][y] = ntp[x][y].ln();
            }
        }

        for x in 0..sp.len() {
            sp[x] = sp[x].ln();
        }*/

        Self {
            tp: tp,
            ep: ep,
            sp: sp,
            ntp: ntp,
        }
    }

    fn viterbi(&self, notes: &Vec<u8>, sp: &Vec<f64>, tp: &Vec<Vec<f64>>, ep: &Vec<Vec<f64>>) -> (Vec<u8>, f64) {
        let mut v = vec![vec![Key{ prob: 0.0, prev: 0}; 24]; notes.len()];
        for i in 0..24 {
            let prob: f64 = sp[i]+ep[i][notes[0] as usize];
            v[0][i].prob = prob;
        }
    
        for t in 1..notes.len() {
            for st in 0..24 {
                let mut max_tr_prob: f64 = 0.0;
                for prev_st in 0..24 {
    
                    let a = v[t-1][prev_st].prob + tp[prev_st][st];
                    if a > max_tr_prob  {
                        max_tr_prob = a;
                    } else if prev_st == 0 {
                        max_tr_prob = a;
                    }
                }
                for prev_st in 0..24 {
                    let a = v[t-1][prev_st].prob;
                    let b = tp[prev_st][st];
                    if a + b == max_tr_prob {
                        let max_prob = max_tr_prob + ep[st][notes[t] as usize];
                        v[t][st].prob = max_prob;
                        v[t][st].prev = prev_st;
                        break;
                    }
                }
            }
        }
    
        let mut opt: Vec<u8> = Vec::new();
    
        let mut max_prob: f64 = 0.0;

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
    
        for t in (0..v.len()-2).rev() {
            opt.insert(0, v[t + 1][previous].prev as u8);
            previous = v[t + 1][previous].prev as usize;
        }
    
    
        return (opt, max_prob)
        /*let mut v = vec![vec![Key { prob: 0f64, prev: 0}; 24]; notes.len()];
        let prev_keys = &mut vec![0f64; 24];
        let keys = &mut vec![0f64; 24];

        // Get initial key probabilities.
        for i in 0..24 {
            prev_keys[i] = self.sp[i] + ep[i][notes[0] as usize];
            v[0][i].prob = prev_keys[i]; 
        }

        // Get the list of most likely keys.
        let mut max_prob = 0f64;
        let mut previous = 0usize;
        for t in 1..notes.len() {
            for key in 0..24 {
                let mut max_tp: f64 = 0.0;
                let mut max_prev_key: u8 = 0;
                // moving out the first iteration of the loop
                // removes a else `if prev_key == 0` check saving 20 seconds.
                /*let b = tp[0][key];
                let a = prev_keys[0] + b;
                if a > max_tb  {
                    max_tb = a;
                    max_prev_key = 0;
                } else {
                    max_tb = a;
                }*/

                // Getting most likely key from previous step
                for prev_key in 0..24 {
                    // a: Chance of transitioning from the previous key to the current key.
                    // b: Probability of previous key + chance to transition to current key.
                    let a = v[t][key].prob;
                    let b = tp[prev_key][key];

                    let _tp = a+b;
                    // Get the max transition probability and corresponding key.
                    if _tp > max_tp  {
                        max_tp = b;
                        max_prev_key = prev_key as u8;
                    } else if prev_key == 0 {
                        max_tp = a;
                    }
                }

                // Get the current key probability.
                // a: Chance of the current key emitting the current note.
                let a = ep[key][notes[t] as usize]; 
                keys[key] = max_tp + a;
                v[t][key].prob = keys[key]; 
                v[t][key].prev = max_prev_key; 

                // Get the most likely key at the end of sequence of notes.
                if keys[key] > max_prob {
                    max_prob = keys[key];
                    previous = max_prev_key as usize;
                }
            }
            std::mem::swap(prev_keys, keys);
        }

        let mut opt: Vec<u8> = Vec::new();
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

        // Go back through all the previous keys starting at the most likely.
        // This generates the list of local keys.
    
        for t in (0..v.len()-1).rev() {
            opt.insert(0, v[t + 1][previous].prev as u8);
            previous = v[t + 1][previous].prev as usize;
        }
        
       // opt.push(previous as u8);
    
        return (opt, max_prob)*/
    }

    pub fn analyze(&mut self, notes: &Vec<u8>) -> u8 {
        let (local_keys, _) = self.viterbi(notes, &self.sp, &self.tp, &self.ep);
        let (key, _) = self.viterbi(&local_keys, &self.sp, &self.ntp, &self.tp);
        key[0]
    }
}

pub fn create_transition_probabilities(key_transitions: &Vec<f64>) -> Vec<Vec<f64>> { 
    let mut res = vec![vec![0.0; 24]; 24];
    
    for idx in 0..key_transitions.len() {
        let state_cpy = key_transitions.clone();
        let mut tonic = (&state_cpy[..12]).to_vec();
        let mut relative = (&state_cpy[12..]).to_vec();
        let tr = idx as usize % 12;
        let rr: usize = idx as usize % 12;
    
        tonic.rotate_right(tr);
        relative.rotate_right(rr);
          
        tonic.append(&mut relative);
        let kt_ = tonic;

        for idy in 0..key_transitions.len() {
            res[idx][idy] = kt_[idy];
        }
    }
    res
}

pub fn create_emission_probabilities(major: &Vec<f64>, minor: &Vec<f64>) -> Vec<Vec<f64>> {
    let mut res = vec![vec![0.0; 12]; 24];
    for idx in 0..24 {
        let mut profile = if idx < 12 {major.clone()} else {minor.clone()};
        let rotation: usize = idx as usize % 12;
       
        profile.rotate_right(rotation);
        for idy in 0..12 {
            res[idx][idy] = profile[idy];
        }
    }
    res
}